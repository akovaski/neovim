use crate::*;

extern "C" {
    pub fn buf_updates_unregister_all(buf: *mut buf_T);
}
