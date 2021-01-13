extern "C" {
    pub fn path_has_wildcard(p: *const libc::c_uchar) -> bool;
    pub fn path_full_compare(
        s1: *mut u8,
        s2: *mut u8,
        checkname: bool,
        expandenv: bool,
    ) -> FileComparison;
    pub fn path_tail(fname: *const u8) -> *mut u8;
    pub fn path_tail_with_sep(fname: *mut u8) -> *mut u8;
    pub fn path_fnamecmp(fname1: *const libc::c_char, fname2: *const libc::c_char) -> libc::c_int;
    pub fn FullName_save(fname: *const libc::c_char, force: bool) -> *mut libc::c_char;
    pub fn fix_fname(fname: *const libc::c_char) -> *mut libc::c_char;
}

pub unsafe fn fnamecmp<T, U>(fname1: *const T, fname2: *const U) -> i32 {
    path_fnamecmp(fname1 as *const libc::c_char, fname2 as *const libc::c_char) as i32
}
pub type file_comparison = libc::c_uint;
pub const kEqualFileNames: file_comparison = 7;
pub const kOneFileMissing: file_comparison = 6;
pub const kBothFilesMissing: file_comparison = 4;
pub const kDifferentFiles: file_comparison = 2;
pub const kEqualFiles: file_comparison = 1;
pub type FileComparison = file_comparison;
pub const EW_PATH: libc::c_int = 0x80 as libc::c_int;
pub const EW_KEEPALL: libc::c_int = 0x10 as libc::c_int;
pub const EW_SILENT: libc::c_int = 0x20 as libc::c_int;
pub const EW_NOTFOUND: libc::c_int = 0x4 as libc::c_int;
pub const EW_DIR: libc::c_int = 0x1 as libc::c_int;
pub const EW_FILE: libc::c_int = 0x2 as libc::c_int;
pub const EW_EXEC: libc::c_int = 0x40 as libc::c_int;
pub const EW_ADDSLASH: libc::c_int = 0x8 as libc::c_int;
pub const EW_ICASE: libc::c_int = 0x100 as libc::c_int;
pub const EW_NOERROR: libc::c_int = 0x200 as libc::c_int;
pub const EW_NOTWILD: libc::c_int = 0x400 as libc::c_int;
pub const EW_KEEPDOLLAR: libc::c_int = 0x800 as libc::c_int;
pub const EW_ALLLINKS: libc::c_int = 0x1000 as libc::c_int;
pub const EW_SHELLCMD: libc::c_int = 0x2000 as libc::c_int;
pub const EW_DODOT: libc::c_int = 0x4000 as libc::c_int;
pub const EW_EMPTYOK: libc::c_int = 0x8000 as libc::c_int;
pub const EW_NOTENV: libc::c_int = 0x10000 as libc::c_int;
