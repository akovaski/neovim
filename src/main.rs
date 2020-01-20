#![feature(extern_types)]
#![feature(c_variadic)]
#![feature(ptr_offset_from)]
#![allow(non_camel_case_types, non_snake_case)]

use libc::{c_char, c_int};
use std::ffi::CString;

#[macro_use]
mod c_helpers;
#[macro_use]
mod log;
#[macro_use]
mod rbuffer;

mod aucmd;
mod auevents;
mod buffer_defs;
mod bufhl_defs;
mod eval;
mod event;
mod fileio;
mod fmark_defs;
mod garray;
mod globals;
mod grid_defs;
mod hashtab;
mod lib;
mod mark_defs;
mod mark_extended;
mod mark_extended_defs;
mod math;
mod memfile_defs;
mod memline_defs;
mod memory;
mod option_defs;
mod os;
mod path;
mod pos;
mod profile;
mod regexp_defs;
mod sign_defs;
mod strings;
mod syntax_defs;
mod terminal;
mod types;
mod ugrid;
mod undo_defs;
mod viml;
mod xdiff;

use crate::event::r#loop::Loop;

extern "C" {
    fn nvim_main(argc: c_int, argv: *const *const c_char) -> c_int;
    static mut main_loop: Loop;
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
