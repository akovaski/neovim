use crate::eval::typval::*;
use crate::os::time::*;
use crate::pos::*;

pub const NMARKS: usize = 'z' as usize - 'a' as usize + 1;
pub const JUMPLISTSIZE: usize = 100;

pub type fmark_T = filemark;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filemark {
    pub mark: pos_T,
    pub fnum: libc::c_int,
    pub timestamp: Timestamp,
    pub additional_data: *mut dict_T,
}
// /< Cursor position.
// /< File number.
// /< Time when this mark was last set.
// /< Additional data from ShaDa file.
// / Structure defining extended mark (mark with file name attached)
pub type xfmark_T = xfilemark;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xfilemark {
    pub fmark: fmark_T,
    pub fname: *mut u8,
}
