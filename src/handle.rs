use crate::*;

extern "C" {
    pub fn handle_get_buffer(handle: handle_T) -> *mut buf_T;
    pub fn handle_register_buffer(buffer: *mut buf_T);
    pub fn handle_unregister_buffer(buffer: *mut buf_T);
}
