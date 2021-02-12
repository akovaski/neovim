use crate::*;

extern "C" {
    pub fn getviscol() -> i32;
    pub fn getviscol2(mut col: colnr_T, mut coladd: colnr_T) -> i32;
    pub fn coladvance_force(mut wcol: colnr_T) -> i32;
    pub fn coladvance(mut wcol: colnr_T) -> i32;
    pub fn getvpos(mut pos: *mut pos_T, mut wcol: colnr_T) -> i32;
    pub fn inc_cursor() -> i32;
    pub fn dec_cursor() -> i32;
    pub fn get_cursor_rel_lnum(mut wp: *mut win_T, mut lnum: linenr_T) -> linenr_T;
    pub fn check_pos(mut buf: *mut buf_T, mut pos: *mut pos_T);
    pub fn check_cursor_lnum();
    pub fn check_cursor_col();
    pub fn check_cursor_col_win(mut win: *mut win_T);
    pub fn check_cursor();
    pub fn adjust_cursor_col();
    pub fn leftcol_changed() -> bool;
    pub fn gchar_cursor() -> i32;
    pub fn pchar_cursor(mut c: u8);
    pub fn get_cursor_line_ptr() -> *mut u8;
    pub fn get_cursor_pos_ptr() -> *mut u8;
}
