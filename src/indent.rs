use crate::*;
extern "C" {
    pub fn get_breakindent_win(wp: *mut win_T, line: *mut libc::c_uchar) -> libc::c_int;
    pub fn parse_cino(buf: *mut buf_T);
    pub fn inindent(extra: libc::c_int) -> libc::c_int;
}
