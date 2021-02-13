use crate::*;

extern "C" {
    pub fn getviscol() -> i32;
    pub fn getviscol2(col: colnr_T, coladd: colnr_T) -> i32;
    pub fn coladvance_force(wcol: colnr_T) -> i32;
    pub fn coladvance(wcol: colnr_T) -> i32;
    pub fn getvpos(pos: *mut pos_T, wcol: colnr_T) -> i32;
    pub fn inc_cursor() -> i32;
    pub fn dec_cursor() -> i32;
    pub fn get_cursor_rel_lnum(wp: *mut win_T, lnum: linenr_T) -> linenr_T;
    pub fn check_pos(buf: *mut buf_T, pos: *mut pos_T);
    pub fn check_cursor_lnum();
    pub fn check_cursor_col();
    pub fn check_cursor_col_win(win: *mut win_T);
    pub fn check_cursor();
    pub fn adjust_cursor_col();
    pub fn leftcol_changed() -> bool;
    pub fn gchar_cursor() -> i32;
    pub fn pchar_cursor(c: u8);
    pub fn get_cursor_line_ptr() -> *mut u8;
    pub fn get_cursor_pos_ptr() -> *mut u8;
}
