use crate::*;
extern "C" {
    pub fn win_col_off(wp: *mut win_T) -> libc::c_int;
    pub fn win_col_off2(wp: *mut win_T) -> libc::c_int;
    pub fn validate_virtcol();
    pub fn scroll_cursor_halfway(atend: libc::c_int);
}
