extern "C" {
    pub fn os_path_exists(path: *const libc::c_uchar) -> bool;
}
