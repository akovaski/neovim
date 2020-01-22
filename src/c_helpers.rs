extern "C" {
    #[doc(hidden)]
    pub fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    pub fn abort() -> !;
}

macro_rules! vargs {
    ( $($arg:expr),* $(,)?) => {
        &[ $($arg as *mut libc::c_void),* ];
    }
}

macro_rules! c_assert {
    ( $x:expr ) => {
        if !$x {
            use crate::c_helpers::__assert_fail;
            __assert_fail(
                std::ffi::CString::new(stringify!($x))
                    .expect("CString::new failed")
                    .as_ptr(),
                std::ffi::CString::new(file!())
                    .expect("CString::new failed")
                    .as_ptr(),
                line!(),
                std::ffi::CString::new("(a rust function)")
                    .expect("CString::new failed")
                    .as_ptr(),
            );
        }
    };
}

#[allow(dead_code)]
unsafe extern "C" fn xdl_diff(
    mf1: *mut xdiff::mmfile_t,
    mf2: *mut xdiff::mmfile_t,
    xpp: *const xdiff::xpparam_t,
    xecfg: *const xdiff::xdemitconf_t,
    ecb: *mut xdiff::xdemitcb_t,
) -> libc::c_int {
    xdiff::xdl_diff(mf1, mf2, xpp, xecfg, ecb)
}
