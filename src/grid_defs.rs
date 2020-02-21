use crate::types::*;

pub const MAX_MCO: usize = 6;
pub type schar_T = [libc::c_uchar; (MAX_MCO + 1) * 4 + 1];
pub type sattr_T = i16;

#[allow(non_snake_case)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenGrid {
    pub handle: handle_T,
    pub chars: *mut schar_T,
    pub attrs: *mut sattr_T,
    pub line_offset: *mut libc::c_uint,
    pub line_wraps: *mut libc::c_uchar,
    pub dirty_col: *mut libc::c_int,
    pub Rows: libc::c_int,
    pub Columns: libc::c_int,
    pub valid: bool,
    pub throttled: bool,
    pub row_offset: libc::c_int,
    pub col_offset: libc::c_int,
    pub blending: bool,
    pub focusable: bool,
    pub comp_row: libc::c_int,
    pub comp_col: libc::c_int,
    pub comp_index: libc::size_t,
    pub comp_disabled: bool,
}
