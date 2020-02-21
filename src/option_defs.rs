use crate::eval::typval::sctx_T;
extern "C" {
    pub static mut breakat_flags: [libc::c_char; 256];
    pub static mut dy_flags: libc::c_uint;
    pub static mut p_isf: *mut libc::c_uchar;
    pub static mut p_isi: *mut libc::c_uchar;
    pub static mut p_isp: *mut libc::c_uchar;
    pub static mut p_sel: *mut libc::c_uchar;
    pub static mut p_sbr: *mut libc::c_uchar;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LastSet {
    pub script_ctx: sctx_T,
    pub channel_id: u64,
}
// end-of-line style
pub const EOL_UNKNOWN: i32 = -1; // not defined yet
pub const EOL_UNIX: i32 = 0; // NL
pub const EOL_DOS: i32 = 1; // CR NL
pub const EOL_MAC: i32 = 2; // CR

pub const DY_LASTLINE: u32 = 0x001;
pub const DY_TRUNCATE: u32 = 0x002;
pub const DY_UHEX: u32 = 0x004;
