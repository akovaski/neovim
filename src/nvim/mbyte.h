#ifndef NVIM_MBYTE_H
#define NVIM_MBYTE_H

#include <stdint.h>
#include <stdbool.h>
#include <string.h>

#include "nvim/iconv.h"
#include "nvim/func_attr.h"
#include "nvim/os/os_defs.h"  // For indirect
#include "nvim/types.h"  // for char_u

/*
 * Return byte length of character that starts with byte "b".
 * Returns 1 for a single-byte character.
 * MB_BYTE2LEN_CHECK() can be used to count a special key as one byte.
 * Don't call MB_BYTE2LEN(b) with b < 0 or b > 255!
 */
#define MB_BYTE2LEN(b)         utf8len_tab[b]
#define MB_BYTE2LEN_CHECK(b)   (((b) < 0 || (b) > 255) ? 1 : utf8len_tab[b])

// max length of an unicode char
#define MB_MAXCHAR     6

/* properties used in enc_canon_table[] (first three mutually exclusive) */
#define ENC_8BIT       0x01
#define ENC_DBCS       0x02
#define ENC_UNICODE    0x04

#define ENC_ENDIAN_B   0x10        /* Unicode: Big endian */
#define ENC_ENDIAN_L   0x20        /* Unicode: Little endian */

#define ENC_2BYTE      0x40        /* Unicode: UCS-2 */
#define ENC_4BYTE      0x80        /* Unicode: UCS-4 */
#define ENC_2WORD      0x100       /* Unicode: UTF-16 */

#define ENC_LATIN1     0x200       /* Latin1 */
#define ENC_LATIN9     0x400       /* Latin9 */
#define ENC_MACROMAN   0x800       /* Mac Roman (not Macro Man! :-) */

// TODO(bfredl): eventually we should keep only one of the namings
#define mb_ptr2len utfc_ptr2len
#define mb_char2len utf_char2len
#define mb_char2cells utf_char2cells

/// Flags for vimconv_T
typedef enum {
  CONV_NONE      = 0,
  CONV_TO_UTF8   = 1,
  CONV_9_TO_UTF8 = 2,
  CONV_TO_LATIN1 = 3,
  CONV_TO_LATIN9 = 4,
  CONV_ICONV     = 5,
} ConvFlags;

#define MBYTE_NONE_CONV { \
  .vc_type = CONV_NONE, \
  .vc_factor = 1, \
  .vc_fail = false, \
}

/// Structure used for string conversions
typedef struct {
  int vc_type;  ///< Zero or more ConvFlags.
  int vc_factor;  ///< Maximal expansion factor.
# ifdef HAVE_ICONV
  iconv_t vc_fd;  ///< Value for CONV_ICONV.
# endif
  bool vc_fail;  ///< What to do with invalid characters: if true, fail,
                 ///< otherwise use '?'.
} vimconv_T;

extern const uint8_t utf8len_tab_zero[256];

extern const uint8_t utf8len_tab[256];

#ifdef INCLUDE_GENERATED_DECLARATIONS
# include "mbyte.h.generated.h"
#endif

static inline int mb_strcmp_ic(bool ic, const char *s1, const char *s2)
  REAL_FATTR_NONNULL_ALL REAL_FATTR_PURE REAL_FATTR_WARN_UNUSED_RESULT;

/// Compare strings
///
/// @param[in]  ic  True if case is to be ignored.
///
/// @return 0 if s1 == s2, <0 if s1 < s2, >0 if s1 > s2.
static inline int mb_strcmp_ic(bool ic, const char *s1, const char *s2)
{
  return (ic ? mb_stricmp(s1, s2) : strcmp(s1, s2));
}
#endif  // NVIM_MBYTE_H

int enc_canon_props(const char_u *name);
int bomb_size(void);
void remove_bom(char_u *s);
int mb_get_class(const char_u *p);
int mb_get_class_tab(const char_u *p, const uint64_t *const chartab);
int utf_char2cells(int c);
int utf_ptr2cells(const char_u *p);
int utf_ptr2cells_len(const char_u *p, int size);
size_t mb_string2cells(const char_u *str);
size_t mb_string2cells_len(const char_u *str, size_t size);
int utf_ptr2char(const char_u *const p) FUNC_ATTR_PURE FUNC_ATTR_WARN_UNUSED_RESULT;
int mb_ptr2char_adv(const char_u **const pp);
int mb_cptr2char_adv(const char_u **pp);
_Bool utf_composinglike(const char_u *p1, const char_u *p2);
int utfc_ptr2char(const char_u *p, int *pcc);
int utfc_ptr2char_len(const char_u *p, int *pcc, int maxlen);
int utf_ptr2len(const char_u *const p) FUNC_ATTR_PURE FUNC_ATTR_WARN_UNUSED_RESULT FUNC_ATTR_NONNULL_ALL;
int utf_byte2len(int b);
int utf_ptr2len_len(const char_u *p, int size);
int utfc_ptr2len(const char_u *const p) FUNC_ATTR_PURE FUNC_ATTR_WARN_UNUSED_RESULT FUNC_ATTR_NONNULL_ALL;
int utfc_ptr2len_len(const char_u *p, int size);
int utf_char2len(const int c);
int utf_char2bytes(const int c, char_u *const buf);
_Bool utf_iscomposing(int c);
_Bool utf_printable(int c);
int utf_class(const int c);
int utf_class_tab(const int c, const uint64_t *const chartab);
_Bool utf_ambiguous_width(int c);
int utf_fold(int a);
int mb_toupper(int a);
_Bool mb_islower(int a);
int mb_tolower(int a);
_Bool mb_isupper(int a);
void mb_utflen(const char_u *s, size_t len, size_t *codepoints, size_t *codeunits) FUNC_ATTR_NONNULL_ALL;
ssize_t mb_utf_index_to_bytes(const char_u *s, size_t len, size_t index, _Bool use_utf16_units) FUNC_ATTR_NONNULL_ALL;
int mb_strnicmp(const char_u *s1, const char_u *s2, const size_t nn);
int mb_stricmp(const char *s1, const char *s2);
void show_utf8(void);
int utf_head_off(const char_u *base, const char_u *p);
void mb_copy_char(const char_u **const fp, char_u **const tp);
int mb_off_next(char_u *base, char_u *p);
int mb_tail_off(char_u *base, char_u *p);
void utf_find_illegal(void);
void mb_adjust_cursor(void);
void mb_check_adjust_col(void *win_);
char_u *mb_prevptr(char_u *line, char_u *p );
int mb_charlen(char_u *str);
int mb_charlen_len(char_u *str, int len);
const char *mb_unescape(const char **const pp) FUNC_ATTR_WARN_UNUSED_RESULT FUNC_ATTR_NONNULL_ALL;
char_u *enc_skip(char_u *p);
char_u *enc_canonize(char_u *enc) FUNC_ATTR_NONNULL_RET;
char_u *enc_locale(void);
void *my_iconv_open(char_u *to, char_u *from);
int convert_setup(vimconv_T *vcp, char_u *from, char_u *to);
int convert_setup_ext(vimconv_T *vcp, char_u *from, _Bool from_unicode_is_utf8, char_u *to, _Bool to_unicode_is_utf8);
char_u *string_convert(const vimconv_T *const vcp, char_u *ptr, size_t *lenp);
char_u *string_convert_ext(const vimconv_T *const vcp, char_u *ptr, size_t *lenp, size_t *unconvlenp);
