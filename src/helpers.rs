use crate::*;

extern "C" {
    pub fn cstr_as_string(str: *mut libc::c_char) -> String_0;
}
