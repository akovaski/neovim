use crate::*;
extern "C" {
    pub fn try_getdigits(pp: *mut *mut libc::c_uchar, nr: *mut intmax_t) -> bool;
}
