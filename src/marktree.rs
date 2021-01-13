use crate::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MarkTree {
    pub root: *mut mtnode_t,
    pub n_keys: size_t,
    pub n_nodes: size_t,
    pub next_id: u64,
    pub id2node: *mut Map_uint64_t_ptr_t,
}
pub type mtnode_t = mtnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtnode_s {
    pub n: i32,
    pub level: i32,
    pub parent: *mut mtnode_t,
    pub key: [mtkey_t; 19],
    pub ptr: [*mut mtnode_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtkey_t {
    pub pos: mtpos_t,
    pub id: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtpos_t {
    pub row: i32,
    pub col: i32,
}
