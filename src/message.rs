use crate::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg_hist {
    pub next: *mut msg_hist,
    pub msg: *mut u8,
    pub kind: *const i8,
    pub attr: i32,
    pub multiline: bool,
}
pub type MessageHistoryEntry = msg_hist;
pub const VIM_NO: i32 = 3;
pub const VIM_YES: i32 = 2;
pub const VIM_CANCEL: i32 = 4;
pub const VIM_DISCARDALL: i32 = 6;
pub const VIM_ALL: i32 = 5;
extern "C" {
    pub static mut msg_ext_need_clear: bool;
    pub static mut msg_grid: ScreenGrid;
    pub static mut msg_grid_pos: i32;
    pub static mut msg_grid_adj: ScreenGrid;
    pub static mut msg_scrolled_at_flush: i32;
}
pub const HAS_HOTKEY_LEN: i32 = 30;
pub type msgchunk_T = msgchunk_S;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msgchunk_S {
    pub sb_next: *mut msgchunk_T,
    pub sb_prev: *mut msgchunk_T,
    pub sb_eol: i8,
    pub sb_msg_col: i32,
    pub sb_attr: i32,
    pub sb_text: [u8; 1],
}
pub type sb_clear_T = u32;
pub const SB_CLEAR_CMDLINE_DONE: sb_clear_T = 3;
pub const SB_CLEAR_CMDLINE_BUSY: sb_clear_T = 2;
pub const SB_CLEAR_ALL: sb_clear_T = 1;
pub const SB_CLEAR_NONE: sb_clear_T = 0;
pub const DLG_BUTTON_SEP: i32 = '\n' as i32;
pub const DLG_HOTKEY_CHAR: i32 = '&' as i32;
extern "C" {
    pub static mut first_msg_hist: *mut MessageHistoryEntry;
    pub static mut last_msg_hist: *mut MessageHistoryEntry;
}
extern "C" {
    pub fn msg_grid_set_pos(row: i32, scrolled: bool);
    pub fn msg_use_grid() -> bool;
    pub fn msg_grid_validate();
    pub fn msg(s: *const u8) -> i32;
    pub fn verb_msg(s: *mut i8) -> i32;
    pub fn msg_attr(s: *const i8, attr: i32) -> i32;
    pub fn msg_multiline_attr(s: *const i8, attr: i32, check_int: bool, need_clear: *mut bool);
    pub fn msg_attr_keep(s: *mut u8, attr: i32, keep: bool, multiline: bool) -> bool;
    pub fn msg_strtrunc(s: *mut u8, force: i32) -> *mut u8;
    pub fn trunc_string(s: *mut u8, buf: *mut u8, room_in: i32, buflen: i32);
    pub fn smsg(s: *const i8, args: ...) -> i32;
    pub fn smsg_attr(attr: i32, s: *const i8, args: ...) -> i32;
    pub fn smsg_attr_keep(attr: i32, s: *mut i8, args: ...) -> i32;
    pub fn reset_last_sourcing();
    pub fn msg_source(attr: i32);
    pub fn emsg_not_now() -> i32;
    pub fn emsg(s: *const u8) -> bool;
    pub fn emsg_invreg(name: i32);
    pub fn emsgf(fmt: *const i8, args: ...) -> bool;
    pub fn emsgf_multiline(fmt: *const i8, args: ...) -> bool;
    pub fn iemsg(s: *const i8);
    pub fn iemsgf(s: *const i8, args: ...);
    pub fn internal_error(where_0: *mut i8);
    pub fn msg_schedule_emsgf(fmt: *const i8, args: ...);
    pub fn msg_trunc_attr(s: *mut u8, force: i32, attr: i32) -> *mut u8;
    pub fn msg_may_trunc(force: i32, s: *mut u8) -> *mut u8;
    pub fn delete_first_msg() -> i32;
    pub fn ex_messages(eap_p: *mut libc::c_void);
    pub fn msg_end_prompt();
    pub fn wait_return(redraw: i32);
    pub fn set_keep_msg(s: *mut u8, attr: i32);
    pub fn msg_ext_set_kind(msg_kind: *const i8);
    pub fn msg_start();
    pub fn msg_starthere();
    pub fn msg_putchar(c: i32);
    pub fn msg_putchar_attr(c: i32, attr: i32);
    pub fn msg_outnum(n: i64);
    pub fn msg_home_replace(fname: *mut u8);
    pub fn msg_home_replace_hl(fname: *mut u8);
    pub fn msg_outtrans(str: *mut u8) -> i32;
    pub fn msg_outtrans_attr(str: *const u8, attr: i32) -> i32;
    pub fn msg_outtrans_len(str: *const u8, len: i32) -> i32;
    pub fn msg_outtrans_one(p: *mut u8, attr: i32) -> *mut u8;
    pub fn msg_outtrans_len_attr(msgstr: *const u8, len: i32, attr: i32) -> i32;
    pub fn msg_make(arg: *mut u8);
    pub fn msg_outtrans_special(strstart: *const u8, from: bool, maxlen: i32) -> i32;
    pub fn str2special_save(str: *const i8, replace_spaces: bool, replace_lt: bool) -> *mut i8;
    pub fn str2special(sp: *mut *const i8, replace_spaces: bool, replace_lt: bool) -> *const i8;
    pub fn str2specialbuf(sp: *const i8, buf: *mut i8, len: size_t);
    pub fn msg_prt_line(s: *mut u8, list: i32);
    pub fn msg_puts(s: *const i8);
    pub fn msg_puts_title(s: *const i8);
    pub fn msg_puts_long_attr(longstr: *mut u8, attr: i32);
    pub fn msg_puts_long_len_attr(longstr: *mut u8, len: i32, attr: i32);
    pub fn msg_puts_attr(s: *const i8, attr: i32);
    pub fn msg_puts_attr_len(str: *const i8, len: ptrdiff_t, attr: i32);
    pub fn msg_printf_attr(attr: i32, fmt: *const i8, args: ...);
    pub fn message_filtered(msg_0: *mut u8) -> bool;
    pub fn msg_scrollsize() -> i32;
    pub fn msg_use_msgsep() -> bool;
    pub fn msg_do_throttle() -> bool;
    pub fn msg_scroll_up(may_throttle: bool);
    pub fn msg_scroll_flush();
    pub fn msg_reset_scroll();
    pub fn may_clear_sb_text();
    pub fn sb_text_start_cmdline();
    pub fn sb_text_end_cmdline();
    pub fn clear_sb_text(all: i32);
    pub fn show_sb_text();
    pub fn msg_sb_eol();
    pub fn msg_use_printf() -> i32;
    pub fn msg_moremsg(full: i32);
    pub fn repeat_message();
    pub fn msg_clr_eos();
    pub fn msg_clr_eos_force();
    pub fn msg_clr_cmdline();
    pub fn msg_end() -> i32;
    pub fn msg_ext_ui_flush();
    pub fn msg_ext_flush_showmode();
    pub fn msg_ext_clear(force: bool);
    pub fn msg_ext_clear_later();
    pub fn msg_ext_check_clear();
    pub fn msg_ext_is_visible() -> bool;
    pub fn msg_check();
    pub fn redirecting() -> i32;
    pub fn verbose_enter();
    pub fn verbose_leave();
    pub fn verbose_enter_scroll();
    pub fn verbose_leave_scroll();
    pub fn verbose_stop();
    pub fn verbose_open() -> i32;
    pub fn give_warning(message: *mut u8, hl: bool);
    pub fn give_warning2(message: *mut u8, a1: *mut u8, hl: bool);
    pub fn msg_advance(col: i32);
    pub fn do_dialog(
        type_0: i32,
        title: *mut u8,
        message: *mut u8,
        buttons: *mut u8,
        dfltbutton: i32,
        textfield: *mut u8,
        ex_cmd: i32,
    ) -> i32;
    pub fn display_confirm_msg();
    pub fn vim_dialog_yesno(type_0: i32, title: *mut u8, message: *mut u8, dflt: i32) -> i32;
    pub fn vim_dialog_yesnocancel(type_0: i32, title: *mut u8, message: *mut u8, dflt: i32) -> i32;
    pub fn vim_dialog_yesnoallcancel(
        type_0: i32,
        title: *mut u8,
        message: *mut u8,
        dflt: i32,
    ) -> i32;
}
