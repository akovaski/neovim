use crate::*;
extern "C" {
    pub fn try_getdigits(pp: *mut *mut libc::c_uchar, nr: *mut intmax_t) -> bool;
    pub fn vim_str2nr(
        start: *const libc::c_uchar,
        prep: *mut libc::c_int,
        len: *mut libc::c_int,
        what: ChStr2NrFlags,
        nptr: *mut varnumber_T,
        unptr: *mut uvarnumber_T,
        maxlen: libc::c_int,
    );
    #[no_mangle]
    pub fn hex2nr(c: libc::c_int) -> libc::c_int;
}

pub type ChStr2NrFlags = i32;
pub const STR2NR_ALL: ChStr2NrFlags = 7;
pub const STR2NR_FORCE: ChStr2NrFlags = 8;
pub const STR2NR_HEX: ChStr2NrFlags = 4;
pub const STR2NR_OCT: ChStr2NrFlags = 2;
pub const STR2NR_BIN: ChStr2NrFlags = 1;
pub const STR2NR_DEC: ChStr2NrFlags = 0;
