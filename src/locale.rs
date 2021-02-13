pub const __LC_CTYPE: i32 = 0;
pub const LC_CTYPE: i32 = __LC_CTYPE;
extern "C" {
    pub fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
}
