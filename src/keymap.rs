extern "C" {
    pub fn trans_special(
        srcp: *mut *const libc::c_uchar,
        src_len: libc::size_t,
        dst: *mut libc::c_uchar,
        keycode: bool,
        in_string: bool,
    ) -> libc::c_uint;
}
