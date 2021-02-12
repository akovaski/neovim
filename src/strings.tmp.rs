use crate::*;

pub const TMP_LEN: i32 = 350;
extern "C" {
    pub fn vim_strsave(mut string: *const u8) -> *mut u8;
    pub fn vim_strnsave(mut string: *const u8, mut len: size_t) -> *mut u8;
    pub fn vim_strsave_escaped(mut string: *const u8, mut esc_chars: *const u8) -> *mut u8;
    pub fn vim_strsave_escaped_ext(mut string: *const u8, mut esc_chars: *const u8, mut cc: u8, mut bsl: bool) -> *mut u8;
    pub fn vim_strnsave_unquoted(string: *const i8, length: size_t) -> *mut i8;
    pub fn vim_strsave_shellescape(mut string: *const u8, mut do_special: bool, mut do_newline: bool) -> *mut u8;
    pub fn vim_strsave_up(mut string: *const u8) -> *mut u8;
    pub fn vim_strnsave_up(mut string: *const u8, mut len: size_t) -> *mut u8;
    pub fn vim_strup(mut p: *mut u8);
    pub fn strcase_save(orig: *const i8, mut upper: bool) -> *mut i8;
    pub fn del_trailing_spaces(mut ptr: *mut u8);
    pub fn vim_strchr(string: *const u8, c: i32) -> *mut u8;
    pub fn sort_strings(mut files: *mut *mut u8, mut count: i32);
    pub fn has_non_ascii(mut s: *const u8) -> bool;
    pub fn has_non_ascii_len(s: *const i8, len: size_t) -> bool;
    pub fn concat_str(mut str1: *const u8, mut str2: *const u8) -> *mut u8;
    static mut e_printf: *const i8;
    pub fn vim_snprintf_add(mut str: *mut i8, mut str_m: size_t, mut fmt: *mut i8, mut args: ...) -> i32;
    pub fn vim_snprintf(mut str: *mut i8, mut str_m: size_t, mut fmt: *const i8, mut args: ...) -> i32;
    pub fn vim_vsnprintf(mut str: *mut i8, mut str_m: size_t, mut fmt: *const i8, mut ap: ::std::ffi::VaList) -> i32;
    pub fn vim_vsnprintf_typval(mut str: *mut i8, mut str_m: size_t, mut fmt: *const i8, mut ap: ::std::ffi::VaList, tvs: *mut typval_T) -> i32;
}
