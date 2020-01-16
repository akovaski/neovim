use crate::memfile_defs::*;
use crate::pos::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct memline_T {
    pub ml_line_count: linenr_T,
    pub ml_mfp: *mut memfile_T,
    pub ml_flags: libc::c_int,
    pub ml_stack: *mut infoptr_T,
    pub ml_stack_top: libc::c_int,
    pub ml_stack_size: libc::c_int,
    pub ml_line_lnum: linenr_T,
    pub ml_line_ptr: *mut libc::c_uchar,
    pub ml_locked: *mut bhdr_T,
    pub ml_locked_low: linenr_T,
    pub ml_locked_high: linenr_T,
    pub ml_locked_lineadd: libc::c_int,
    pub ml_chunksize: *mut chunksize_T,
    pub ml_numchunks: libc::c_int,
    pub ml_usedchunks: libc::c_int,
}
