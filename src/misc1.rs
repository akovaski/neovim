#![allow(dead_code)]
use crate::*;

pub const BREAKCHECK_SKIP: i32 = 1000;
extern "C" {
    pub fn exit_free() -> bool;
    static mut ga_users: garray_T;
    pub fn get_leader_len(line: *mut u8, flags: *mut *mut u8, backward: i32, include_space: i32) -> i32;
    pub fn get_last_leader_offset(line: *mut u8, flags: *mut *mut u8) -> i32;
    pub fn plines(lnum: linenr_T) -> i32;
    pub fn plines_win(wp: *mut win_T, lnum: linenr_T, winheight: bool) -> i32;
    pub fn plines_nofill(lnum: linenr_T) -> i32;
    pub fn plines_win_nofill(wp: *mut win_T, lnum: linenr_T, winheight: bool) -> i32;
    pub fn plines_win_nofold(wp: *mut win_T, lnum: linenr_T) -> i32;
    pub fn plines_win_col(wp: *mut win_T, lnum: linenr_T, column: i64) -> i32;
    pub fn plines_win_full(wp: *mut win_T, lnum: linenr_T, nextp: *mut linenr_T, foldedp: *mut bool, cache: bool) -> i32;
    pub fn plines_m_win(wp: *mut win_T, first: linenr_T, last: linenr_T) -> i32;
    pub fn gchar_pos(pos: *mut pos_T) -> i32;
    pub fn check_status(buf: *mut buf_T);
    pub fn ask_yesno(str: *const i8, direct: bool) -> i32;
    pub fn is_mouse_key(c: i32) -> i32;
    pub fn get_keystroke(events: *mut MultiQueue) -> i32;
    pub fn get_number(colon: i32, mouse_used: *mut i32) -> i32;
    pub fn prompt_for_number(mouse_used: *mut i32) -> i32;
    pub fn msgmore(n: i64);
    pub fn beep_flush();
    pub fn vim_beep(val: u32);
    pub fn get_users(xp: *mut expand_T, idx: i32) -> *mut u8;
    pub fn match_user(name: *mut u8) -> i32;
    pub fn preserve_exit() -> !;
    static mut breakcheck_count: i32;
    pub fn line_breakcheck();
    pub fn fast_breakcheck();
    pub fn call_shell(cmd: *mut u8, opts: ShellOpts, extra_shell_arg: *mut u8) -> i32;
    pub fn get_cmd_output(cmd: *mut u8, infile: *mut u8, flags: ShellOpts, ret_len: *mut size_t) -> *mut u8;
    pub fn FreeWild(count: i32, files: *mut *mut u8);
    pub fn goto_im() -> i32;
}
