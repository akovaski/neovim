use crate::*;
// read from fifo or socket
/*
 * Struct to save values in before executing autocommands for a buffer that is
 * not the current buffer.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aco_save_T {
    pub save_curbuf: *mut buf_T,
    pub use_aucmd_win: libc::c_int,
    pub save_curwin: *mut win_T,
    pub save_prevwin: *mut win_T,
    pub new_curwin: *mut win_T,
    pub new_curbuf: bufref_T,
    pub globaldir: *mut u8,
}
impl Default for aco_save_T {
    fn default() -> Self {
        aco_save_T {
            save_curbuf: ptr::null_mut(),
            use_aucmd_win: 0,
            save_curwin: ptr::null_mut(),
            save_prevwin: ptr::null_mut(),
            new_curwin: ptr::null_mut(),
            new_curbuf: bufref_T::default(),
            globaldir: ptr::null_mut(),
        }
    }
}
// Values for readfile() flags
// read a file into a new buffer
// read filter output
// read from stdin
// read from curbuf (converting stdin)
pub const READ_DUMMY: libc::c_int = 0x10 as libc::c_int;
// reading into a dummy buffer
// keep undo info
pub const READ_FIFO: libc::c_int = 0x40 as libc::c_int;
pub const READ_NEW: libc::c_int = 0x1 as libc::c_int;
pub const READ_STDIN: libc::c_int = 0x4 as libc::c_int;
pub const READ_BUFFER: libc::c_int = 0x8 as libc::c_int;

extern "C" {
    pub fn apply_autocmds(
        event: event_T,
        fname: *const libc::c_uchar,
        fname_io: *const libc::c_uchar,
        force: bool,
        buf: *mut buf_T,
    ) -> bool;
    pub fn prep_exarg(eap: *mut exarg_T, buf: *const buf_T);
    pub fn shorten_fnames(force: libc::c_int);
    pub fn readfile(
        fname: *mut u8,
        sfname: *mut u8,
        from: linenr_T,
        lines_to_skip: linenr_T,
        lines_to_read: linenr_T,
        eap: *mut exarg_T,
        flags: libc::c_int,
    ) -> libc::c_int;
    pub fn aucmd_prepbuf(aco: *mut aco_save_T, buf: *mut buf_T);
    pub fn apply_autocmds_retval(
        event: event_T,
        fname: *mut u8,
        fname_io: *mut u8,
        force: bool,
        buf: *mut buf_T,
        retval: *mut libc::c_int,
    ) -> bool;
    pub fn aucmd_restbuf(aco: *mut aco_save_T);
    pub fn block_autocmds();
    pub fn unblock_autocmds();
    pub fn aubuflocal_remove(buf: *mut buf_T);
    pub fn buf_check_timestamp(buf: *mut buf_T, focus: libc::c_int) -> libc::c_int;
    pub fn file_pat_to_reg_pat(
        pat: *const u8,
        pat_end: *const u8,
        allow_dirs: *mut libc::c_char,
        no_bslash: libc::c_int,
    ) -> *mut u8;
}
