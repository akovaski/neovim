extern "C" {
    // C std
    pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;

    // strings.c
    pub fn sort_strings(files: *mut *mut libc::c_uchar, count: libc::c_int);
}
