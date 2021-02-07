use crate::*;

pub const MAX_ENVPATHLEN: i32 = 2147483647;
extern "C" {
    pub fn env_init();
    pub fn os_env_var_lock();
    pub fn os_env_var_unlock();
    pub fn os_getenv(name: *const i8) -> *const i8;
    pub fn os_env_exists(name: *const i8) -> bool;
    pub fn os_setenv(name: *const i8, value: *const i8, overwrite: i32) -> i32;
    pub fn os_unsetenv(name: *const i8) -> i32;
    pub fn os_get_fullenv_size() -> size_t;
    pub fn os_free_fullenv(env: *mut *mut i8);
    pub fn os_copy_fullenv(env: *mut *mut i8, env_size: size_t);
    pub fn os_getenvname_at_index(index: size_t) -> *mut i8;
    pub fn os_get_pid() -> i64;
    pub fn os_get_hostname(hostname: *mut i8, size: size_t);
    pub fn init_homedir();
    pub fn expand_env_save(src: *const u8) -> *mut u8;
    pub fn expand_env_save_opt(src: *const u8, one: bool) -> *mut u8;
    pub fn expand_env(src: *const u8, dst: *mut u8, dstlen: i32);
    pub fn expand_env_esc(
        srcp: *const u8,
        dst: *mut u8,
        dstlen: i32,
        esc: bool,
        one: bool,
        prefix: *mut u8,
    );
    pub fn vim_env_iter(
        delim: i8,
        val: *const i8,
        iter: *const libc::c_void,
        dir: *mut *const i8,
        len: *mut size_t,
    ) -> *const libc::c_void;
    pub fn vim_env_iter_rev(
        delim: i8,
        val: *const i8,
        iter: *const libc::c_void,
        dir: *mut *const i8,
        len: *mut size_t,
    ) -> *const libc::c_void;
    pub fn vim_get_prefix_from_exepath(exe_name: *mut i8);
    pub fn vim_getenv(name: *const i8) -> *mut i8;
    pub fn home_replace(
        buf: *const buf_T,
        src: *const u8,
        dst: *mut u8,
        dstlen: size_t,
        one: bool,
    ) -> size_t;
    pub fn home_replace_save(buf: *mut buf_T, src: *const u8) -> *mut u8;
    pub fn get_env_name(xp: *mut expand_T, idx: i32) -> *mut u8;
    pub fn os_setenv_append_path(fname: *const i8) -> bool;
    pub fn os_shell_is_cmdexe(sh: *const i8) -> bool;
}
