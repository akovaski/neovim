use crate::*;
extern "C" {
    pub static utf8len_tab: [u8; 256];
    pub fn utfc_ptr2len_len(p: *const libc::c_uchar, size: libc::c_int) -> libc::c_int;
    pub fn utf_char2len(c: libc::c_int) -> libc::c_int;
    pub fn utf_char2bytes(c: libc::c_int, buf: *mut libc::c_uchar) -> libc::c_int;
    pub fn mb_copy_char(fp: *mut *const libc::c_uchar, tp: *mut *mut libc::c_uchar);
    pub fn string_convert(
        vcp: *const vimconv_T,
        ptr: *mut libc::c_uchar,
        lenp: *mut libc::size_t,
    ) -> *mut libc::c_uchar;
    pub fn utf_char2cells(c: libc::c_int) -> libc::c_int;
    pub fn utf_ptr2cells(p: *const libc::c_uchar) -> libc::c_int;
    pub fn utf_ptr2char(p: *const libc::c_uchar) -> libc::c_int;
    pub fn mb_ptr2char_adv(pp: *mut *const libc::c_uchar) -> libc::c_int;
    pub fn utfc_ptr2char(p: *const libc::c_uchar, pcc: *mut libc::c_int) -> libc::c_int;
    pub fn utf_ptr2len(p: *const libc::c_uchar) -> libc::c_int;
    pub fn utfc_ptr2len(p: *const libc::c_uchar) -> libc::c_int;
    pub fn utf_printable(c: libc::c_int) -> bool;
    pub fn utf_class_tab(c: libc::c_int, chartab: *const u64) -> libc::c_int;
    pub fn mb_islower(a: libc::c_int) -> bool;
    pub fn mb_tolower(a: libc::c_int) -> libc::c_int;
    pub fn mb_isupper(a: libc::c_int) -> bool;
    pub fn utf_head_off(base: *const libc::c_uchar, p: *const libc::c_uchar) -> libc::c_int;
    pub fn mb_charlen(str: *mut libc::c_uchar) -> libc::c_int;
}

pub unsafe fn MB_BYTE2LEN(b: u8) -> u8 {
    utf8len_tab[b as usize]
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
