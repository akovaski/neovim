use crate::*;

extern "C" {
    pub fn status_redraw_all();
    pub fn get_keymap_str(
        wp: *mut win_T,
        fmt: *const u8,
        buf: *mut u8,
        len: libc::c_int,
    ) -> libc::c_int;
    pub fn redrawing() -> libc::c_int;
    pub fn redraw_buf_later(buf: *mut buf_T, type_0: libc::c_int);
    pub fn redraw_later(type_0: libc::c_int);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StlClickDefinition {
    pub type_0: StlClickDefinitionType,
    pub tabnr: libc::c_int,
    pub func: *mut libc::c_char,
}
pub type StlClickDefinitionType = libc::c_uint;
pub const kStlClickFuncRun: StlClickDefinitionType = 3;
pub const kStlClickTabClose: StlClickDefinitionType = 2;
pub const kStlClickTabSwitch: StlClickDefinitionType = 1;
pub const kStlClickDisabled: StlClickDefinitionType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StlClickRecord {
    pub def: StlClickDefinition,
    pub start: *const libc::c_char,
}
pub const NOT_VALID: libc::c_int = 40 as libc::c_int;
