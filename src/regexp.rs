use crate::*;

extern "C" {
    pub fn vim_regcomp(expr_arg: *mut u8, re_flags: libc::c_int) -> *mut regprog_T;
    pub fn vim_regfree(prog: *mut regprog_T);
    pub fn vim_regexec(rmp: *mut regmatch_T, line: *mut u8, col: colnr_T) -> libc::c_int;
}
pub const RE_MAGIC: libc::c_int = 1 as libc::c_int;
pub const RE_STRING: libc::c_int = 2 as libc::c_int;
pub const RE_STRICT: libc::c_int = 4 as libc::c_int;
pub const RE_AUTO: libc::c_int = 8 as libc::c_int;
pub const REX_USE: libc::c_int = 2 as libc::c_int;
pub const REX_ALL: libc::c_int = REX_SET | REX_USE;
pub const REX_SET: libc::c_int = 1 as libc::c_int;
