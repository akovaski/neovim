use crate::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct msglist {
    pub msg: *mut u8,
    pub throw_msg: *mut u8,
    pub next: *mut msglist,
}
/* next of several messages in a row */
// The exception types.
pub type except_type_T = libc::c_uint;
// interrupt exception triggered by Ctrl-C
// error exception
pub const ET_INTERRUPT: except_type_T = 2;
// exception caused by ":throw" command
pub const ET_ERROR: except_type_T = 1;
pub const ET_USER: except_type_T = 0;
/*
 * Structure describing an exception.
 * (don't use "struct exception", it's used by the math library).
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vim_exception {
    pub type_0: except_type_T,
    pub value: *mut u8,
    pub messages: *mut msglist,
    pub throw_name: *mut u8,
    pub throw_lnum: linenr_T,
    pub caught: *mut except_T,
}
pub type except_T = vim_exception;
// next exception on the caught stack
/*
 * Structure to save the error/interrupt/exception state between calls to
 * enter_cleanup() and leave_cleanup().  Must be allocated as an automatic
 * variable by the (common) caller of these functions.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cleanup_stuff {
    pub pending: libc::c_int,
    pub exception: *mut except_T,
}
impl Default for cleanup_stuff {
    fn default() -> Self {
        cleanup_T {
            pending: 0,
            exception: ptr::null_mut(),
        }
    }
}
pub type cleanup_T = cleanup_stuff;

extern "C" {
    pub fn aborting() -> libc::c_int;
    pub fn enter_cleanup(csp: *mut cleanup_T);
    pub fn leave_cleanup(csp: *mut cleanup_T);
}
