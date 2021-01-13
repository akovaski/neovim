use crate::*;

extern "C" {
    pub fn did_set_spelllang(wp: *mut win_T) -> *mut u8;
}
