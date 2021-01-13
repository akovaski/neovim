use crate::*;

extern "C" {
    pub fn keymap_init() -> *mut u8;
    pub fn keymap_ga_clear(kmap_ga: *mut garray_T);
}
