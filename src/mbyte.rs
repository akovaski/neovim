use crate::*;
extern "C" {
    pub fn utfc_ptr2len_len(p: *const libc::c_uchar, size: libc::c_int) -> libc::c_int;
    pub fn utf_char2len(c: libc::c_int) -> libc::c_int;
    pub fn utf_char2bytes(c: libc::c_int, buf: *mut libc::c_uchar) -> libc::c_int;
    pub fn mb_copy_char(fp: *mut *const libc::c_uchar, tp: *mut *mut libc::c_uchar);
    pub fn string_convert(
        vcp: *const vimconv_T,
        ptr: *mut libc::c_uchar,
        lenp: *mut libc::size_t,
    ) -> *mut libc::c_uchar;
}

// / Flags for vimconv_T
pub type ConvFlags = libc::c_uint;
pub const CONV_ICONV: ConvFlags = 5;
pub const CONV_TO_LATIN9: ConvFlags = 4;
pub const CONV_TO_LATIN1: ConvFlags = 3;
pub const CONV_9_TO_UTF8: ConvFlags = 2;
pub const CONV_TO_UTF8: ConvFlags = 1;
pub const CONV_NONE: ConvFlags = 0;
// / Structure used for string conversions

#[derive(Copy, Clone)]
#[repr(C)]
pub struct vimconv_T {
    pub vc_type: libc::c_int,
    pub vc_factor: libc::c_int,
    pub vc_fd: iconv_t,
    pub vc_fail: bool,
}
