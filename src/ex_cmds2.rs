use crate::*;

extern "C" {
    pub fn dialog_close_terminal(buf: *mut buf_T) -> bool;
    pub fn can_abandon(buf: *mut buf_T, forceit: libc::c_int) -> bool;
    pub fn dialog_changed(buf: *mut buf_T, checkall: bool);
    pub fn check_arg_idx(win: *mut win_T);
    pub fn autowrite(buf: *mut buf_T, forceit: libc::c_int) -> libc::c_int;
}
