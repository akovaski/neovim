use crate::mark_extended_defs::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct kbpos_markitems_t {
    pub x: *mut kbnode_markitems_t,
    pub i: libc::c_int,
}
pub type kbnode_markitems_t = kbnode_markitems_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kbnode_markitems_s {
    pub n: i32,
    pub is_internal: bool,
    pub key: [Extmark; 19],
    pub ptr: [*mut kbnode_markitems_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kbtree_markitems_t {
    pub root: *mut kbnode_markitems_t,
    pub n_keys: libc::c_int,
    pub n_nodes: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kbpos_extmarklines_t {
    pub x: *mut kbnode_extmarklines_t,
    pub i: libc::c_int,
}
pub type kbnode_extmarklines_t = kbnode_extmarklines_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kbnode_extmarklines_s {
    pub n: i32,
    pub is_internal: bool,
    pub key: [*mut ExtmarkLine; 19],
    pub ptr: [*mut kbnode_extmarklines_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kbtree_extmarklines_t {
    pub root: *mut kbnode_extmarklines_t,
    pub n_keys: libc::c_int,
    pub n_nodes: libc::c_int,
}
