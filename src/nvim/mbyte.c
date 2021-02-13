// This is an open source non-commercial project. Dear PVS-Studio, please check
// it. PVS-Studio Static Code Analyzer for C, C++ and C#: http://www.viva64.com

/// mbyte.c: Code specifically for handling multi-byte characters.
/// Multibyte extensions partly by Sung-Hoon Baek
///
/// Strings internal to Nvim are always encoded as UTF-8 (thus the legacy
/// 'encoding' option is always "utf-8").
///
/// The cell width on the display needs to be determined from the character
/// value. Recognizing UTF-8 bytes is easy: 0xxx.xxxx is a single-byte char,
/// 10xx.xxxx is a trailing byte, 11xx.xxxx is a leading byte of a multi-byte
/// character. To make things complicated, up to six composing characters
/// are allowed. These are drawn on top of the first char. For most editing
/// the sequence of bytes with composing characters included is considered to
/// be one character.
///
/// UTF-8 is used everywhere in the core. This is in registers, text
/// manipulation, buffers, etc. Nvim core communicates with external plugins
/// and GUIs in this encoding.
///
/// The encoding of a file is specified with 'fileencoding'.  Conversion
/// is to be done when it's different from "utf-8".
///
/// Vim scripts may contain an ":scriptencoding" command. This has an effect
/// for some commands, like ":menutrans".

#include <inttypes.h>
#include <stdbool.h>
#include <string.h>
#include <wchar.h>
#include <wctype.h>

#include "nvim/vim.h"
#include "nvim/ascii.h"
#ifdef HAVE_LOCALE_H
# include <locale.h>
#endif
#include "nvim/eval.h"
#include "nvim/path.h"
#include "nvim/iconv.h"
#include "nvim/mbyte.h"
#include "nvim/charset.h"
#include "nvim/cursor.h"
#include "nvim/fileio.h"
#include "nvim/func_attr.h"
#include "nvim/memline.h"
#include "nvim/message.h"
#include "nvim/misc1.h"
#include "nvim/memory.h"
#include "nvim/option.h"
#include "nvim/screen.h"
#include "nvim/spell.h"
#include "nvim/strings.h"
#include "nvim/os/os.h"
#include "nvim/arabic.h"
#include "nvim/mark.h"

typedef struct {
  int rangeStart;
  int rangeEnd;
  int step;
  int offset;
} convertStruct;

struct interval {
  long first;
  long last;
};

#ifdef INCLUDE_GENERATED_DECLARATIONS
# include "mbyte.c.generated.h"
# include "unicode_tables.generated.h"
#endif

char_u e_loadlib[] = "E370: Could not load library %s";
char_u e_loadfunc[] = "E448: Could not load library function %s";

// To speed up BYTELEN(); keep a lookup table to quickly get the length in
// bytes of a UTF-8 character from the first byte of a UTF-8 string.  Bytes
// which are illegal when used as the first byte have a 1.  The NUL byte has
// length 1.
const uint8_t utf8len_tab[] = {
  // ?1 ?2 ?3 ?4 ?5 ?6 ?7 ?8 ?9 ?A ?B ?C ?D ?E ?F
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 0?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 1?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 2?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 3?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 4?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 5?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 6?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 7?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 8?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 9?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // A?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // B?
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,  // C?
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,  // D?
  3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,  // E?
  4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 1, 1,  // F?
};

// Like utf8len_tab above, but using a zero for illegal lead bytes.
const uint8_t utf8len_tab_zero[] = {
  // ?1 ?2 ?3 ?4 ?5 ?6 ?7 ?8 ?9 ?A ?B ?C ?D ?E ?F
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 0?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 1?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 2?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 3?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 4?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 5?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 6?
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 7?
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 8?
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 9?
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // A?
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // B?
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,  // C?
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,  // D?
  3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,  // E?
  4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 0, 0,  // F?
};

/*
 * Canonical encoding names and their properties.
 * "iso-8859-n" is handled by enc_canonize() directly.
 */
static struct
{   const char *name;   int prop;              int codepage; }
enc_canon_table[] =
{
#define IDX_LATIN_1     0
  {"latin1",          ENC_8BIT + ENC_LATIN1,  1252},
#define IDX_ISO_2       1
  {"iso-8859-2",      ENC_8BIT,               0},
#define IDX_ISO_3       2
  {"iso-8859-3",      ENC_8BIT,               0},
#define IDX_ISO_4       3
  {"iso-8859-4",      ENC_8BIT,               0},
#define IDX_ISO_5       4
  {"iso-8859-5",      ENC_8BIT,               0},
#define IDX_ISO_6       5
  {"iso-8859-6",      ENC_8BIT,               0},
#define IDX_ISO_7       6
  {"iso-8859-7",      ENC_8BIT,               0},
#define IDX_ISO_8       7
  {"iso-8859-8",      ENC_8BIT,               0},
#define IDX_ISO_9       8
  {"iso-8859-9",      ENC_8BIT,               0},
#define IDX_ISO_10      9
  {"iso-8859-10",     ENC_8BIT,               0},
#define IDX_ISO_11      10
  {"iso-8859-11",     ENC_8BIT,               0},
#define IDX_ISO_13      11
  {"iso-8859-13",     ENC_8BIT,               0},
#define IDX_ISO_14      12
  {"iso-8859-14",     ENC_8BIT,               0},
#define IDX_ISO_15      13
  {"iso-8859-15",     ENC_8BIT + ENC_LATIN9,  0},
#define IDX_KOI8_R      14
  {"koi8-r",          ENC_8BIT,               0},
#define IDX_KOI8_U      15
  {"koi8-u",          ENC_8BIT,               0},
#define IDX_UTF8        16
  {"utf-8",           ENC_UNICODE,            0},
#define IDX_UCS2        17
  {"ucs-2",           ENC_UNICODE + ENC_ENDIAN_B + ENC_2BYTE, 0},
#define IDX_UCS2LE      18
  {"ucs-2le",         ENC_UNICODE + ENC_ENDIAN_L + ENC_2BYTE, 0},
#define IDX_UTF16       19
  {"utf-16",          ENC_UNICODE + ENC_ENDIAN_B + ENC_2WORD, 0},
#define IDX_UTF16LE     20
  {"utf-16le",        ENC_UNICODE + ENC_ENDIAN_L + ENC_2WORD, 0},
#define IDX_UCS4        21
  {"ucs-4",           ENC_UNICODE + ENC_ENDIAN_B + ENC_4BYTE, 0},
#define IDX_UCS4LE      22
  {"ucs-4le",         ENC_UNICODE + ENC_ENDIAN_L + ENC_4BYTE, 0},

  /* For debugging DBCS encoding on Unix. */
#define IDX_DEBUG       23
  {"debug",           ENC_DBCS,               DBCS_DEBUG},
#define IDX_EUC_JP      24
  {"euc-jp",          ENC_DBCS,               DBCS_JPNU},
#define IDX_SJIS        25
  {"sjis",            ENC_DBCS,               DBCS_JPN},
#define IDX_EUC_KR      26
  {"euc-kr",          ENC_DBCS,               DBCS_KORU},
#define IDX_EUC_CN      27
  {"euc-cn",          ENC_DBCS,               DBCS_CHSU},
#define IDX_EUC_TW      28
  {"euc-tw",          ENC_DBCS,               DBCS_CHTU},
#define IDX_BIG5        29
  {"big5",            ENC_DBCS,               DBCS_CHT},

  /* MS-DOS and MS-Windows codepages are included here, so that they can be
   * used on Unix too.  Most of them are similar to ISO-8859 encodings, but
   * not exactly the same. */
#define IDX_CP437       30
  {"cp437",           ENC_8BIT,               437},   /* like iso-8859-1 */
#define IDX_CP737       31
  {"cp737",           ENC_8BIT,               737},   /* like iso-8859-7 */
#define IDX_CP775       32
  {"cp775",           ENC_8BIT,               775},   /* Baltic */
#define IDX_CP850       33
  {"cp850",           ENC_8BIT,               850},   /* like iso-8859-4 */
#define IDX_CP852       34
  {"cp852",           ENC_8BIT,               852},   /* like iso-8859-1 */
#define IDX_CP855       35
  {"cp855",           ENC_8BIT,               855},   /* like iso-8859-2 */
#define IDX_CP857       36
  {"cp857",           ENC_8BIT,               857},   /* like iso-8859-5 */
#define IDX_CP860       37
  {"cp860",           ENC_8BIT,               860},   /* like iso-8859-9 */
#define IDX_CP861       38
  {"cp861",           ENC_8BIT,               861},   /* like iso-8859-1 */
#define IDX_CP862       39
  {"cp862",           ENC_8BIT,               862},   /* like iso-8859-1 */
#define IDX_CP863       40
  {"cp863",           ENC_8BIT,               863},   /* like iso-8859-8 */
#define IDX_CP865       41
  {"cp865",           ENC_8BIT,               865},   /* like iso-8859-1 */
#define IDX_CP866       42
  {"cp866",           ENC_8BIT,               866},   /* like iso-8859-5 */
#define IDX_CP869       43
  {"cp869",           ENC_8BIT,               869},   /* like iso-8859-7 */
#define IDX_CP874       44
  {"cp874",           ENC_8BIT,               874},   /* Thai */
#define IDX_CP932       45
  {"cp932",           ENC_DBCS,               DBCS_JPN},
#define IDX_CP936       46
  {"cp936",           ENC_DBCS,               DBCS_CHS},
#define IDX_CP949       47
  {"cp949",           ENC_DBCS,               DBCS_KOR},
#define IDX_CP950       48
  {"cp950",           ENC_DBCS,               DBCS_CHT},
#define IDX_CP1250      49
  {"cp1250",          ENC_8BIT,               1250},   /* Czech, Polish, etc. */
#define IDX_CP1251      50
  {"cp1251",          ENC_8BIT,               1251},   /* Cyrillic */
  /* cp1252 is considered to be equal to latin1 */
#define IDX_CP1253      51
  {"cp1253",          ENC_8BIT,               1253},   /* Greek */
#define IDX_CP1254      52
  {"cp1254",          ENC_8BIT,               1254},   /* Turkish */
#define IDX_CP1255      53
  {"cp1255",          ENC_8BIT,               1255},   /* Hebrew */
#define IDX_CP1256      54
  {"cp1256",          ENC_8BIT,               1256},   /* Arabic */
#define IDX_CP1257      55
  {"cp1257",          ENC_8BIT,               1257},   /* Baltic */
#define IDX_CP1258      56
  {"cp1258",          ENC_8BIT,               1258},   /* Vietnamese */

#define IDX_MACROMAN    57
  {"macroman",        ENC_8BIT + ENC_MACROMAN, 0},      /* Mac OS */
#define IDX_HPROMAN8    58
  {"hp-roman8",       ENC_8BIT,               0},       /* HP Roman8 */
#define IDX_COUNT       59
};

/*
 * Aliases for encoding names.
 */
static struct
{   const char *name; int canon; }
enc_alias_table[] =
{
  { "ansi",            IDX_LATIN_1 },
  { "iso-8859-1",      IDX_LATIN_1 },
  { "latin2",          IDX_ISO_2 },
  { "latin3",          IDX_ISO_3 },
  { "latin4",          IDX_ISO_4 },
  { "cyrillic",        IDX_ISO_5 },
  { "arabic",          IDX_ISO_6 },
  { "greek",           IDX_ISO_7 },
  { "hebrew",          IDX_ISO_8 },
  { "latin5",          IDX_ISO_9 },
  { "turkish",         IDX_ISO_9 },   // ?
  { "latin6",          IDX_ISO_10 },
  { "nordic",          IDX_ISO_10 },  // ?
  { "thai",            IDX_ISO_11 },  // ?
  { "latin7",          IDX_ISO_13 },
  { "latin8",          IDX_ISO_14 },
  { "latin9",          IDX_ISO_15 },
  { "utf8",            IDX_UTF8 },
  { "unicode",         IDX_UCS2 },
  { "ucs2",            IDX_UCS2 },
  { "ucs2be",          IDX_UCS2 },
  { "ucs-2be",         IDX_UCS2 },
  { "ucs2le",          IDX_UCS2LE },
  { "utf16",           IDX_UTF16 },
  { "utf16be",         IDX_UTF16 },
  { "utf-16be",        IDX_UTF16 },
  { "utf16le",         IDX_UTF16LE },
  { "ucs4",            IDX_UCS4 },
  { "ucs4be",          IDX_UCS4 },
  { "ucs-4be",         IDX_UCS4 },
  { "ucs4le",          IDX_UCS4LE },
  { "utf32",           IDX_UCS4 },
  { "utf-32",          IDX_UCS4 },
  { "utf32be",         IDX_UCS4 },
  { "utf-32be",        IDX_UCS4 },
  { "utf32le",         IDX_UCS4LE },
  { "utf-32le",        IDX_UCS4LE },
  { "932",             IDX_CP932 },
  { "949",             IDX_CP949 },
  { "936",             IDX_CP936 },
  { "gbk",             IDX_CP936 },
  { "950",             IDX_CP950 },
  { "eucjp",           IDX_EUC_JP },
  { "unix-jis",        IDX_EUC_JP },
  { "ujis",            IDX_EUC_JP },
  { "shift-jis",       IDX_SJIS },
  { "pck",             IDX_SJIS },        // Sun: PCK
  { "euckr",           IDX_EUC_KR },
  { "5601",            IDX_EUC_KR },      // Sun: KS C 5601
  { "euccn",           IDX_EUC_CN },
  { "gb2312",          IDX_EUC_CN },
  { "euctw",           IDX_EUC_TW },
  { "japan",           IDX_EUC_JP },
  { "korea",           IDX_EUC_KR },
  { "prc",             IDX_EUC_CN },
  { "zh-cn",           IDX_EUC_CN },
  { "chinese",         IDX_EUC_CN },
  { "zh-tw",           IDX_EUC_TW },
  { "taiwan",          IDX_EUC_TW },
  { "cp950",           IDX_BIG5 },
  { "950",             IDX_BIG5 },
  { "mac",             IDX_MACROMAN },
  { "mac-roman",       IDX_MACROMAN },
  { NULL,              0 }
};
