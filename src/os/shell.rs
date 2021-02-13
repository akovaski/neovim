pub type ShellOpts = u32;
extern "C" {
    pub fn shell_free_argv(argv: *mut *mut libc::c_char);
}
