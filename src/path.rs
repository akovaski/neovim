extern "C" {
    fn path_fnamecmp(fname1: *const libc::c_char, fname2: *const libc::c_char) -> libc::c_int;
    pub fn path_tail(fname: *const libc::c_uchar) -> *mut libc::c_uchar;
    pub fn path_has_wildcard(p: *const libc::c_uchar) -> bool;
}

pub unsafe fn fnamecmp<T, U>(fname1: *const T, fname2: *const U) -> i32 {
    path_fnamecmp(fname1 as *const libc::c_char, fname2 as *const libc::c_char) as i32
}
