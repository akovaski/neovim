use crate::*;

extern "C" {
    // C std
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

    // strings.c
    pub fn sort_strings(files: *mut *mut libc::c_uchar, count: libc::c_int);
    pub fn vim_strsave(string: *const libc::c_uchar) -> *mut libc::c_uchar;
    pub fn vim_strchr(string: *const u8, c: libc::c_int) -> *mut u8;
    pub fn vim_snprintf_add(
        str: *const libc::c_char,
        str_m: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    pub fn vim_snprintf(
        str: *const libc::c_char,
        str_m: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;
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
