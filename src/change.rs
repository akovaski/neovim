use crate::*;

extern "C" {
    pub fn changed();
    pub fn deleted_lines_mark(lnum: linenr_T, count: libc::c_long);
    pub fn unchanged(buf: *mut buf_T, ff: libc::c_int, always_inc_changedtick: bool);
}
