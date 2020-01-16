use crate::eval::typval::*;
use crate::os::time::*;
use crate::pos::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct fmark_T {
    pub mark: pos_T,
    pub fnum: libc::c_int,
    pub timestamp: Timestamp,
    pub additional_data: *mut dict_T,
}
