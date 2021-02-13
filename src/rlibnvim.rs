#![feature(extern_types)]
#![feature(c_variadic)]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![feature(concat_idents)]
#![feature(label_break_value)]

use libc::{c_char, c_int};
pub use std::cmp::min;
pub use std::ptr;

#[macro_use]
mod macros;
pub use macros::*;
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
#[macro_use]
mod globals;
pub use globals::*;

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
mod cursor;
pub use cursor::*;
mod eval;
pub use eval::*;
mod extmark_defs;
pub use extmark_defs::*;
mod fileio;
pub use fileio::*;
mod garray;
pub use garray::*;
mod gettext;
pub use gettext::*;
mod grid_defs;
pub use grid_defs::*;
mod hashtab;
pub use hashtab::*;
mod indent;
pub use indent::*;
mod keymap;
pub use keymap::*;
mod map;
pub use map::*;
mod map_defs;
pub use map_defs::*;
mod mark;
pub use mark::*;
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
mod memline;
pub use memline::*;
mod memline_defs;
pub use memline_defs::*;
mod memory;
pub use memory::*;
mod misc1;
pub use misc1::*;
mod r#move;
pub use r#move::*;
mod option;
pub use option::*;
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
mod state;
pub use state::*;
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
mod vim;
pub use vim::*;
mod viml;
pub use viml::*;
mod buffer;
pub use buffer::*;
mod stdbool;
pub use stdbool::*;
mod change;
pub use change::*;
mod message;
pub use message::*;
mod stat;
pub use stat::*;
mod ex_cmds;
pub use ex_cmds::*;
mod ex_eval;
pub use ex_eval::*;
mod ex_cmds_enum;
pub use ex_cmds_enum::*;
mod ex_cmds_defs;
pub use ex_cmds_defs::*;
mod fold;
pub use fold::*;
mod window;
pub use window::*;
mod diff;
pub use diff::*;
mod normal;
pub use normal::*;
mod buffer_updates;
pub use buffer_updates::*;
mod syntax;
pub use syntax::*;
mod undo;
pub use undo::*;
mod handle;
pub use handle::*;
mod ex_docmd;
pub use ex_docmd::*;
mod sign;
pub use sign::*;
mod extmark;
pub use extmark::*;
mod getchar;
pub use getchar::*;
mod ex_cmds2;
pub use ex_cmds2::*;
mod spell;
pub use spell::*;
mod digraph;
pub use digraph::*;
mod ui;
pub use ui::*;
mod screen;
pub use screen::*;
mod file_search;
pub use file_search::*;
mod regexp;
pub use regexp::*;
mod ex_getln;
pub use ex_getln::*;
mod channel;
pub use channel::*;
mod api;
pub use api::*;
mod ui_events_call;
pub use ui_events_call::*;
mod helpers;
pub use helpers::*;
mod stdlib;
pub use stdlib::*;
mod ctype;
pub use ctype::*;
mod version;
pub use version::*;
mod marktree;
pub use marktree::*;
mod arabic;
pub use arabic::*;

pub use libc::{backtrace, fclose, fopen, iovec, ptrdiff_t, size_t, ssize_t, time_t, timeval, tm, FILE};
extern "C" {
    // main.h
    pub fn getout(exitval: libc::c_int) -> !;
}
extern "C" {
    pub fn nvim_main(argc: c_int, argv: *const *const c_char) -> c_int;
    pub static mut main_loop: Loop;
}

pub const MIN_LOG_LEVEL: i32 = 1; // build variable 1(INFO) - 3(DEBUG)
