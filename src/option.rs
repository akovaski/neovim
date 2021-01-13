use crate::*;
extern "C" {
    pub fn skip_to_option_part(p: *const libc::c_uchar) -> *mut libc::c_uchar;
    pub fn do_set(arg: *mut u8, opt_flags: libc::c_int) -> libc::c_int;
    pub fn clear_string_option(pp: *mut *mut u8);
    pub fn was_set_insecurely(opt: *const u8, opt_flags: libc::c_int) -> libc::c_int;
    pub fn set_string_option_direct(
        name: *const u8,
        opt_idx: libc::c_int,
        val: *const u8,
        opt_flags: libc::c_int,
        set_sid: libc::c_int,
    );
    pub fn check_colorcolumn(wp: *mut win_T) -> *mut u8;
    pub fn set_option_value(
        name: *const libc::c_char,
        number: libc::c_long,
        string: *const libc::c_char,
        opt_flags: libc::c_int,
    ) -> *mut libc::c_char;
    pub fn copy_winopt(from: *mut winopt_T, to: *mut winopt_T);
    pub fn clear_winopt(wop: *mut winopt_T);
    pub fn didset_window_options(wp: *mut win_T);
    pub fn buf_copy_options(buf: *mut buf_T, flags: libc::c_int);
    pub fn shortmess(x: libc::c_int) -> bool;
    pub fn save_file_ff(buf: *mut buf_T);
    pub fn get_fileformat(buf: *mut buf_T) -> libc::c_int;
}
pub type OptionFlags = i32;
pub const OPT_LOCAL: OptionFlags = 4;
pub const OPT_MODELINE: OptionFlags = 8;
pub const OPT_FREE: OptionFlags = 1;
pub const OPT_NOWIN: OptionFlags = 32;
pub const OPT_WINONLY: OptionFlags = 16;
pub const OPT_GLOBAL: OptionFlags = 2;
pub const BCO_ALWAYS: libc::c_int = 2 as libc::c_int;
pub const BCO_ENTER: libc::c_int = 1 as libc::c_int;
pub const BCO_NOHELP: libc::c_int = 4 as libc::c_int;
