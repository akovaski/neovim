use crate::buffer_defs::*;
use crate::pos::pos_T;
use crate::*;

/*
use super::pos_h::linenr_T;
use super::typval_h::{sctx_T, scid_T};
use super::buffer_defs_h::{bufref_T, buf_T, win_T, tabpage_T, alist_T};
use super::garray_h::garray_T;
use super::ex_cmds_defs_h::cmdmod_T;
use super::regexp_defs_h::{regmatch_T, regprog_T};
use super::stdbool_h::true_0;
*/
extern "C" {
    pub static mut VIsual: pos_T;
}

// / The scope of a working-directory command like `:cd`.
// /
// / Scopes are enumerated from lowest to highest. When adding a scope make sure
// / to update all functions using scopes as well, such as the implementation of
// / `getcwd()`. When using scopes as limits (e.g. in loops) don't use the scopes
// / directly, use `MIN_CD_SCOPE` and `MAX_CD_SCOPE` instead.
pub type CdScope = libc::c_int;
// /< Affects the entire Nvim instance.
// /< Affects one tab page.
pub const kCdScopeGlobal: CdScope = 2;
// /< Affects one window.
pub const kCdScopeTab: CdScope = 1;
pub const kCdScopeWindow: CdScope = 0;
pub const kCdScopeInvalid: CdScope = -1;
// highlight attr for keep_msg
// keep_msg was set by msgmore()
// do fileinfo() after redraw
// msg_start() will scroll
// msg_outstr() was used in line
// msg_outstr() was used at all
// don't wait for this msg
// don't display errors for now,
// unless 'debug' is set.
// printing informative message
// don't add messages to history
// need to clear text before
// displaying a message.
// don't display errors for
// expression that is skipped
// use message of next of several
//  emsg() calls for throw
// just had ":endif"
// Dictionary with v: variables
// Dictionary with g: variables
// set by emsg() when the message
// is displayed or thrown
// set if vim_beep() is called
// did_emsg set because of a
// syntax error
// always set by emsg()
// exit value for ex mode
// there is an error message
// vim_regcomp() called emsg()
// don't wait for return for now
// need to wait for return later
// wait_return() was used and
// nothing written since then
// call maketitle() soon
// 'q' hit at "--more--" msg
// getexmodeline(): keep indent
// when inside vgetc() then > 0
// did set $VIM ourselves
// idem for $VIMRUNTIME
// / Lines left before a "more" message.  Ex mode needs to be able to reset this
// / after you type something.
// lines left for listing
// don't use more prompt, truncate
// messages
// name of error message source
// line number of the source file
// nesting level
// break below this level
// did "debug mode" message
// breakpoint change count
// breakpoint backtrace level
// Values for "do_profiling".
// /< profiling not started
// /< profiling busy
// /< profiling paused
// /< PROF_ values
// / Exception currently being thrown.  Used to pass an exception to a different
// / cstack.  Also used for discarding an exception before it is caught or made
// / pending.
// / Set when a throw that cannot be handled in do_cmdline() must be propagated
// / to the cstack of the previously called do_cmdline().
// / Set when a ":finish" or ":return" that cannot be handled in do_cmdline()
// / must be propagated to the cstack of the previously called do_cmdline().
// / Number of nested try conditionals (across function calls and ":source"
// / commands).
// / When "force_abort" is true, always skip commands after an error message,
// / even after the outermost ":endif", ":endwhile" or ":endfor" or for a
// / function without the "abort" flag.  It is set to true when "trylevel" is
// / non-zero (and ":silent!" was not used) or an exception is being thrown at
// / the time an error is detected.  It is set to false when "trylevel" gets
// / zero again and there was no error or interrupt or throw.
// / "msg_list" points to a variable in the stack of do_cmdline() which keeps
// / the list of arguments of several emsg() calls, one of which is to be
// / converted to an error exception immediately after the failing command
// / returns.  The message to be used for the exception value is pointed to by
// / the "throw_msg" field of the first element in the list.  It is usually the
// / same as the "msg" field of that element, but can be identical to the "msg"
// / field of a later list element, when the "emsg_severe" flag was set when the
// / emsg() call was made.
// / When set, don't convert an error to an exception.  Used when displaying the
// / interrupt message or reporting an exception that is still uncaught at the
// / top level (which has already been discarded then).  Also used for the error
// / message when no exception can be thrown.
// / The stack of all caught and not finished exceptions.  The exception on the
// / top of the stack is the one got by evaluation of v:exception.  The complete
// / stack of all caught and pending exceptions is embedded in the various
// / cstacks; the pending exceptions, however, are not on the caught stack.
// /
// / Garbage collection can only take place when we are sure there are no Lists
// / or Dictionaries being used internally.  This is flagged with
// / "may_garbage_collect" when we are at the toplevel.
// / "want_garbage_collect" is set by the garbagecollect() function, which means
// / we do garbage collection before waiting for a char at the toplevel.
// / "garbage_collect_at_exit" indicates garbagecollect(1) was called.
// /
// Special values for current_SID.
pub const SID_MODELINE: libc::c_int = -(1 as libc::c_int);
// when using a modeline
// for "--cmd" argument
// for "-c" argument
// for sourcing environment variable
// option was reset because of an error
// don't set scriptID
// for Lua scripts/chunks
// for API clients
// for sourcing a string
// Script CTX being sourced or was sourced to define the current function.
// ID of the current channel making a client API call
// Scope information for the code that indirectly triggered the current
// provider function call
// int value of T_CCO
// When highlight_match is true, highlight a match, starting at the cursor
// position.  Search_match_lines is the number of lines after the match (0 for
// a match within one line), search_match_endcol the column number of the
// character just after the match in the last line.
// show search match pos
// lines of of matched string
// col nr of match end
// don't use 'smartcase' once
// need to check file
// timestamps asap
// did check timestamps
// recently
// Don't check timestamps
// Is apply_autocmds() busy?
// *Enter autocmds disabled
// *Leave autocmds disabled
// did ":set modified"
// FileType event found
// value for did_filetype when starting to execute autocommands
// When deleting the current buffer, another one must be loaded.
// If we know which one is preferred, au_new_curbuf is set to it.
// When deleting a buffer/window and autocmd_busy is true, do not free the
// buffer/window. but link it in the list starting with
// au_pending_free_buf/ap_pending_free_win, using b_next/w_next.
// Free the buffer/window when autocmd_busy is being set to false.
// Mouse coordinates, set by handle_mouse_event()
// mouse below last line
// mouse right of line
// extending Visual area with
// mouse dragging
// The root of the menu hierarchy.
// While defining the system menu, sys_menu is true.  This avoids
// overruling of menus that the user already defined.
// While redrawing the screen this flag is set.  It means the screen size
// ('lines' and 'rows') must not be changed.
// All windows are linked in a list. firstwin points to the first entry,
// lastwin to the last entry (can be the same as firstwin) and curwin to the
// currently active window.
// first window
// last window
// previous window
// NOLINT
// When using this macro "break" only breaks out of the inner loop. Use "goto"
// to break out of the tabpage loop.
// -V:FOR_ALL_WINDOWS_IN_TAB:501
// currently active window
// window used in aucmd_prepbuf()
// aucmd_win is being used
// The window layout is kept in a tree of frames.  topframe points to the top
// of the tree.
// top of the window frame tree
// Tab pages are alternative topframes.  "first_tabpage" points to the first
// one in the list, "curtab" is the current one.
// need to redraw tabline
// Iterates over all tabs in the tab list
// All buffers are linked in a list. 'firstbuf' points to the first entry,
// 'lastbuf' to the last entry and 'curbuf' to the currently active buffer.
// first buffer
// last buffer
// currently active buffer
// Iterates over all buffers in the buffer list.
// Iterate through all the signs placed in a buffer
// NOLINT
// List of files being edited (global argument list).  curwin->w_alist points
// to this when the window is using the global argument list.
// global argument list
// /< the previous argument list id
// accessed last file in
// global_alist
// column for ruler
// 'rulerfmt' width of ruler when non-zero
// column for shown command
// When starting or exiting some things are done differently (e.g. screen
// updating).
// First NO_SCREEN, then NO_BUFFERS, then 0 when startup finished.
// true when planning to exit. Might keep running if there is a changed buffer.
// is stdin a terminal?
// is stdout a terminal?
// true when doing full-screen output, otherwise only writing some messages.
// volatile because it is used in a signal handler.
// When started in restricted mode (-Z).
// / Non-zero when only "safe" commands are allowed, e.g. when sourcing .exrc or
// / .vimrc in current directory.
// / Non-zero when changing text and jumping to another window/buffer is not
// / allowed.
// / Non-zero when the current buffer can't be changed.  Used for FileChangedRO.
// / Non-zero when no buffer name can be changed, no buffer can be deleted and
// / current directory can't be changed. Used for SwapExists et al.
// / Non-zero when evaluating an expression in a "sandbox".  Several things are
// / not allowed then.
// / Batch-mode: "-es" or "-Es" commandline argument was given.
// / Start position of active Visual selection.
// / Whether Visual mode is active.
// / Whether Select mode is active.
// / Whether to restart the selection after a Select-mode mapping or menu.
// / Type of Visual mode.
// / true when redoing Visual.
// / When pasting text with the middle mouse button in visual mode with
// / restart_edit set, remember where it started so we can set Insstart.
// This flag is used to make auto-indent work right on lines where only a
// <RETURN> or <ESC> is typed. It is set when an auto-indent is done, and
// reset when any other editing is done on the line. If an <ESC> or <RETURN>
// is received, and did_ai is true, the line is truncated.
// Column of first char after autoindent.  0 when no autoindent done.  Used
// when 'backspace' is 0, to avoid backspacing over autoindent.
// This is a character which will end a start-middle-end comment when typed as
// the first character on a new line.  It is taken from the last character of
// the "end" comment leader when the COM_AUTO_END flag is given for that
// comment end in 'comments'.  It is only valid when did_ai is true.
// This flag is set after a ":syncbind" to let the check_scrollbind() function
// know that it should not attempt to perform scrollbinding due to the scroll
// that was a result of the ":syncbind." (Otherwise, check_scrollbind() will
// undo some of the work done by ":syncbind.")  -ralston
// This flag is set when a smart indent has been performed. When the next typed
// character is a '{' the inserted tab will be deleted again.
// This flag is set after an auto indent. If the next typed character is a '}'
// one indent will be removed.
// This flag is set after an "O" command. If the next typed character is a '{'
// one indent will be removed.
// w_cursor before formatting text.
// Stuff for insert mode.
// This is where the latest
// insert/append mode started.
// This is where the latest insert/append mode started. In contrast to
// Insstart, this won't be reset by certain keys and is needed for
// op_insert(), to detect correctly where inserting by the user started.
// Stuff for VREPLACE mode.
// Line count when "gR" started
// #Lines changed by "gR" so far
// increase around internal delete/replace
// These flags are set based upon 'fileencoding'.
// Note that "enc_utf8" is also set for "unicode", because the characters are
// internally stored as UTF-8 (to avoid trouble with NUL bytes).
// japan
// euc-jp
// korea
// euc-kr
// chinese
// euc-cn
// taiwan
// euc-tw
// 2byte-
// mbyte flags that used to depend on 'encoding'. These are now deprecated, as
// 'encoding' is always "utf-8". Code that use them can be refactored to
// remove dead code.
// / Encoding used when 'fencs' is set to "default"
// / "State" is the main state of Vim.
// / There are other variables that modify the state:
// /    Visual_mode:    When State is NORMAL or INSERT.
// /    finish_op  :    When State is NORMAL, after typing the operator and
// /                    before typing the motion command.
// /    motion_force:   Last motion_force from do_pending_operator()
// /    debug_mode:     Debug mode
// This is the current state of the
// command interpreter.
// true while an operator is pending
// count for pending operator
// motion force for pending operator
// Ex Mode (Q) state
// Zero, EXMODE_NORMAL or EXMODE_VIM.
// No need to print after z or p.
// register for recording  or zero
// register being executed or zero
// currently no mapping allowed
// mapping zero not allowed
// Don't call u_sync()
// Call u_sync() once when evaluating
// an expression.
// force restart_edit after
// ex_normal returns
// call edit when next cmd finished
// Normally false, set to true after
// hitting cursor key in insert mode.
// Used by vgetorpeek() to decide when
// to call u_sync()
// put cursor after eol when
// restarting edit after CTRL-O
// msg for CTRL-X submode
// prepended to edit_submode
// appended to edit_submode
// highl. method for extra info
// true when no abbreviations loaded
// Modes where CTRL-C is mapped.
// Ex command modifiers
// don't print messages
// don't print error messages
// don't redirect error messages
// don't echo the command line
// Values for swap_exists_action: what to do when swap file already exists
// don't use dialog
pub const SEA_DIALOG: libc::c_int = 1 as libc::c_int;
pub const IOSIZE: i32 = 1024 + 1;
pub const has_mbyte: bool = true;
pub const SID_ERROR: libc::c_int = -(5 as libc::c_int);
pub const STL_IN_ICON: libc::c_int = 1 as libc::c_int;
pub const STL_IN_TITLE: libc::c_int = 2 as libc::c_int;
pub const SEA_NONE: libc::c_int = 0 as libc::c_int;
pub const SEA_RECOVER: libc::c_int = 3 as libc::c_int;
pub const SEA_QUIT: libc::c_int = 2 as libc::c_int;
extern "C" {
    pub static mut Rows: libc::c_int;
    pub static mut Columns: libc::c_int;
    pub static mut cmdline_row: libc::c_int;
    pub static mut msg_row: libc::c_int;
    pub static mut msg_scrolled: libc::c_int;
    pub static mut need_fileinfo: libc::c_int;
    pub static mut msg_scroll: libc::c_int;
    pub static mut called_emsg: libc::c_int;
    pub static mut need_wait_return: libc::c_int;
    pub static mut need_maketitle: libc::c_int;
    pub static mut sourcing_name: *mut u8;
    pub static mut sourcing_lnum: linenr_T;
    pub static mut current_sctx: sctx_T;
    pub static mut autocmd_busy: libc::c_int;
    pub static mut autocmd_no_enter: libc::c_int;
    pub static mut autocmd_no_leave: libc::c_int;
    pub static mut modified_was_set: libc::c_int;
    pub static mut did_filetype: libc::c_int;
    pub static mut au_new_curbuf: bufref_T;
    pub static mut au_pending_free_buf: *mut buf_T;
    pub static mut updating_screen: libc::c_int;
    pub static mut firstwin: *mut win_T;
    pub static mut lastwin: *mut win_T;
    pub static mut curwin: *mut win_T;
    pub static mut first_tabpage: *mut tabpage_T;
    pub static mut curtab: *mut tabpage_T;
    pub static mut firstbuf: *mut buf_T;
    pub static mut lastbuf: *mut buf_T;
    pub static mut curbuf: *mut buf_T;
    pub static mut global_alist: alist_T;
    pub static mut arg_had_last: bool;
    pub static mut starting: libc::c_int;
    pub static mut secure: libc::c_int;
    pub static mut VIsual_active: libc::c_int;
    pub static mut VIsual_reselect: libc::c_int;
    pub static mut State: libc::c_int;
    pub static mut restart_edit: libc::c_int;
    pub static mut cmdmod: cmdmod_T;
    pub static mut msg_silent: libc::c_int;
    pub static mut emsg_silent: libc::c_int;
    // use dialog when possible
    // quit editing the file
    // recover the file
    pub static mut swap_exists_action: libc::c_int;
    // For dialog when swap file already
    // exists.
    pub static mut swap_exists_did_quit: libc::c_int;
    // Selected "quit" at the dialog.
    pub static mut IObuff: [u8; 1025];
    // /< Buffer for sprintf, I/O, etc.
    pub static mut NameBuff: [u8; 4096];
    // /< Buffer for the os/ layer
    // When non-zero, postpone redrawing.
    pub static mut RedrawingDisabled: libc::c_int;
    pub static mut readonlymode: libc::c_int;
    // tick for each non-mapped char
    pub static mut must_redraw: libc::c_int;
    // /< Stream to write script to.
    // volatile because it is used in a signal handler.
    pub static mut got_int: libc::c_int;
    // /< cmdline recursion level
    pub static mut no_lines_msg: [u8; 0];
    // whether titlestring and iconstring contains statusline syntax
    pub static mut stl_syntax: libc::c_int;
    // Page number used for %N in 'pageheader' and 'guitablabel'.
    pub static mut printer_page_num: linenr_T;
    pub static mut e_noalt: [u8; 0];
    pub static mut e_trailing: [u8; 0];
    pub static mut e_nobufnr: [u8; 0];
}

macro_rules! ONE_WINDOW {
    () => {
        (firstwin == lastwin)
    };
}

// Iterates over all buffers in the buffer list.
macro_rules! FOR_ALL_BUFFERS {
    ($buf: ident, $blk: block) => {
        let mut $buf = firstbuf;
        while !$buf.is_null() {
            $blk;
            $buf = (*$buf).b_next;
        }
    };
}
macro_rules! FOR_ALL_BUFFERS_BACKWARDS {
    ($buf: ident, $blk: block) => {
        let mut $buf = lastbuf;
        while !$buf.is_null() {
            $blk;
            $buf = (*$buf).b_prev;
        }
    };
}

// Iterate through all the signs placed in a buffer
macro_rules! FOR_ALL_SIGNS_IN_BUF {
    ($buf: ident, $sign: ident, $blk: block) => {
        $sign = $buf.b_signlist;
        while !$sign.is_null() {
            $blk;
            $sign = (*$sign).next
        }
    };
}

// When using this macro "break" only breaks out of the inner loop. Use "goto"
// to break out of the tabpage loop.
macro_rules! FOR_ALL_TAB_WINDOWS {
    ($tp: ident, $wp: ident, $blk: block) => {
        FOR_ALL_TABS!($tp, {
            FOR_ALL_WINDOWS_IN_TAB!($wp, $tp, $blk);
        });
    };
}
macro_rules! FOR_ALL_WINDOWS_IN_TAB {
    ($wp: ident, $tp: ident, $blk: block) => {
        let mut $wp = if $tp == curtab {
            firstwin
        } else {
            (*$tp).tp_firstwin
        };
        while !$wp.is_null() {
            $blk;
            $wp = (*$wp).w_next;
        }
    };
}

// Iterates over all tabs in the tab list
macro_rules! FOR_ALL_TABS {
    ($tp: ident, $blk: block) => {
        let mut $tp = first_tabpage;
        while !$tp.is_null() {
            $blk;
            $tp = (*$tp).tp_next;
        }
    };
}
