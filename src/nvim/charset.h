#ifndef NVIM_CHARSET_H
#define NVIM_CHARSET_H

#include "nvim/types.h"
#include "nvim/pos.h"
#include "nvim/buffer_defs.h"
#include "nvim/eval/typval.h"
#include "nvim/option_defs.h"

/// Return the folded-case equivalent of the given character
///
/// @param[in]  c  Character to transform.
///
/// @return Folded variant.
#define CH_FOLD(c) \
    utf_fold((sizeof(c) == sizeof(char)) \
             ?((int)(uint8_t)(c)) \
             :((int)(c)))

/// Flags for vim_str2nr()
typedef enum {
  STR2NR_DEC = 0,
  STR2NR_BIN = (1 << 0),  ///< Allow binary numbers.
  STR2NR_OCT = (1 << 1),  ///< Allow octal numbers.
  STR2NR_HEX = (1 << 2),  ///< Allow hexadecimal numbers.
  /// Force one of the above variants.
  ///
  /// STR2NR_FORCE|STR2NR_DEC is actually not different from supplying zero
  /// as flags, but still present for completeness.
  STR2NR_FORCE = (1 << 3),
  /// Recognize all formats vim_str2nr() can recognize.
  STR2NR_ALL = STR2NR_BIN | STR2NR_OCT | STR2NR_HEX,
} ChStr2NrFlags;

int init_chartab(void);
int buf_init_chartab(buf_T *buf, int global);
void trans_characters(char_u *buf, int bufsize);
size_t transstr_len(const char *const s);
size_t transstr_buf(const char *const s, char *const buf, const size_t len);
char *transstr(const char *const s);
char_u* str_foldcase(char_u *str, int orglen, char_u *buf, int buflen);
char_u *transchar(int c);
char_u *transchar_byte(const int c);
void transchar_nonprint(char_u *buf, int c);
size_t transchar_hex(char *const buf, const int c);
int byte2cells(int b);
int char2cells(int c);
int ptr2cells(const char_u *p);
int vim_strsize(char_u *s);
int vim_strnsize(char_u *s, int len);
int chartabsize(char_u *p, colnr_T col);
int linetabsize(char_u *s);
int linetabsize_col(int startcol, char_u *s);
unsigned int win_linetabsize(win_T *wp, char_u *line, colnr_T len);
bool vim_isIDc(int c);
bool vim_iswordc(const int c);
bool vim_iswordc_tab(const int c, const uint64_t *const chartab);
bool vim_iswordc_buf(const int c, buf_T *const buf);
bool vim_iswordp(const char_u *const p);
bool vim_iswordp_buf(const char_u *const p, buf_T *const buf);
bool vim_isfilec(int c);
bool vim_isfilec_or_wc(int c);
bool vim_isprintc(int c);
bool vim_isprintc_strict(int c);
int lbr_chartabsize(char_u *line, unsigned char *s, colnr_T col);
int lbr_chartabsize_adv(char_u *line, char_u **s, colnr_T col);
int win_lbr_chartabsize(win_T *wp, char_u *line, char_u *s, colnr_T col, int *headp);
bool in_win_border(win_T *wp, colnr_T vcol);
void getvcol(win_T *wp, pos_T *pos, colnr_T *start, colnr_T *cursor,
             colnr_T *end);
colnr_T getvcol_nolist(pos_T *posp);
void getvvcol(win_T *wp, pos_T *pos, colnr_T *start, colnr_T *cursor,
              colnr_T *end);
void getvcols(win_T *wp, pos_T *pos1, pos_T *pos2, colnr_T *left,
              colnr_T *right);
char_u *skipwhite(const char_u *q);
intptr_t getwhitecols_curline(void);
intptr_t getwhitecols(const char_u *p);
char_u *skipdigits(const char_u *q);
const char* skipbin(const char *q);
char_u* skiphex(char_u *q);
char_u* skiptodigit(char_u *q);
const char* skiptobin(const char *q);
char_u* skiptohex(char_u *q);
char_u *skiptowhite(const char_u *p);
char_u* skiptowhite_esc(char_u *p);
bool try_getdigits(char_u **pp, intmax_t *nr);
intmax_t getdigits(char_u **pp, bool strict, intmax_t def);
int getdigits_int(char_u **pp, bool strict, int def);
long getdigits_long(char_u **pp, bool strict, long def);
bool vim_isblankline(char_u *lbuf);
void vim_str2nr(const char_u *const start, int *const prep, int *const len,
                const int what, varnumber_T *const nptr,
                uvarnumber_T *const unptr, const int maxlen);
int hex2nr(int c);
bool rem_backslash(const char_u *str);
void backslash_halve(char_u *p);
char_u *backslash_halve_save(const char_u *p);

static inline bool vim_isbreak(int c)
  REAL_FATTR_CONST
  REAL_FATTR_ALWAYS_INLINE;

/// Check if `c` is one of the characters in 'breakat'.
/// Used very often if 'linebreak' is set
static inline bool vim_isbreak(int c)
{
  return breakat_flags[(char_u)c];
}
#endif  // NVIM_CHARSET_H
