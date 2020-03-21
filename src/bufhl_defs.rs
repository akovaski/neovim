use crate::extmark_defs::*;
use crate::pos::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct kbpos_bufhl_t {
    pub x: *mut kbnode_bufhl_t,
    pub i: libc::c_int,
}
pub type kbnode_bufhl_t = kbnode_bufhl_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kbnode_bufhl_s {
    pub n: i32,
    pub is_internal: bool,
    pub key: [*mut BufhlLine; 19],
    pub ptr: [*mut kbnode_bufhl_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufhlLine {
    pub line: linenr_T,
    pub items: C2RustUnnamed_10,
    pub virt_text_src: libc::c_int,
    pub virt_text: VirtText,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufhlItem {
    pub src_id: libc::c_int,
    pub hl_id: libc::c_int,
    pub start: colnr_T,
    pub stop: colnr_T,
}

#[allow(dead_code)]
pub type kbtree_bufhl_t = BufhlInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufhlInfo {
    pub root: *mut kbnode_bufhl_t,
    pub n_keys: libc::c_int,
    pub n_nodes: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub size: libc::size_t,
    pub capacity: libc::size_t,
    pub items: *mut BufhlItem,
}
