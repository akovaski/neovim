use crate::*;

extern "C" {
    pub fn diff_buf_delete(buf: *mut buf_T);
    pub fn diff_buf_add(buf: *mut buf_T);
    pub fn diff_check_fill(wp: *const win_T, lnum: linenr_T) -> libc::c_int;
    pub fn diffopt_hiddenoff() -> bool;
    pub fn diff_mode_buf(buf: *mut buf_T) -> bool;
}
