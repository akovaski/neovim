use crate::*;

extern "C" {
    pub fn win_set_minimal_style(wp: *mut win_T);
    pub fn win_split(size: libc::c_int, flags: libc::c_int) -> libc::c_int;
    pub fn win_valid(win: *const win_T) -> bool;
    pub fn win_valid_any_tab(win: *mut win_T) -> bool;
    pub fn win_move_after(win1: *mut win_T, win2: *mut win_T);
    pub fn close_windows(buf: *mut buf_T, keep_curwin: libc::c_int);
    pub fn last_nonfloat(wp: *mut win_T) -> bool;
    pub fn win_close(win: *mut win_T, free_buf: bool) -> libc::c_int;
    pub fn valid_tabpage(tpc: *mut tabpage_T) -> bool;
    pub fn tabpage_index(ftp: *mut tabpage_T) -> libc::c_int;
    pub fn goto_tabpage_tp(
        tp: *mut tabpage_T,
        trigger_enter_autocmds: libc::c_int,
        trigger_leave_autocmds: libc::c_int,
    );
    pub fn goto_tabpage_win(tp: *mut tabpage_T, wp: *mut win_T);
    pub fn win_enter(wp: *mut win_T, undo_sync: bool);
    pub fn buf_jump_open_win(buf: *mut buf_T) -> *mut win_T;
    pub fn buf_jump_open_tab(buf: *mut buf_T) -> *mut win_T;
    pub fn tabline_height() -> libc::c_int;
}

pub const WSP_ABOVE: libc::c_int = 64 as libc::c_int;
pub const FNAME_UNESC: libc::c_int = 32 as libc::c_int;
pub const FNAME_HYP: libc::c_int = 4 as libc::c_int;
pub const WSP_HELP: libc::c_int = 16 as libc::c_int;
pub const WSP_VERT: libc::c_int = 2 as libc::c_int;
pub const FNAME_EXP: libc::c_int = 2 as libc::c_int;
pub const FNAME_REL: libc::c_int = 16 as libc::c_int;
pub const FNAME_MESS: libc::c_int = 1 as libc::c_int;
pub const WSP_BOT: libc::c_int = 8 as libc::c_int;
pub const WSP_BELOW: libc::c_int = 32 as libc::c_int;
pub const WSP_ROOM: libc::c_int = 1 as libc::c_int;
pub const WSP_TOP: libc::c_int = 4 as libc::c_int;
pub const WSP_NEWLOC: libc::c_int = 128 as libc::c_int;

pub const MIN_LINES: libc::c_int = 2 as libc::c_int;
