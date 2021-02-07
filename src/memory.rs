use crate::*;
use std::convert::TryInto;

extern "C" {
    // C std
    #[link_name = "memcpy"]
    pub fn c_memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: usize) -> *mut libc::c_void;
    #[link_name = "memmove"]
    pub fn c_memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    #[link_name = "memchr"]
    pub fn c_memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type MemMalloc = Option<unsafe extern "C" fn(_: size_t) -> *mut libc::c_void>;
pub type MemFree = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type MemCalloc = Option<unsafe extern "C" fn(_: size_t, _: size_t) -> *mut libc::c_void>;
pub type MemRealloc =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void>;

pub unsafe fn XFREE_CLEAR<T>(ptr: &mut *mut T) {
    xfree(*ptr as *mut libc::c_void);
    *ptr = std::ptr::null_mut();
}

pub unsafe fn memcpy<T>(dest: *mut T, src: *const T, count: usize) -> *mut libc::c_void {
    c_memcpy(dest as *mut libc::c_void, src as *const libc::c_void, count)
}

pub unsafe fn memmove<T, N: TryInto<u64>>(dst: *mut T, src: *const T, num: N) -> *mut libc::c_void
where
    <N as TryInto<u64>>::Error: std::fmt::Debug,
{
    c_memmove(
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        num.try_into().unwrap(),
    )
}

pub unsafe fn memchr<T, N: std::convert::TryInto<u64>, C: Into<char>>(
    ptr: *const T,
    value: C,
    num: N,
) -> *mut libc::c_void
where
    <N as std::convert::TryInto<u64>>::Error: std::fmt::Debug,
{
    let value: char = value.into();
    c_memchr(
        ptr as *const libc::c_void,
        value as i32,
        num.try_into().unwrap(),
    )
}

pub unsafe fn memcmp<T>(ptr1: *const T, ptr2: *const T, n: usize) -> libc::c_int {
    libc::memcmp(ptr1 as *const libc::c_void, ptr2 as *const libc::c_void, n)
}

pub unsafe fn memset<T>(dest: *mut T, c: libc::c_int, n: usize) -> *mut T {
    libc::memset(dest as *mut libc::c_void, c, n) as *mut T
}

pub unsafe fn xmalloc<S, T>(size: S) -> *mut T
where
    S: TryInto<usize>,
    <S as TryInto<usize>>::Error: std::fmt::Debug,
{
    c_xmalloc(size.try_into().unwrap()) as *mut T
}

pub unsafe fn xfree<T>(ptr: *mut T) {
    c_xfree(ptr as *mut libc::c_void);
}

pub unsafe fn xcalloc<T>(count: usize, size: usize) -> *mut T {
    c_xcalloc(count, size) as *mut T
}

pub unsafe fn xrealloc<S, T>(ptr: *mut S, size: usize) -> *mut T {
    c_xrealloc(ptr as *mut libc::c_void, size) as *mut T
}

pub unsafe fn xmallocz<T: Into<usize>, U>(size: T) -> *mut U {
    c_xmallocz(size.into()) as *mut U
}

pub unsafe fn xstrdup(str: &str) -> *mut libc::c_char {
    let str = std::ffi::CString::new(str).expect("CString::new failed");
    let c_str = str.as_ptr();
    c_xstrdup(c_str)
}

pub unsafe fn xmemdupz<S, T: Into<size_t>>(data: *const S, len: T) -> *mut S {
    c_xmemdupz(data as *const _, len.into()) as *mut S
}
extern "C" {
    pub static mut mem_malloc: MemMalloc;
    pub static mut mem_free: MemFree;
    pub static mut mem_calloc: MemCalloc;
    pub static mut mem_realloc: MemRealloc;
    pub fn try_to_free_memory();
    pub fn try_malloc(size: size_t) -> *mut libc::c_void;
    pub fn verbose_try_malloc(size: size_t) -> *mut libc::c_void;
    #[link_name = "xmalloc"]
    pub fn c_xmalloc(size: size_t) -> *mut libc::c_void;
    #[link_name = "xfree"]
    pub fn c_xfree(ptr: *mut libc::c_void);
    #[link_name = "xcalloc"]
    pub fn c_xcalloc(count: size_t, size: size_t) -> *mut libc::c_void;
    #[link_name = "xrealloc"]
    pub fn c_xrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    #[link_name = "xmallocz"]
    pub fn c_xmallocz(size: size_t) -> *mut libc::c_void;
    #[link_name = "xmemdupz"]
    pub fn c_xmemdupz(data: *const libc::c_void, len: size_t) -> *mut libc::c_void;
    pub fn xstrchrnul(str: *const i8, c: i8) -> *mut i8;
    pub fn xmemscan(addr: *const libc::c_void, c: i8, size: size_t) -> *mut libc::c_void;
    pub fn strchrsub(str: *mut i8, c: i8, x: i8);
    pub fn memchrsub(data: *mut libc::c_void, c: i8, x: i8, len: size_t);
    pub fn strcnt(str: *const i8, c: i8) -> size_t;
    pub fn memcnt(data: *const libc::c_void, c: i8, len: size_t) -> size_t;
    pub fn xstpcpy(dst: *mut i8, src: *const i8) -> *mut i8;
    pub fn xstpncpy(dst: *mut i8, src: *const i8, maxlen: size_t) -> *mut i8;
    pub fn xstrlcpy(dst: *mut i8, src: *const i8, dsize: size_t) -> size_t;
    pub fn xstrlcat(dst: *mut i8, src: *const i8, dsize: size_t) -> size_t;
    #[link_name = "xstrdup"]
    pub fn c_xstrdup(str: *const i8) -> *mut i8;
    pub fn xstrdupnul(str: *const i8) -> *mut i8;
    pub fn xmemrchr(src: *const libc::c_void, c: u8, len: size_t) -> *mut libc::c_void;
    pub fn xstrndup(str: *const i8, len: size_t) -> *mut i8;
    pub fn xmemdup(data: *const libc::c_void, len: size_t) -> *mut libc::c_void;
    pub fn strequal(a: *const i8, b: *const i8) -> bool;
    pub fn striequal(a: *const i8, b: *const i8) -> bool;
    pub fn do_outofmem_msg(size: size_t);
    pub fn time_to_bytes(time_: time_t, buf: *mut u8);
}
