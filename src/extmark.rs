use crate::*;

extern "C" {
    pub fn extmark_free_all(buf: *mut buf_T);
}
