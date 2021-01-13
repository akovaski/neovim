extern "C" {
    pub fn vim_chdirfile(fname: *mut u8) -> libc::c_int;
}
