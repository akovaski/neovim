use crate::*;

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
#[inline]
pub unsafe extern "C" fn mb_strcmp_ic(
    ic: bool,
    s1: *const libc::c_char,
    s2: *const libc::c_char,
) -> libc::c_int {
    return if ic as libc::c_int != 0 {
        mb_stricmp(s1, s2)
    } else {
        strcmp(s1, s2)
    };
}
pub const mb_ptr2len: unsafe extern "C" fn(_: *const u8) -> libc::c_int = utfc_ptr2len;
pub const mb_char2len: unsafe extern "C" fn(_: libc::c_int) -> libc::c_int = utf_char2len;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub name: *const i8,
    pub prop: i32,
    pub codepage: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct interval {
    pub first: i64,
    pub last: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clinterval {
    pub first: u32,
    pub last: u32,
    pub class: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct convertStruct {
    pub rangeStart: i32,
    pub rangeEnd: i32,
    pub step: i32,
    pub offset: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub name: *const i8,
    pub canon: i32,
}
extern "C" {
    pub static mut e_loadlib: [u8; 32];
    pub static mut e_loadfunc: [u8; 41];
    pub static mut utf8len_tab: [u8; 256];
    pub static mut utf8len_tab_zero: [u8; 256];
}
pub const IDX_LATIN_1: i32 = 0;
pub const IDX_ISO_2: i32 = 1;
pub const IDX_ISO_3: i32 = 2;
pub const IDX_ISO_4: i32 = 3;
pub const IDX_ISO_5: i32 = 4;
pub const IDX_ISO_6: i32 = 5;
pub const IDX_ISO_7: i32 = 6;
pub const IDX_ISO_8: i32 = 7;
pub const IDX_ISO_9: i32 = 8;
pub const IDX_ISO_10: i32 = 9;
pub const IDX_ISO_11: i32 = 10;
pub const IDX_ISO_13: i32 = 11;
pub const IDX_ISO_14: i32 = 12;
pub const IDX_ISO_15: i32 = 13;
pub const IDX_UTF8: i32 = 16;
pub const IDX_UCS2: i32 = 17;
pub const IDX_UCS2LE: i32 = 18;
pub const IDX_UTF16: i32 = 19;
pub const IDX_UTF16LE: i32 = 20;
pub const IDX_UCS4: i32 = 21;
pub const IDX_UCS4LE: i32 = 22;
pub const IDX_EUC_JP: i32 = 24;
pub const IDX_SJIS: i32 = 25;
pub const IDX_EUC_KR: i32 = 26;
pub const IDX_EUC_CN: i32 = 27;
pub const IDX_EUC_TW: i32 = 28;
pub const IDX_BIG5: i32 = 29;
pub const IDX_CP932: i32 = 45;
pub const IDX_CP936: i32 = 46;
pub const IDX_CP949: i32 = 47;
pub const IDX_CP950: i32 = 48;
pub const IDX_MACROMAN: i32 = 57;
pub const IDX_COUNT: i32 = 59;
pub const ICONV_TESTLEN: i32 = 400;
extern "C" {
    pub fn enc_canon_props(name: *const u8) -> i32;
    pub fn bomb_size() -> i32;
    pub fn remove_bom(s: *mut u8);
    pub fn mb_get_class(p: *const u8) -> i32;
    pub fn mb_get_class_tab(p: *const u8, chartab: *const u64) -> i32;
    pub fn utf_char2cells(c: i32) -> i32;
    pub fn utf_ptr2cells(p: *const u8) -> i32;
    pub fn utf_ptr2cells_len(p: *const u8, size: i32) -> i32;
    pub fn mb_string2cells(str: *const u8) -> size_t;
    pub fn mb_string2cells_len(str: *const u8, size: size_t) -> size_t;
    pub fn utf_ptr2char(p: *const u8) -> i32;
    pub fn mb_ptr2char_adv(pp: *mut *const u8) -> i32;
    pub fn mb_cptr2char_adv(pp: *mut *const u8) -> i32;
    pub fn utf_composinglike(p1: *const u8, p2: *const u8) -> bool;
    pub fn utfc_ptr2char(p: *const u8, pcc: *mut i32) -> i32;
    pub fn utfc_ptr2char_len(p: *const u8, pcc: *mut i32, maxlen: i32) -> i32;
    pub fn utf_ptr2len(p: *const u8) -> i32;
    pub fn utf_byte2len(b: i32) -> i32;
    pub fn utf_ptr2len_len(p: *const u8, size: i32) -> i32;
    pub fn utfc_ptr2len(p: *const u8) -> i32;
    pub fn utfc_ptr2len_len(p: *const u8, size: i32) -> i32;
    pub fn utf_char2len(c: i32) -> i32;
    pub fn utf_char2bytes(c: i32, buf: *mut u8) -> i32;
    pub fn utf_iscomposing(c: i32) -> bool;
    pub fn utf_printable(c: i32) -> bool;
    pub fn utf_class(c: i32) -> i32;
    pub fn utf_class_tab(c: i32, chartab: *const u64) -> i32;
    pub fn utf_ambiguous_width(c: i32) -> bool;
    pub fn utf_fold(a: i32) -> i32;
    pub fn mb_toupper(a: i32) -> i32;
    pub fn mb_islower(a: i32) -> bool;
    pub fn mb_tolower(a: i32) -> i32;
    pub fn mb_isupper(a: i32) -> bool;
    pub fn mb_utflen(s: *const u8, len: size_t, codepoints: *mut size_t, codeunits: *mut size_t);
    pub fn mb_utf_index_to_bytes(
        s: *const u8,
        len: size_t,
        index: size_t,
        use_utf16_units: bool,
    ) -> isize;
    pub fn mb_strnicmp(s1: *const u8, s2: *const u8, nn: size_t) -> i32;
    pub fn mb_stricmp(s1: *const i8, s2: *const i8) -> i32;
    pub fn show_utf8();
    pub fn utf_head_off(base: *const u8, p: *const u8) -> i32;
    pub fn mb_copy_char(fp: *mut *const u8, tp: *mut *mut u8);
    pub fn mb_off_next(base: *mut u8, p: *mut u8) -> i32;
    pub fn mb_tail_off(base: *const u8, p: *const u8) -> i32;
    pub fn utf_find_illegal();
    pub fn mb_adjust_cursor();
    pub fn mb_check_adjust_col(win_: *mut libc::c_void);
    pub fn mb_prevptr(line: *mut u8, p: *mut u8) -> *mut u8;
    pub fn mb_charlen(str: *mut u8) -> i32;
    pub fn mb_charlen_len(str: *mut u8, len: i32) -> i32;
    pub fn mb_unescape(pp: *mut *const i8) -> *const i8;
    pub fn enc_skip(p: *mut u8) -> *mut u8;
    pub fn enc_canonize(enc: *mut u8) -> *mut u8;
    pub fn enc_locale() -> *mut u8;
    pub fn my_iconv_open(to: *mut u8, from: *mut u8) -> *mut libc::c_void;
    pub fn convert_setup(vcp: *mut vimconv_T, from: *mut u8, to: *mut u8) -> i32;
    pub fn convert_setup_ext(
        vcp: *mut vimconv_T,
        from: *mut u8,
        from_unicode_is_utf8: bool,
        to: *mut u8,
        to_unicode_is_utf8: bool,
    ) -> i32;
    pub fn string_convert(vcp: *const vimconv_T, ptr: *mut u8, lenp: *mut size_t) -> *mut u8;
    pub fn string_convert_ext(
        vcp: *const vimconv_T,
        ptr: *mut u8,
        lenp: *mut size_t,
        unconvlenp: *mut size_t,
    ) -> *mut u8;
}
