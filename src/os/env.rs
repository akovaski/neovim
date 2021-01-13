use crate::*;

extern "C" {
    pub fn home_replace_save(buf: *mut buf_T, src: *mut u8) -> *mut u8;
    pub fn home_replace(
        buf: *const buf_T,
        src: *const u8,
        dst: *mut u8,
        dstlen: size_t,
        one: bool,
    ) -> size_t;
}
