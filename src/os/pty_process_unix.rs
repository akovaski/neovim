use crate::*;

extern "C" {
    pub fn pty_process_spawn(ptyproc: *mut PtyProcess) -> libc::c_int;
    pub fn pty_process_close(ptyproc: *mut PtyProcess);
    pub fn pty_process_close_master(ptyproc: *mut PtyProcess);
    pub fn pty_process_teardown(loop_0: *mut Loop);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PtyProcess {
    pub process: Process,
    pub term_name: *mut libc::c_char,
    pub width: u16,
    pub height: u16,
    pub winsize: winsize,
    pub tty_fd: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
