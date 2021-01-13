#![allow(dead_code)]
extern "C" {
    pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    pub fn free(__ptr: *mut libc::c_void);
    pub fn abort() -> !;
    pub fn abs(_: libc::c_int) -> libc::c_int;
    pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    pub fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
}
