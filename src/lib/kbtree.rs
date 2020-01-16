use crate::lib::khash::*;
use crate::mark_extended_defs::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Map_uint64_t_ptr_t {
    pub table: *mut kh_uint64_t_ptr_t_map_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kh_uint64_t_ptr_t_map_t {
    pub n_buckets: khint_t,
    pub size: khint_t,
    pub n_occupied: khint_t,
    pub upper_bound: khint_t,
    pub flags: *mut khint_t,
    pub keys: *mut u64,
    pub vals: *mut ptr_t,
}
pub type ptr_t = *mut libc::c_void;
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
