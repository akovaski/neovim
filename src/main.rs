use libc::{c_char, c_int};
use std::ffi::CString;

extern "C" {
    fn nvim_main(argc: c_int, argv: *const *const c_char) -> c_int;
}

fn main() {
    // create a vector of zero terminated strings
    let args = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect::<Vec<CString>>();
    // convert the strings to raw pointers
    let c_args = args
        .iter()
        .map(|arg| arg.as_ptr())
        .collect::<Vec<*const c_char>>();
    unsafe {
        nvim_main(c_args.len() as c_int, c_args.as_ptr());
    };
}
