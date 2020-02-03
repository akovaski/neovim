use std::ffi::CString;

mod ffi {
    extern "C" {
        pub fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}

pub unsafe fn gettext(msgid: &str) -> *mut libc::c_char {
    ffi::gettext(CString::new(msgid).expect("CString::new failed").as_ptr())
}
