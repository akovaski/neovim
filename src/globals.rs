use crate::buffer_defs::*;

extern "C" {
    pub static mut curbuf: *mut buf_T;
    pub static mut got_int: libc::c_int;
}
