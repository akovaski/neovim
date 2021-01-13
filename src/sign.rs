use crate::*;

extern "C" {
    pub fn buf_delete_signs(buf: *mut buf_T, group: *const u8);
}
