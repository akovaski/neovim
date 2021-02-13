use crate::buffer_defs::*;
use crate::pos::pos_T;
use crate::*;

extern "C" {
    pub static mut VIsual: pos_T;
}

pub type WorkingStatus = u32;
pub const kBroken: WorkingStatus = 2;
pub const kWorking: WorkingStatus = 1;
pub const kUnknown: WorkingStatus = 0;

pub const DBCS_JPN: i32 = 932;
pub const DBCS_JPNU: i32 = 9932;
pub const DBCS_KOR: i32 = 949;
pub const DBCS_KORU: i32 = 9949;
pub const DBCS_CHS: i32 = 936;
pub const DBCS_CHSU: i32 = 9936;
pub const DBCS_CHT: i32 = 950;
pub const DBCS_CHTU: i32 = 9950;
pub const DBCS_DEBUG: i32 = -(1);

pub type CdScope = libc::c_int;
pub const kCdScopeGlobal: CdScope = 2;
pub const kCdScopeTab: CdScope = 1;
pub const kCdScopeWindow: CdScope = 0;
pub const kCdScopeInvalid: CdScope = -1;

pub const SID_MODELINE: libc::c_int = -(1 as libc::c_int);
pub const SEA_DIALOG: libc::c_int = 1 as libc::c_int;
pub const IOSIZE: i32 = 1024 + 1;
pub const has_mbyte: bool = true;
pub const SID_ERROR: libc::c_int = -(5 as libc::c_int);
pub const STL_IN_ICON: libc::c_int = 1 as libc::c_int;
pub const STL_IN_TITLE: libc::c_int = 2 as libc::c_int;
pub const SEA_NONE: libc::c_int = 0 as libc::c_int;
pub const SEA_RECOVER: libc::c_int = 3 as libc::c_int;
pub const SEA_QUIT: libc::c_int = 2 as libc::c_int;
extern "C" {
    pub static mut Rows: libc::c_int;
    pub static mut Columns: libc::c_int;
    pub static mut cmdline_row: libc::c_int;
    pub static mut msg_row: libc::c_int;
    pub static mut msg_scrolled: libc::c_int;
    pub static mut need_fileinfo: libc::c_int;
    pub static mut msg_scroll: libc::c_int;
    pub static mut called_emsg: libc::c_int;
    pub static mut need_wait_return: libc::c_int;
    pub static mut need_maketitle: libc::c_int;
    pub static mut sourcing_name: *mut u8;
    pub static mut sourcing_lnum: linenr_T;
    pub static mut current_sctx: sctx_T;
    pub static mut autocmd_busy: libc::c_int;
    pub static mut autocmd_no_enter: libc::c_int;
    pub static mut autocmd_no_leave: libc::c_int;
    pub static mut modified_was_set: libc::c_int;
    pub static mut did_filetype: libc::c_int;
    pub static mut au_new_curbuf: bufref_T;
    pub static mut au_pending_free_buf: *mut buf_T;
    pub static mut updating_screen: libc::c_int;
    pub static mut firstwin: *mut win_T;
    pub static mut lastwin: *mut win_T;
    pub static mut curwin: *mut win_T;
    pub static mut first_tabpage: *mut tabpage_T;
    pub static mut curtab: *mut tabpage_T;
    pub static mut firstbuf: *mut buf_T;
    pub static mut lastbuf: *mut buf_T;
    pub static mut curbuf: *mut buf_T;
    pub static mut global_alist: alist_T;
    pub static mut arg_had_last: bool;
    pub static mut starting: libc::c_int;
    pub static mut secure: libc::c_int;
    pub static mut VIsual_active: libc::c_int;
    pub static mut VIsual_reselect: libc::c_int;
    pub static mut State: libc::c_int;
    pub static mut restart_edit: libc::c_int;
    pub static mut cmdmod: cmdmod_T;
    pub static mut msg_silent: libc::c_int;
    pub static mut emsg_silent: libc::c_int;
    // use dialog when possible
    // quit editing the file
    // recover the file
    pub static mut swap_exists_action: libc::c_int;
    // For dialog when swap file already
    // exists.
    pub static mut swap_exists_did_quit: libc::c_int;
    // Selected "quit" at the dialog.
    pub static mut IObuff: [u8; 1025];
    pub static mut fenc_default: *mut u8;
    // /< Buffer for sprintf, I/O, etc.
    pub static mut NameBuff: [u8; 4096];
    // /< Buffer for the os/ layer
    // When non-zero, postpone redrawing.
    pub static mut RedrawingDisabled: libc::c_int;
    pub static mut readonlymode: libc::c_int;
    // tick for each non-mapped char
    pub static mut must_redraw: libc::c_int;
    // /< Stream to write script to.
    // volatile because it is used in a signal handler.
    pub static mut got_int: libc::c_int;
    // /< cmdline recursion level
    pub static mut no_lines_msg: [u8; 0];
    // whether titlestring and iconstring contains statusline syntax
    pub static mut stl_syntax: libc::c_int;
    // Page number used for %N in 'pageheader' and 'guitablabel'.
    pub static mut printer_page_num: linenr_T;
    pub static mut e_noalt: [u8; 0];
    pub static mut e_trailing: [u8; 0];
    pub static mut e_nobufnr: [u8; 0];
}

macro_rules! ONE_WINDOW {
    () => {
        (firstwin == lastwin)
    };
}

// Iterates over all buffers in the buffer list.
macro_rules! FOR_ALL_BUFFERS {
    ($buf: ident, $blk: block) => {
        let mut $buf = firstbuf;
        while !$buf.is_null() {
            $blk;
            $buf = (*$buf).b_next;
        }
    };
}
macro_rules! FOR_ALL_BUFFERS_BACKWARDS {
    ($buf: ident, $blk: block) => {
        let mut $buf = lastbuf;
        while !$buf.is_null() {
            $blk;
            $buf = (*$buf).b_prev;
        }
    };
}

// Iterate through all the signs placed in a buffer
macro_rules! FOR_ALL_SIGNS_IN_BUF {
    ($buf: ident, $sign: ident, $blk: block) => {
        $sign = $buf.b_signlist;
        while !$sign.is_null() {
            $blk;
            $sign = (*$sign).next
        }
    };
}

// When using this macro "break" only breaks out of the inner loop. Use "goto"
// to break out of the tabpage loop.
macro_rules! FOR_ALL_TAB_WINDOWS {
    ($tp: ident, $wp: ident, $blk: block) => {
        FOR_ALL_TABS!($tp, {
            FOR_ALL_WINDOWS_IN_TAB!($wp, $tp, $blk);
        });
    };
}
macro_rules! FOR_ALL_WINDOWS_IN_TAB {
    ($wp: ident, $tp: ident, $blk: block) => {
        let mut $wp = if $tp == curtab {
            firstwin
        } else {
            (*$tp).tp_firstwin
        };
        while !$wp.is_null() {
            $blk;
            $wp = (*$wp).w_next;
        }
    };
}

// Iterates over all tabs in the tab list
macro_rules! FOR_ALL_TABS {
    ($tp: ident, $blk: block) => {
        let mut $tp = first_tabpage;
        while !$tp.is_null() {
            $blk;
            $tp = (*$tp).tp_next;
        }
    };
}
