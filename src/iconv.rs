use crate::*;
use once_cell::sync::Lazy;

//include iconv
pub type iconv_t = *mut libc::c_void;
extern "C" {
    pub fn iconv_open(__tocode: *const i8, __fromcode: *const i8) -> iconv_t;
    pub fn iconv(__cd: iconv_t, __inbuf: *mut *mut i8, __inbytesleft: *mut size_t, __outbuf: *mut *mut i8, __outbytesleft: *mut size_t) -> size_t;
    pub fn iconv_close(__cd: iconv_t) -> i32;
}
//nvim iconv
pub const ICONV_E2BIG: i32 = E2BIG;
pub const ICONV_ERRNO: Lazy<i32> = Lazy::new(|| unsafe { *__errno_location() } );
pub const ICONV_EINVAL: i32 = EINVAL;
pub const ICONV_EILSEQ: i32 = EILSEQ;

