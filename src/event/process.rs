use crate::*;

pub type internal_process_cb = Option<unsafe extern "C" fn(_: *mut Process) -> ()>;
pub type Process = process;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct process {
    pub type_0: ProcessType,
    pub loop_0: *mut Loop,
    pub data: *mut libc::c_void,
    pub pid: libc::c_int,
    pub status: libc::c_int,
    pub refcount: libc::c_int,
    pub exit_signal: u8,
    pub stopped_time: u64,
    pub cwd: *const libc::c_char,
    pub argv: *mut *mut libc::c_char,
    pub env: *mut *mut libc::c_char,
    pub in_0: Stream,
    pub out: Stream,
    pub err: Stream,
    pub cb: process_exit_cb,
    pub internal_exit_cb: internal_process_cb,
    pub internal_close_cb: internal_process_cb,
    pub closed: bool,
    pub detach: bool,
    pub events: *mut MultiQueue,
}
pub type process_exit_cb =
    Option<unsafe extern "C" fn(_: *mut Process, _: libc::c_int, _: *mut libc::c_void) -> ()>;

pub type ProcessType = libc::c_uint;
pub const kProcessTypePty: ProcessType = 1;
pub const kProcessTypeUv: ProcessType = 0;
