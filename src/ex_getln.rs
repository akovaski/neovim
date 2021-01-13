use crate::*;

extern "C" {
    pub fn text_locked() -> libc::c_int;
    pub fn text_locked_msg();
    pub fn curbuf_locked() -> libc::c_int;
}
pub type HistoryType = libc::c_int;
pub const HIST_DEBUG: HistoryType = 4;
pub const HIST_INPUT: HistoryType = 3;
pub const HIST_EXPR: HistoryType = 2;
pub const HIST_SEARCH: HistoryType = 1;
pub const HIST_CMD: HistoryType = 0;
pub const HIST_INVALID: HistoryType = -1;
pub const HIST_DEFAULT: HistoryType = -2;
pub type CompleteListItemGetter =
    Option<unsafe extern "C" fn(_: *mut expand_T, _: libc::c_int) -> *mut u8>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hist_entry {
    pub hisnum: libc::c_int,
    pub hisstr: *mut u8,
    pub timestamp: Timestamp,
    pub additional_elements: *mut list_T,
}

pub type histentry_T = hist_entry;

pub const WILD_HOME_REPLACE: libc::c_int = 0x2 as libc::c_int;
pub const WILD_LIST_NOTFOUND: libc::c_int = 0x1 as libc::c_int;
pub const WILD_ADD_SLASH: libc::c_int = 0x10 as libc::c_int;
pub const WILD_KEEP_ALL: libc::c_int = 0x20 as libc::c_int;
pub const WILD_NOERROR: libc::c_int = 0x800 as libc::c_int;
pub const WILD_ALLLINKS: libc::c_int = 0x200 as libc::c_int;
pub const WILD_ICASE: libc::c_int = 0x100 as libc::c_int;
pub const WILD_EXPAND_KEEP: libc::c_int = 3 as libc::c_int;
pub const WILD_ESCAPE: libc::c_int = 0x80 as libc::c_int;
pub const WILD_ALL_KEEP: libc::c_int = 8 as libc::c_int;
pub const WILD_SILENT: libc::c_int = 0x40 as libc::c_int;
pub const WILD_LONGEST: libc::c_int = 7 as libc::c_int;
pub const WILD_NO_BEEP: libc::c_int = 0x8 as libc::c_int;
pub const WILD_EXPAND_FREE: libc::c_int = 2 as libc::c_int;
pub const WILD_ALL: libc::c_int = 6 as libc::c_int;
pub const WILD_FREE: libc::c_int = 1 as libc::c_int;
pub const WILD_PREV: libc::c_int = 5 as libc::c_int;
pub const HIST_COUNT: libc::c_int = HIST_DEBUG as libc::c_int + 1 as libc::c_int;
pub const WILD_NEXT: libc::c_int = 4 as libc::c_int;
