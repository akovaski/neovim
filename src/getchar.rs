use crate::*;

extern "C" {
    pub fn vgetc() -> libc::c_int;
    pub fn map_clear_int(buf: *mut buf_T, mode: libc::c_int, local: libc::c_int, abbr: libc::c_int);
}
