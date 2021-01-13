use crate::*;

pub const STL_MAX_ITEM: usize = 80;

pub type Direction = libc::c_int;
pub const BACKWARD_FILE: Direction = -3;
pub const FORWARD_FILE: Direction = 3;
pub const BACKWARD: Direction = -1;
pub const FORWARD: Direction = 1;
pub const kDirectionNotSet: Direction = 0;

pub const FAIL: libc::c_int = 0;
pub const OK: libc::c_int = 1;
pub const NOTDONE: libc::c_int = 2; // not OK or FAIL but skipped

pub unsafe fn STRMOVE(d: *mut u8, s: *const u8) -> *mut libc::c_void {
    memmove(d, s, strlen(s as *const i8) + 1)
}

// values for State
//
// The lower bits up to 0x20 are used to distinguish normal/visual/op_pending
// and cmdline/insert+replace mode.  This is used for mapping.  If none of
// these bits are set, no mapping is done.
// The upper bits are used to distinguish between other states.

pub const NORMAL: i32 = 0x01; // Normal mode, command expected
pub const VISUAL: i32 = 0x02; // Visual mode - use get_real_state()
pub const OP_PENDING: i32 = 0x04; // Normal mode, operator is pending - use
                                  // get_real_state()
pub const CMDLINE: i32 = 0x08; // Editing command line
pub const INSERT: i32 = 0x10; // Insert mode
pub const LANGMAP: i32 = 0x20; // Language mapping, can be combined with
                               // INSERT and CMDLINE

pub const REPLACE_FLAG: i32 = 0x40; // Replace mode flag
pub const REPLACE: i32 = REPLACE_FLAG + INSERT;
pub const VREPLACE_FLAG: i32 = 0x80; // Virtual-replace mode flag
pub const VREPLACE: i32 = REPLACE_FLAG + VREPLACE_FLAG + INSERT;
pub const LREPLACE: i32 = REPLACE_FLAG + LANGMAP;

pub const NORMAL_BUSY: i32 = 0x100 + NORMAL; // Normal mode, busy with a command
pub const HITRETURN: i32 = 0x200 + NORMAL; // waiting for return or command
pub const ASKMORE: i32 = 0x300; // Asking if you want --more--
pub const SETWSIZE: i32 = 0x400; // window size has changed
pub const ABBREV: i32 = 0x500; // abbreviation instead of mapping
pub const EXTERNCMD: i32 = 0x600; // executing an external command
pub const SHOWMATCH: i32 = 0x700 + INSERT; // show matching paren
pub const CONFIRM: i32 = 0x800; // ":confirm" prompt
pub const SELECTMODE: i32 = 0x1000; // Select mode, only for mappings
pub const TERM_FOCUS: i32 = 0x2000; // Terminal focus mode
pub const CMDPREVIEW: i32 = 0x4000; // Showing 'inccommand' command "live" preview.

// all mode bits used for mapping
pub const MAP_ALL_MODES: i32 = 0x3f | SELECTMODE | TERM_FOCUS;
