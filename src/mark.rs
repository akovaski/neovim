use crate::*;

#[inline(always)]
pub unsafe extern "C" fn equalpos(a: pos_T, b: pos_T) -> bool {
    return a.lnum == b.lnum && a.col == b.col && a.coladd == b.coladd;
}
#[inline(always)]
pub unsafe extern "C" fn lt(a: pos_T, b: pos_T) -> bool {
    if a.lnum != b.lnum {
        return a.lnum < b.lnum;
    } else if a.col != b.col {
        return a.col < b.col;
    } else {
        return a.coladd < b.coladd;
    };
}
// / Return true if position a is less than or equal to b.
#[inline(always)]
pub unsafe extern "C" fn ltoreq(a: pos_T, b: pos_T) -> bool {
    return lt(a, b) as libc::c_int != 0 || equalpos(a, b) as libc::c_int != 0;
}
extern "C" {
    pub fn free_fmark(fm: fmark_T);
    pub fn clear_fmark(fm: *mut fmark_T);
    pub fn setpcmark();
    pub fn fmarks_check_names(buf: *mut buf_T);
    pub fn clrallmarks(buf: *mut buf_T);
    pub fn set_last_cursor(win: *mut win_T);
}
