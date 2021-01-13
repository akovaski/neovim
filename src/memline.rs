use crate::*;
extern "C" {
    pub fn ml_open(buf: *mut buf_T) -> libc::c_int;
    pub fn ml_setname(buf: *mut buf_T);
    pub fn ml_close(buf: *mut buf_T, del_file: libc::c_int);
    pub fn ml_timestamp(buf: *mut buf_T);
    pub fn ml_recover(checkext: bool);
    pub fn ml_get(lnum: linenr_T) -> *mut u8;
    pub fn ml_get_buf(buf: *mut buf_T, lnum: linenr_T, will_change: bool) -> *mut u8;
    pub fn ml_delete(lnum: linenr_T, message: bool) -> libc::c_int;
    pub fn ml_find_line_or_offset(
        buf: *mut buf_T,
        lnum: linenr_T,
        offp: *mut libc::c_long,
        no_ff: bool,
    ) -> libc::c_long;
}
