use crate::*;
extern "C" {
    pub fn get_breakindent_win(wp: *mut win_T, line: *mut libc::c_uchar) -> libc::c_int;
}
