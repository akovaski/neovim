use crate::*;

extern "C" {
    pub fn u_sync(force: libc::c_int);
    pub fn u_clearall(buf: *mut buf_T);
    pub fn u_blockfree(buf: *mut buf_T);
    pub fn bufIsChanged(buf: *mut buf_T) -> bool;
    pub fn curbufIsChanged() -> bool;
}
