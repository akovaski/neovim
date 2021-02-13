#![allow(dead_code)]
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
    static mut namedfm: [xfmark_T; 36];
    pub fn setmark(c: i32) -> i32;
    pub fn free_fmark(fm: fmark_T);
    pub fn free_xfmark(fm: xfmark_T);
    pub fn clear_fmark(fm: *mut fmark_T);
    pub fn setmark_pos(c: i32, pos: *mut pos_T, fnum: i32) -> i32;
    pub fn setpcmark();
    pub fn checkpcmark();
    pub fn movemark(count: i32) -> *mut pos_T;
    pub fn movechangelist(count: i32) -> *mut pos_T;
    pub fn getmark_buf(buf: *mut buf_T, c: i32, changefile: bool) -> *mut pos_T;
    pub fn getmark(c: i32, changefile: bool) -> *mut pos_T;
    pub fn getmark_buf_fnum(
        buf: *mut buf_T,
        c: i32,
        changefile: bool,
        fnum: *mut i32,
    ) -> *mut pos_T;
    pub fn getnextmark(startpos: *mut pos_T, dir: i32, begin_line: i32) -> *mut pos_T;
    pub fn fmarks_check_names(buf: *mut buf_T);
    pub fn check_mark(pos: *mut pos_T) -> i32;
    pub fn clrallmarks(buf: *mut buf_T);
    pub fn fm_getname(fmark: *mut fmark_T, lead_len: i32) -> *mut u8;
    pub fn ex_marks(eap: *mut exarg_T);
    pub fn ex_delmarks(eap: *mut exarg_T);
    pub fn ex_jumps(eap: *mut exarg_T);
    pub fn ex_clearjumps(eap: *mut exarg_T);
    pub fn ex_changes(eap: *mut exarg_T);
    pub fn mark_adjust(
        line1: linenr_T,
        line2: linenr_T,
        amount: i64,
        amount_after: i64,
        op: ExtmarkOp,
    );
    pub fn mark_adjust_nofold(
        line1: linenr_T,
        line2: linenr_T,
        amount: i64,
        amount_after: i64,
        op: ExtmarkOp,
    );
    pub fn mark_col_adjust(
        lnum: linenr_T,
        mincol: colnr_T,
        lnum_amount: i64,
        col_amount: i64,
        spaces_removed: i32,
    );
    pub fn cleanup_jumplist(wp: *mut win_T, checktail: bool);
    pub fn copy_jumplist(from: *mut win_T, to: *mut win_T);
    pub fn mark_jumplist_iter(
        iter: *const libc::c_void,
        win: *const win_T,
        fm: *mut xfmark_T,
    ) -> *const libc::c_void;
    pub fn mark_global_iter(
        iter: *const libc::c_void,
        name: *mut i8,
        fm: *mut xfmark_T,
    ) -> *const libc::c_void;
    pub fn mark_buffer_iter(
        iter: *const libc::c_void,
        buf: *const buf_T,
        name: *mut i8,
        fm: *mut fmark_T,
    ) -> *const libc::c_void;
    pub fn mark_set_global(name: i8, fm: xfmark_T, update: bool) -> bool;
    pub fn mark_set_local(name: i8, buf: *mut buf_T, fm: fmark_T, update: bool) -> bool;
    pub fn free_jumplist(wp: *mut win_T);
    pub fn set_last_cursor(win: *mut win_T);
    pub fn mark_mb_adjustpos(buf: *mut buf_T, lp: *mut pos_T);
}
