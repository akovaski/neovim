#![allow(dead_code)]
use crate::*;

pub const TMP_LEN: i32 = 350;
extern "C" {
    // <string.h>
    pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    pub fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    pub fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[link_name = "strrchr"]
    pub fn c_strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    pub fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;

    pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;

    // strings.c
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
    pub fn vim_snprintf_add(str: *mut i8, str_m: size_t, fmt: *const i8, args: ...) -> i32;
    pub fn vim_snprintf(str: *mut i8, str_m: size_t, fmt: *const i8, args: ...) -> i32;
    pub fn vim_vsnprintf(str: *mut i8, str_m: size_t, fmt: *const i8, ap: ::std::ffi::VaList) -> i32;
    pub fn vim_vsnprintf_typval(str: *mut i8, str_m: size_t, fmt: *const i8, ap: ::std::ffi::VaList, tvs: *mut typval_T) -> i32;
}
#[inline]
pub unsafe extern "C" fn strappend(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
) -> *mut libc::c_char {
    let src_len = strlen(src);
    return (memmove(
        dst as *mut libc::c_void,
        src as *const libc::c_void,
        src_len,
    ) as *mut libc::c_char)
        .offset(src_len as isize);
}

pub unsafe fn strrchr(str: *const libc::c_char, character: char) -> *mut libc::c_char {
    c_strrchr(str, character as i32)
}
