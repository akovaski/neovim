extern "C" {
    fn path_fnamecmp(fname1: *const libc::c_char, fname2: *const libc::c_char) -> libc::c_int;
}

pub unsafe fn fnamecmp<T, U>(fname1: *const T, fname2: *const U) -> i32 {
    path_fnamecmp(fname1 as *const libc::c_char, fname2 as *const libc::c_char) as i32
}
