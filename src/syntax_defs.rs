use crate::buffer_defs::*;
use crate::garray::*;
use crate::pos::*;
use crate::regexp_defs::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct synstate_T {
    pub sst_next: *mut synstate_T,
    pub sst_lnum: linenr_T,
    pub sst_union: synstate_sst,
    pub sst_next_flags: libc::c_int,
    pub sst_stacksize: libc::c_int,
    pub sst_next_list: *mut i16,
    pub sst_tick: disptick_T,
    pub sst_change_lnum: linenr_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union synstate_sst {
    pub sst_stack: [bufstate_T; 7],
    pub sst_ga: garray_T,
}
pub type bufstate_T = buf_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf_state {
    pub bs_idx: libc::c_int,
    pub bs_flags: libc::c_int,
    pub bs_seqnr: libc::c_int,
    pub bs_cchar: libc::c_int,
    pub bs_extmatch: *mut reg_extmatch_T,
}
