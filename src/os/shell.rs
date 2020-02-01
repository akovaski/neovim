extern "C" {
    pub fn shell_free_argv(argv: *mut *mut libc::c_char);
}
