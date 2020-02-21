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
}

pub unsafe fn strrchr(str: *const libc::c_char, character: char) -> *mut libc::c_char {
    c_strrchr(str, character as i32)
}
