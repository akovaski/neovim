extern "C" {
    #[doc(hidden)]
    pub fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}

#[macro_export]
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
