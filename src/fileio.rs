use crate::auevents::*;
use crate::buffer_defs::*;

extern "C" {
    pub fn apply_autocmds(
        event: event_T,
        fname: *const libc::c_uchar,
        fname_io: *const libc::c_uchar,
        force: bool,
        buf: *mut buf_T,
    ) -> bool;
}
