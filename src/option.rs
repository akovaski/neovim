use crate::*;
extern "C" {
    pub fn get_fileformat(buf: *mut buf_T) -> libc::c_int;
    pub fn skip_to_option_part(p: *const libc::c_uchar) -> *mut libc::c_uchar;
}
