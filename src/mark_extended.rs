use crate::pos::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtmarkClear {
    pub ns_id: u64,
    pub l_lnum: linenr_T,
    pub u_lnum: linenr_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtmarkCopyPlace {
    pub l_lnum: linenr_T,
    pub l_col: colnr_T,
    pub u_lnum: linenr_T,
    pub u_col: colnr_T,
    pub p_lnum: linenr_T,
    pub p_col: colnr_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtmarkCopy {
    pub ns_id: u64,
    pub mark_id: u64,
    pub lnum: linenr_T,
    pub col: colnr_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtmarkUpdate {
    pub ns_id: u64,
    pub mark_id: u64,
    pub old_lnum: linenr_T,
    pub old_col: colnr_T,
    pub lnum: linenr_T,
    pub col: colnr_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtmarkSet {
    pub ns_id: u64,
    pub mark_id: u64,
    pub lnum: linenr_T,
    pub col: colnr_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AdjustMove {
    pub line1: linenr_T,
    pub line2: linenr_T,
    pub last_line: linenr_T,
    pub dest: linenr_T,
    pub num_lines: linenr_T,
    pub extra: linenr_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColAdjustDelete {
    pub lnum: linenr_T,
    pub mincol: colnr_T,
    pub endcol: colnr_T,
    pub eol: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColAdjust {
    pub lnum: linenr_T,
    pub mincol: colnr_T,
    pub col_amount: libc::c_long,
    pub lnum_amount: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Adjust {
    pub line1: linenr_T,
    pub line2: linenr_T,
    pub amount: libc::c_long,
    pub amount_after: libc::c_long,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
#[repr(C)]
pub enum UndoObjectType {
    kLineAdjust,
    kColAdjust,
    kColAdjustDelete,
    kAdjustMove,
    kExtmarkSet,
    kExtmarkDel,
    kExtmarkUpdate,
    kExtmarkCopy,
    kExtmarkCopyPlace,
    kExtmarkClear,
}
pub type ExtmarkOp = u32;
