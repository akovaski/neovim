extern "C" {
    #[doc(hidden)]
    pub fn logmsg(
        log_level: libc::c_int,
        context: *const libc::c_char,
        func_name: *const libc::c_char,
        line_num: libc::c_int,
        eol: bool,
        fmt: *const libc::c_char,
        _: ...
    ) -> bool;
}

#[allow(dead_code)]
pub const DEBUG_LOG_LEVEL: libc::c_int = 0;
#[allow(dead_code)]
pub const INFO_LOG_LEVEL: libc::c_int = 1;
pub const WARN_LOG_LEVEL: libc::c_int = 2;
#[allow(dead_code)]
pub const ERROR_LOG_LEVEL: libc::c_int = 3;

macro_rules! WLOG {
    ($s:expr ,$($x:expr),+) => {
        logmsg(WARN_LOG_LEVEL,
               std::ptr::null(),
               std::ffi::CString::new(file!())
                   .expect("CString::new failed")
                   .as_ptr() as *const libc::c_char,
                line!() as libc::c_int,
                true,
               std::ffi::CString::new($s)
                   .expect("CString::new failed")
                   .as_ptr() as *const libc::c_char,
                $($x),+
                );
    }
}
