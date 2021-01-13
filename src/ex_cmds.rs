use crate::*;

extern "C" {
    pub fn getfile(
        fnum: libc::c_int,
        ffname: *mut u8,
        sfname: *mut u8,
        setpm: libc::c_int,
        lnum: linenr_T,
        forceit: libc::c_int,
    ) -> libc::c_int;
    pub fn do_ecmd(
        fnum: libc::c_int,
        ffname: *mut u8,
        sfname: *mut u8,
        eap: *mut exarg_T,
        newlnum: linenr_T,
        flags: libc::c_int,
        oldwin: *mut win_T,
    ) -> libc::c_int;
    pub fn fix_help_buffer();
}
// flags for do_ecmd()
pub const ECMD_HIDE: libc::c_int = 0x1 as libc::c_int;
// don't free the current buffer
// set b_help flag of (new) buffer before
// opening file
pub const ECMD_OLDBUF: libc::c_int = 0x4 as libc::c_int;
// use existing buffer if it exists
pub const ECMD_FORCEIT: libc::c_int = 0x8 as libc::c_int;
// ! used in Ex command
// don't edit, just add to buffer list
/* for lnum argument in do_ecmd() */
/* use last position in loaded file */
/* use last position in all files */
pub const ECMD_ONE: linenr_T = 1;
