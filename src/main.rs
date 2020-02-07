#![feature(extern_types)]
#![feature(c_variadic)]
#![feature(ptr_offset_from)]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![feature(concat_idents)]

use libc::{c_char, c_int};
use std::ffi::CString;

#[macro_use]
mod c_helpers;
pub use c_helpers::*;
#[macro_use]
mod log;
pub use log::*;
#[macro_use]
mod rbuffer;
pub use rbuffer::*;
#[macro_use]
mod lib;
pub use lib::*;
#[macro_use]
mod event;
pub use event::*;

mod ascii;
pub use ascii::*;
mod aucmd;
pub use aucmd::*;
mod auevents;
pub use auevents::*;
mod buffer_defs;
pub use buffer_defs::*;
mod bufhl_defs;
pub use bufhl_defs::*;
mod charset;
pub use charset::*;
mod eval;
pub use eval::*;
mod fileio;
pub use fileio::*;
mod fmark_defs;
pub use fmark_defs::*;
mod garray;
pub use garray::*;
mod gettext;
pub use gettext::*;
mod globals;
pub use globals::*;
mod grid_defs;
pub use grid_defs::*;
mod hashtab;
pub use hashtab::*;
mod keymap;
pub use keymap::*;
mod mark_defs;
pub use mark_defs::*;
mod mark_extended;
pub use mark_extended::*;
mod mark_extended_defs;
pub use mark_extended_defs::*;
mod math;
pub use math::*;
mod mbyte;
pub use mbyte::*;
mod memfile_defs;
pub use memfile_defs::*;
mod memline_defs;
pub use memline_defs::*;
mod memory;
pub use memory::*;
mod misc1;
pub use misc1::*;
mod option_defs;
pub use option_defs::*;
mod os;
pub use os::*;
mod path;
pub use path::*;
mod pos;
pub use pos::*;
mod profile;
pub use profile::*;
mod regexp_defs;
pub use regexp_defs::*;
mod sign_defs;
pub use sign_defs::*;
mod strings;
pub use strings::*;
mod syntax_defs;
pub use syntax_defs::*;
mod terminal;
pub use terminal::*;
mod types;
pub use types::*;
mod ugrid;
pub use ugrid::*;
mod undo_defs;
pub use undo_defs::*;
mod viml;
pub use viml::*;

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
