use std::collections::HashMap;
use std::ffi::CString;

mod libintl {
    extern "C" {
        pub fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
pub unsafe fn c_gettext(msgid: *const libc::c_char) -> *mut libc::c_char {
    libintl::gettext(msgid)
}

static mut static_cache: Option<HashMap<String, *const u8>> = None;

pub unsafe fn gettext<T>(msgid: &str) -> *const T {
    if static_cache.is_none() {
        static_cache = Some(HashMap::new());
    }
    let cache = static_cache.as_mut().unwrap();
    if let Some(result) = cache.get(msgid) {
        return *result as *const T;
    }

    let cstr_msgid = CString::new(msgid).expect("CString::new failed");

    // leak the CString because gettext may just return this pointer
    let cstr_msgid_ptr = cstr_msgid.into_raw();

    let result = libintl::gettext(cstr_msgid_ptr) as *const u8;

    if result != cstr_msgid_ptr as *const u8 {
        CString::from_raw(cstr_msgid_ptr); // drop the CString if gettext returned a different pointer
    }
    let already_inserted = cache.insert(msgid.to_string(), result).is_some();
    assert!(!already_inserted);

    result as *const T
}

pub unsafe fn gettext2(msgid: *const u8) -> *const u8 {
    libintl::gettext(msgid as *const i8) as *const u8
}
