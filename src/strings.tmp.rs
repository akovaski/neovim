use crate::*;

pub const TMP_LEN: i32 = 350;
extern "C" {
    pub fn vim_strsave(string: *const u8) -> *mut u8;
    pub fn vim_strnsave(string: *const u8, len: size_t) -> *mut u8;
    pub fn vim_strsave_escaped(string: *const u8, esc_chars: *const u8) -> *mut u8;
    pub fn vim_strsave_escaped_ext(string: *const u8, esc_chars: *const u8, cc: u8, bsl: bool) -> *mut u8;
    pub fn vim_strnsave_unquoted(string: *const i8, length: size_t) -> *mut i8;
    pub fn vim_strsave_shellescape(string: *const u8, do_special: bool, do_newline: bool) -> *mut u8;
    pub fn vim_strsave_up(string: *const u8) -> *mut u8;
    pub fn vim_strnsave_up(string: *const u8, len: size_t) -> *mut u8;
    pub fn vim_strup(p: *mut u8);
    pub fn strcase_save(orig: *const i8, upper: bool) -> *mut i8;
    pub fn del_trailing_spaces(ptr: *mut u8);
    pub fn vim_strchr(string: *const u8, c: i32) -> *mut u8;
    pub fn sort_strings(files: *mut *mut u8, count: i32);
    pub fn has_non_ascii(s: *const u8) -> bool;
    pub fn has_non_ascii_len(s: *const i8, len: size_t) -> bool;
    pub fn concat_str(str1: *const u8, str2: *const u8) -> *mut u8;
    static mut e_printf: *const i8;
    pub fn vim_snprintf_add(str: *mut i8, str_m: size_t, fmt: *mut i8, args: ...) -> i32;
    pub fn vim_snprintf(str: *mut i8, str_m: size_t, fmt: *const i8, args: ...) -> i32;
    pub fn vim_vsnprintf(str: *mut i8, str_m: size_t, fmt: *const i8, ap: ::std::ffi::VaList) -> i32;
    pub fn vim_vsnprintf_typval(str: *mut i8, str_m: size_t, fmt: *const i8, ap: ::std::ffi::VaList, tvs: *mut typval_T) -> i32;
}
