use crate::fmark_defs::*;
use crate::mark_extended_defs::*;
use crate::pos::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct visualinfo_T {
    pub vi_start: pos_T,
    pub vi_end: pos_T,
    pub vi_mode: libc::c_int,
    pub vi_curswant: colnr_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct u_header_T {
    pub uh_next: C2RustUnnamed_24,
    pub uh_prev: C2RustUnnamed_23,
    pub uh_alt_next: C2RustUnnamed_22,
    pub uh_alt_prev: C2RustUnnamed_21,
    pub uh_seq: libc::c_long,
    pub uh_walk: libc::c_int,
    pub uh_entry: *mut u_entry_T,
    pub uh_getbot_entry: *mut u_entry_T,
    pub uh_cursor: pos_T,
    pub uh_cursor_vcol: libc::c_long,
    pub uh_flags: libc::c_int,
    pub uh_namedm: [fmark_T; 26],
    pub uh_extmark: extmark_undo_vec_t,
    pub uh_visual: visualinfo_T,
    pub uh_time: libc::time_t,
    pub uh_save_nr: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct u_entry_T {
    pub ue_next: *mut u_entry_T,
    pub ue_top: linenr_T,
    pub ue_bot: linenr_T,
    pub ue_lcount: linenr_T,
    pub ue_array: *mut *mut libc::c_uchar,
    pub ue_size: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
    pub ptr: *mut u_header_T,
    pub seq: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_22 {
    pub ptr: *mut u_header_T,
    pub seq: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_23 {
    pub ptr: *mut u_header_T,
    pub seq: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_24 {
    pub ptr: *mut u_header_T,
    pub seq: libc::c_long,
}
