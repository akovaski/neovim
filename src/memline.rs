use crate::*;
extern "C" {
    pub fn ml_get_buf(buf: *mut buf_T, lnum: linenr_T, will_change: bool) -> *mut libc::c_uchar;
}
