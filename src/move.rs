use crate::*;
extern "C" {
    pub fn win_col_off(wp: *mut win_T) -> libc::c_int;
    pub fn win_col_off2(wp: *mut win_T) -> libc::c_int;
}
