pub type _ISFlag = libc::c_uint;
pub const _ISalnum: _ISFlag = 8;
pub const _ISpunct: _ISFlag = 4;
pub const _IScntrl: _ISFlag = 2;
pub const _ISblank: _ISFlag = 1;
pub const _ISgraph: _ISFlag = 32768;
pub const _ISprint: _ISFlag = 16384;
pub const _ISspace: _ISFlag = 8192;
pub const _ISxdigit: _ISFlag = 4096;
pub const _ISdigit: _ISFlag = 2048;
pub const _ISalpha: _ISFlag = 1024;
pub const _ISlower: _ISFlag = 512;
pub const _ISupper: _ISFlag = 256;
extern "C" {
    pub fn toupper(_: libc::c_int) -> libc::c_int;
    pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
