use crate::pos::linenr_T;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct signlist_T {
    pub id: libc::c_int,
    pub lnum: linenr_T,
    pub typenr: libc::c_int,
    pub group: *mut signgroup_T,
    pub priority: libc::c_int,
    pub next: *mut signlist_T,
    pub prev: *mut signlist_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct signgroup_T {
    pub refcount: u16,
    pub next_sign_id: libc::c_int,
    pub sg_name: [libc::c_uchar; 1],
}
