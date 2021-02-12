use crate::*;

pub const BREAKCHECK_SKIP: i32 = 1000;
extern "C" {
    pub fn exit_free() -> bool;
    static mut ga_users: garray_T;
    pub fn get_leader_len(mut line: *mut u8, mut flags: *mut *mut u8, mut backward: i32, mut include_space: i32) -> i32;
    pub fn get_last_leader_offset(mut line: *mut u8, mut flags: *mut *mut u8) -> i32;
    pub fn plines(lnum: linenr_T) -> i32;
    pub fn plines_win(wp: *mut win_T, lnum: linenr_T, winheight: bool) -> i32;
    pub fn plines_nofill(lnum: linenr_T) -> i32;
    pub fn plines_win_nofill(wp: *mut win_T, lnum: linenr_T, winheight: bool) -> i32;
    pub fn plines_win_nofold(mut wp: *mut win_T, mut lnum: linenr_T) -> i32;
    pub fn plines_win_col(mut wp: *mut win_T, mut lnum: linenr_T, mut column: i64) -> i32;
    pub fn plines_win_full(mut wp: *mut win_T, mut lnum: linenr_T, nextp: *mut linenr_T, foldedp: *mut bool, cache: bool) -> i32;
    pub fn plines_m_win(mut wp: *mut win_T, mut first: linenr_T, mut last: linenr_T) -> i32;
    pub fn gchar_pos(mut pos: *mut pos_T) -> i32;
    pub fn check_status(mut buf: *mut buf_T);
    pub fn ask_yesno(str: *const i8, direct: bool) -> i32;
    pub fn is_mouse_key(mut c: i32) -> i32;
    pub fn get_keystroke(mut events: *mut MultiQueue) -> i32;
    pub fn get_number(mut colon: i32, mut mouse_used: *mut i32) -> i32;
    pub fn prompt_for_number(mut mouse_used: *mut i32) -> i32;
    pub fn msgmore(mut n: i64);
    pub fn beep_flush();
    pub fn vim_beep(mut val: u32);
    pub fn get_users(mut xp: *mut expand_T, mut idx: i32) -> *mut u8;
    pub fn match_user(mut name: *mut u8) -> i32;
    pub fn preserve_exit() -> !;
    static mut breakcheck_count: i32;
    pub fn line_breakcheck();
    pub fn fast_breakcheck();
    pub fn call_shell(mut cmd: *mut u8, mut opts: ShellOpts, mut extra_shell_arg: *mut u8) -> i32;
    pub fn get_cmd_output(mut cmd: *mut u8, mut infile: *mut u8, mut flags: ShellOpts, mut ret_len: *mut size_t) -> *mut u8;
    pub fn FreeWild(mut count: i32, mut files: *mut *mut u8);
    pub fn goto_im() -> i32;
}
