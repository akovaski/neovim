use crate::*;

extern "C" {
    pub fn syntax_clear(block: *mut synblock_T);
    pub fn reset_synblock(wp: *mut win_T);
    pub fn syn_namen2id(linep: *const u8, len: libc::c_int) -> libc::c_int;
}
