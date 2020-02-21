pub type linenr_T = libc::c_long;
pub type colnr_T = libc::c_int;

pub const MAXCOL: libc::c_int = 0x7fffffff;

/// position in file or buffer
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pos_T {
    pub lnum: linenr_T, // line number
    pub col: colnr_T,   // column number
    pub coladd: colnr_T,
}

/// Same, but without coladd.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lpos_T {
    pub lnum: linenr_T, // line number
    pub col: colnr_T,   // column number
}
