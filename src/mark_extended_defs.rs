use crate::lib::kbtree::*;
use crate::mark_extended::*;
use crate::pos::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Extmark {
    pub ns_id: u64,
    pub mark_id: u64,
    pub line: *mut ExtmarkLine,
    pub col: colnr_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtmarkLine {
    pub lnum: linenr_T,
    pub items: kbtree_markitems_t,
}
pub type ExtmarkUndoObject = undo_object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct undo_object {
    pub type_0: UndoObjectType,
    pub data: C2RustUnnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_20 {
    pub adjust: Adjust,
    pub col_adjust: ColAdjust,
    pub col_adjust_delete: ColAdjustDelete,
    pub move_0: AdjustMove,
    pub set: ExtmarkSet,
    pub update: ExtmarkUpdate,
    pub copy: ExtmarkCopy,
    pub copy_place: ExtmarkCopyPlace,
    pub clear: ExtmarkClear,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extmark_undo_vec_t {
    pub size: libc::size_t,
    pub capacity: libc::size_t,
    pub items: *mut ExtmarkUndoObject,
}
