#![allow(dead_code)]
use crate::*;

pub struct data_block {
    pub db_id: u16,
    pub db_free: u32,
    pub db_txt_start: u32,
    pub db_txt_end: u32,
    pub db_line_count: linenr_T,
    pub db_index: [u32; 1],
}
pub struct pointer_entry {
    pub pe_bnum: blocknr_T,
    pub pe_line_count: linenr_T,
    pub pe_old_lnum: linenr_T,
    pub pe_page_count: i32,
}
pub struct pointer_block {
    pub pb_id: u16,
    pub pb_count: u16,
    pub pb_count_max: u16,
    pub pb_pointer: [PTR_EN; 1],
}
pub struct block0 {
    pub b0_id: [u8; 2],
    pub b0_version: [u8; 10],
    pub b0_page_size: [u8; 4],
    pub b0_mtime: [u8; 4],
    pub b0_ino: [u8; 4],
    pub b0_pid: [u8; 4],
    pub b0_uname: [u8; 40],
    pub b0_hname: [u8; 40],
    pub b0_fname: [u8; 900],
    pub b0_magic_long: i64,
    pub b0_magic_int: i32,
    pub b0_magic_short: i16,
    pub b0_magic_char: u8,
}
pub const UB_SAME_DIR: upd_block0_T = 1;
pub const UB_FNAME: upd_block0_T = 0;
pub const DATA_ID: i32 = (('d' as i32) << 8) + 'a' as i32;
pub const PTR_ID: i32 = (('p' as i32) << 8) + 't' as i32;
pub const BLOCK0_ID0: i32 = 'b' as i32;
pub const BLOCK0_ID1: i32 = '0' as i32;
pub const INDEX_SIZE: u64 = ::std::mem::size_of::<u32>() as u64;
pub const B0_FNAME_SIZE_ORG: i32 = 900;
pub const B0_FNAME_SIZE_NOCRYPT: i32 = 898;
pub const B0_FNAME_SIZE_CRYPT: i32 = 890;
pub const B0_UNAME_SIZE: i32 = 40;
pub const B0_HNAME_SIZE: i32 = 40;
pub const B0_MAGIC_LONG: i64 = 0x30313233 as i64;
pub const B0_MAGIC_INT: i64 = 0x20212223 as i64;
pub const B0_MAGIC_SHORT: i64 = 0x10111213 as i64;
pub const B0_MAGIC_CHAR: i32 = 0x55 as i32;
pub const B0_DIRTY: i32 = 0x55 as i32;
pub const B0_FF_MASK: i32 = 3;
pub const B0_SAME_DIR: i32 = 4;
pub const B0_HAS_FENC: i32 = 8;
pub const STACK_INCR: i32 = 5;
pub const ML_DELETE: i32 = 0x11 as i32;
pub const ML_INSERT: i32 = 0x12 as i32;
pub const ML_FIND: i32 = 0x13 as i32;
pub const ML_FLUSH: i32 = 0x2 as i32;
pub const MLCS_MAXL: i32 = 800;
pub const MLCS_MINL: i32 = 400;
pub type DATA_BL = data_block;
pub type PTR_EN = pointer_entry;
pub type PTR_BL = pointer_block;
pub type ZERO_BL = block0;
pub type upd_block0_T = u32;
extern "C" {
    static mut lowest_marked: linenr_T;
    pub fn ml_open(buf: *mut buf_T) -> i32;
    pub fn ml_setname(buf: *mut buf_T);
    pub fn ml_open_files();
    pub fn ml_open_file(buf: *mut buf_T);
    pub fn check_need_swap(newfile: bool);
    pub fn ml_close(buf: *mut buf_T, del_file: i32);
    pub fn ml_close_all(del_file: i32);
    pub fn ml_close_notmod();
    pub fn ml_timestamp(buf: *mut buf_T);
    pub fn ml_recover(checkext: bool);
    pub fn recover_names(fname: *mut u8, list: i32, nr: i32, fname_out: *mut *mut u8) -> i32;
    pub fn make_percent_swname(dir: *const i8, name: *mut i8) -> *mut i8;
    static mut process_still_running: bool;
    pub fn get_b0_dict(fname: *const i8, d: *mut dict_T);
    pub fn ml_sync_all(check_file: i32, check_char: i32, do_fsync: bool);
    pub fn ml_preserve(buf: *mut buf_T, message: i32, do_fsync: bool);
    pub fn ml_get(lnum: linenr_T) -> *mut u8;
    pub fn ml_get_pos(pos: *const pos_T) -> *mut u8;
    pub fn ml_get_buf(buf: *mut buf_T, lnum: linenr_T, will_change: bool) -> *mut u8;
    pub fn ml_line_alloced() -> i32;
    pub fn ml_append(lnum: linenr_T, line: *mut u8, len: colnr_T, newfile: bool) -> i32;
    pub fn ml_append_buf(buf: *mut buf_T, lnum: linenr_T, line: *mut u8, len: colnr_T, newfile: bool) -> i32;
    pub fn ml_add_deleted_len(ptr: *mut u8, len: ssize_t);
    pub fn ml_replace(lnum: linenr_T, line: *mut u8, copy: bool) -> i32;
    pub fn ml_delete(lnum: linenr_T, message: bool) -> i32;
    pub fn ml_setmarked(lnum: linenr_T);
    pub fn ml_firstmarked() -> linenr_T;
    pub fn ml_clearmarked();
    pub fn ml_flush_deleted_bytes(buf: *mut buf_T, codepoints: *mut size_t, codeunits: *mut size_t) -> size_t;
    pub fn resolve_symlink(fname: *const u8, buf: *mut u8) -> i32;
    pub fn makeswapname(fname: *mut u8, ffname: *mut u8, buf: *mut buf_T, dir_name: *mut u8) -> *mut u8;
    pub fn get_file_in_dir(fname: *mut u8, dname: *mut u8) -> *mut u8;
    pub fn ml_setflags(buf: *mut buf_T);
    pub fn ml_find_line_or_offset(buf: *mut buf_T, lnum: linenr_T, offp: *mut i64, no_ff: bool) -> i64;
    pub fn goto_byte(cnt: i64);
    pub fn inc(lp: *mut pos_T) -> i32;
    pub fn incl(lp: *mut pos_T) -> i32;
    pub fn dec(lp: *mut pos_T) -> i32;
    pub fn decl(lp: *mut pos_T) -> i32;
}
