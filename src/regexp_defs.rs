use crate::buffer_defs::*;
use crate::pos::*;
use crate::profile::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct regprog_T {
    pub engine: *mut regengine_T,
    pub regflags: libc::c_uint,
    pub re_engine: libc::c_uint,
    pub re_flags: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regengine_T {
    pub regcomp:
        Option<unsafe extern "C" fn(_: *mut libc::c_uchar, _: libc::c_int) -> *mut regprog_T>,
    pub regfree: Option<unsafe extern "C" fn(_: *mut regprog_T) -> ()>,
    pub regexec_nl: Option<
        unsafe extern "C" fn(
            _: *mut regmatch_T,
            _: *mut libc::c_uchar,
            _: colnr_T,
            _: bool,
        ) -> libc::c_int,
    >,
    pub regexec_multi: Option<
        unsafe extern "C" fn(
            _: *mut regmmatch_T,
            _: *mut win_T,
            _: *mut buf_T,
            _: linenr_T,
            _: colnr_T,
            _: *mut proftime_T,
            _: *mut libc::c_int,
        ) -> libc::c_long,
    >,
    pub expr: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmmatch_T {
    pub regprog: *mut regprog_T,
    pub startpos: [lpos_T; 10],
    pub endpos: [lpos_T; 10],
    pub rmm_ic: libc::c_int,
    pub rmm_maxcol: colnr_T,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_T {
    pub regprog: *mut regprog_T,
    pub startp: [*mut libc::c_uchar; 10],
    pub endp: [*mut libc::c_uchar; 10],
    pub rm_ic: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reg_extmatch_T {
    pub refcnt: i16,
    pub matches: [*mut libc::c_uchar; 10],
}
