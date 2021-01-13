#ifndef NVIM_BUFFER_H
#define NVIM_BUFFER_H

#include "nvim/vim.h"
#include "nvim/window.h"
#include "nvim/pos.h"  // for linenr_T
#include "nvim/ex_cmds_defs.h"  // for exarg_T
#include "nvim/screen.h"  // for StlClickRecord
#include "nvim/func_attr.h"
#include "nvim/eval.h"
#include "nvim/macros.h"

// Values for buflist_getfile()
enum getf_values {
  GETF_SETMARK = 0x01, // set pcmark before jumping
  GETF_ALT     = 0x02, // jumping to alternate file (not buf num)
  GETF_SWITCH  = 0x04, // respect 'switchbuf' settings when jumping
};

// Return values of getfile()
enum getf_retvalues {
  GETFILE_ERROR       = 1,    // normal error
  GETFILE_NOT_WRITTEN = 2,    // "not written" error
  GETFILE_SAME_FILE   = 0,    // success, same file
  GETFILE_OPEN_OTHER  = -1,   // success, opened another file
  GETFILE_UNUSED      = 8
};

// Values for buflist_new() flags
enum bln_values {
  BLN_CURBUF = 1,   // May re-use curbuf for new buffer
  BLN_LISTED = 2,   // Put new buffer in buffer list
  BLN_DUMMY  = 4,   // Allocating dummy buffer
  BLN_NEW    = 8,   // create a new buffer
  BLN_NOOPT  = 16,  // Don't copy options to existing buffer
};

// Values for action argument for do_buffer()
enum dobuf_action_values {
  DOBUF_GOTO   = 0, // go to specified buffer
  DOBUF_SPLIT  = 1, // split window and go to specified buffer
  DOBUF_UNLOAD = 2, // unload specified buffer(s)
  DOBUF_DEL    = 3, // delete specified buffer(s) from buflist
  DOBUF_WIPE   = 4, // delete specified buffer(s) really
};

// Values for start argument for do_buffer()
enum dobuf_start_values {
  DOBUF_CURRENT = 0, // "count" buffer from current buffer
  DOBUF_FIRST   = 1, // "count" buffer from first buffer
  DOBUF_LAST    = 2, // "count" buffer from last buffer
  DOBUF_MOD     = 3, // "count" mod. buffer from current buffer
};

// flags for buf_freeall()
enum bfa_values {
  BFA_DEL       = 1, // buffer is going to be deleted
  BFA_WIPE      = 2, // buffer is going to be wiped out
  BFA_KEEP_UNDO = 4, // do not free undo information
};

#ifdef INCLUDE_GENERATED_DECLARATIONS
# include "buffer.h.generated.h"
#endif

static inline void buf_set_changedtick(buf_T *const buf,
                                       const varnumber_T changedtick)
  REAL_FATTR_NONNULL_ALL REAL_FATTR_ALWAYS_INLINE;

/// Set b:changedtick, also checking b: for consistency in debug build
///
/// @param[out]  buf  Buffer to set changedtick in.
/// @param[in]  changedtick  New value.
static inline void buf_set_changedtick(buf_T *const buf,
                                       const varnumber_T changedtick)
{
  typval_T old_val = buf->changedtick_di.di_tv;

#ifndef NDEBUG
  dictitem_T *const changedtick_di = tv_dict_find(
      buf->b_vars, S_LEN("changedtick"));
  assert(changedtick_di != NULL);
  assert(changedtick_di->di_tv.v_type == VAR_NUMBER);
  assert(changedtick_di->di_tv.v_lock == VAR_FIXED);
  // For some reason formatc does not like the below.
# ifndef UNIT_TESTING_LUA_PREPROCESSING
  assert(changedtick_di->di_flags == (DI_FLAGS_RO|DI_FLAGS_FIX));
# endif
  assert(changedtick_di == (dictitem_T *)&buf->changedtick_di);
#endif
  buf->changedtick_di.di_tv.vval.v_number = changedtick;

  if (tv_dict_is_watched(buf->b_vars)) {
    tv_dict_watcher_notify(buf->b_vars,
                           (char *)buf->changedtick_di.di_key,
                           &buf->changedtick_di.di_tv,
                           &old_val);
  }
}

static inline varnumber_T buf_get_changedtick(const buf_T *const buf)
  REAL_FATTR_NONNULL_ALL REAL_FATTR_ALWAYS_INLINE REAL_FATTR_PURE
  REAL_FATTR_WARN_UNUSED_RESULT;

/// Get b:changedtick value
///
/// Faster then querying b:.
///
/// @param[in]  buf  Buffer to get b:changedtick from.
static inline varnumber_T buf_get_changedtick(const buf_T *const buf)
{
  return buf->changedtick_di.di_tv.vval.v_number;
}

static inline void buf_inc_changedtick(buf_T *const buf)
  REAL_FATTR_NONNULL_ALL REAL_FATTR_ALWAYS_INLINE;

/// Increment b:changedtick value
///
/// Also checks b: for consistency in case of debug build.
///
/// @param[in,out]  buf  Buffer to increment value in.
static inline void buf_inc_changedtick(buf_T *const buf)
{
  buf_set_changedtick(buf, buf_get_changedtick(buf) + 1);
}
int open_buffer( int read_stdin, exarg_T *eap, int flags);
void set_bufref(bufref_T *bufref, buf_T *buf);
bool bufref_valid(bufref_T *bufref);
bool buf_valid(buf_T *buf);
void close_buffer(win_T *win, buf_T *buf, int action, bool abort_if_last);

void buf_clear_file(buf_T *buf);
void buf_clear(void);
void buf_freeall(buf_T *buf, int flags);
void free_buffer(buf_T *buf);
void free_buffer_stuff(buf_T *buf, int free_flags);
void goto_buffer(exarg_T *eap, int start, int dir, int count);
void handle_swap_exists(bufref_T *old_curbuf);
char_u *
do_bufdel(
    int command,
    char_u *arg,               // pointer to extra arguments
    int addr_count,
    int start_bnr,             // first buffer number in a range
    int end_bnr,               // buffer nr or last buffer nr in a range
    int forceit
);
int
do_buffer(
    int action,
    int start,
    int dir,                        // FORWARD or BACKWARD
    int count,                      // buffer number or number of buffers
    int forceit                     // true for :...!
);
void set_curbuf(buf_T *buf, int action);
void enter_buffer(buf_T *buf);
void do_autochdir(void);
void no_write_message(void);
void no_write_message_nobang(void);
buf_T * buflist_new(char_u *ffname, char_u *sfname, linenr_T lnum, int flags);
bool curbuf_reusable(void);
void free_buf_options(buf_T *buf, int free_p_ff);
int buflist_getfile(int n, linenr_T lnum, int options, int forceit);
void buflist_getfpos(void);
buf_T *buflist_findname_exp(char_u *fname);
buf_T *buflist_findname(char_u *ffname);
int buflist_findpat(
    const char_u *pattern,
    const char_u *pattern_end,  // pointer to first char after pattern
    int unlisted,               // find unlisted buffers
    int diffmode,               // find diff-mode buffers only
    int curtab_only             // find buffers in current tab only
);
int ExpandBufnames(char_u *pat, int *num_file, char_u ***file, int options);
buf_T *buflist_findnr(int nr);
char_u *
buflist_nr2name(
    int n,
    int fullname,
    int helptail                   // for help buffers return tail only
);
void buflist_setfpos(buf_T *const buf, win_T *const win,
                     linenr_T lnum, colnr_T col,
                     bool copy_options);
void get_winopts(buf_T *buf);
pos_T *buflist_findfpos(buf_T *buf);
linenr_T buflist_findlnum(buf_T *buf);
void buflist_list(exarg_T *eap);
int buflist_name_nr(int fnum, char_u **fname, linenr_T *lnum);
int
setfname(
    buf_T *buf,
    char_u *ffname,
    char_u *sfname,
    bool message                  // give message when buffer already exists
);
void buf_set_name(int fnum, char_u *name);
void buf_name_changed(buf_T *buf);
buf_T *setaltfname(char_u *ffname, char_u *sfname, linenr_T lnum);
char_u * getaltfname(
    bool errmsg                   // give error message
);
int buflist_add(char_u *fname, int flags);
void buflist_altfpos(win_T *win);
bool otherfile(char_u *ffname);
void buf_set_file_id(buf_T *buf);
void
fileinfo(
    int fullname,               // when non-zero print full path
    int shorthelp,
    int dont_truncate
);
void col_print(char_u *buf, size_t buflen, int col, int vcol);
void maketitle(void);
void resettitle(void);
int build_stl_str_hl(
    win_T *wp,
    char_u *out,
    size_t outlen,
    char_u *fmt,
    int use_sandbox,
    char_u fillchar,
    int maxwidth,
    struct stl_hlrec *hltab,
    StlClickRecord *tabtab
);
void get_rel_pos(win_T *wp, char_u *buf, int buflen);
void fname_expand(buf_T *buf, char_u **ffname, char_u **sfname);
char_u *alist_name(aentry_T *aep);
void
do_arg_all(
    int count,
    int forceit,                  // hide buffers in current windows
    int keep_tabs                 // keep current tabs, for ":tab drop file"
);
int bt_prompt(buf_T *buf);
void ex_buffer_all(exarg_T *eap);
void do_modelines(int flags);
bool bt_help(const buf_T *const buf);
bool bt_normal(const buf_T *const buf);
bool bt_quickfix(const buf_T *const buf);
bool bt_terminal(const buf_T *const buf);
bool bt_nofile(const buf_T *const buf);
bool bt_dontwrite(const buf_T *const buf);
bool bt_dontwrite_msg(const buf_T *const buf);
bool buf_hide(const buf_T *const buf);
char_u *buf_spname(buf_T *buf);
bool find_win_for_buf(buf_T *buf, win_T **wp, tabpage_T **tp);
int buf_signcols(buf_T *buf);
void set_buflisted(int on);
bool buf_contents_changed(buf_T *buf);
void
wipe_buffer(
    buf_T *buf,
    int aucmd                   // When true trigger autocommands.
);
void buf_open_scratch(handle_T bufnr, char *bufname);
int empty_curbuf(int close_others, int forceit, int action);
void clear_wininfo(buf_T *buf);

#endif  // NVIM_BUFFER_H
