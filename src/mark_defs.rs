use crate::fmark_defs::fmark_T;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xfmark_T {
    pub fmark: fmark_T,
    pub fname: *mut libc::c_uchar,
}
