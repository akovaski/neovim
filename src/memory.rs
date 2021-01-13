use crate::*;
use std::convert::TryInto;
use std::mem;

extern "C" {
    // C std
    #[link_name = "memcpy"]
    pub fn c_memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::size_t,
    ) -> *mut libc::c_void;
    #[link_name = "memmove"]
    pub fn c_memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    #[link_name = "memchr"]
    pub fn c_memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

    // memory.c
    #[link_name = "xmalloc"]
    pub fn c_xmalloc(size: libc::size_t) -> *mut libc::c_void;
    #[link_name = "xfree"]
    fn c_xfree(ptr: *mut libc::c_void);
    #[link_name = "xcalloc"]
    pub fn c_xcalloc(count: libc::size_t, size: libc::size_t) -> *mut libc::c_void;
    #[link_name = "xrealloc"]
    fn c_xrealloc(ptr: *mut libc::c_void, size: libc::size_t) -> *mut libc::c_void;
    #[link_name = "xmallocz"]
    fn c_xmallocz(size: libc::size_t) -> *mut libc::c_void;
    #[link_name = "xmemdupz"]
    pub fn c_xmemdupz(data: *const libc::c_void, len: size_t) -> *mut libc::c_void;
    pub fn xstpcpy(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
    pub fn xstrlcpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        dsize: libc::size_t,
    ) -> libc::size_t;
    #[link_name = "xstrdup"]
    pub fn c_xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    pub fn xstrlcat(dst: *mut libc::c_char, src: *const libc::c_char, dsize: size_t) -> size_t;
}

pub fn var_size<T>(_: T) -> usize {
    mem::size_of::<T>()
}

pub unsafe fn XFREE_CLEAR<T>(ptr: &mut *mut T) {
    xfree(*ptr as *mut libc::c_void);
    *ptr = std::ptr::null_mut();
}

pub unsafe fn memcpy<T>(dest: *mut T, src: *const T, count: libc::size_t) -> *mut libc::c_void {
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

pub unsafe fn memcmp<T>(ptr1: *const T, ptr2: *const T, n: libc::size_t) -> libc::c_int {
    libc::memcmp(ptr1 as *const libc::c_void, ptr2 as *const libc::c_void, n)
}

pub unsafe fn memset<T>(dest: *mut T, c: libc::c_int, n: libc::size_t) -> *mut T {
    libc::memset(dest as *mut libc::c_void, c, n) as *mut T
}

pub unsafe fn xmalloc<S, T>(size: S) -> *mut T
where
    S: TryInto<libc::size_t>,
    <S as TryInto<libc::size_t>>::Error: std::fmt::Debug,
{
    c_xmalloc(size.try_into().unwrap()) as *mut T
}

pub unsafe fn xfree<T>(ptr: *mut T) {
    c_xfree(ptr as *mut libc::c_void);
}

pub unsafe fn xcalloc<T>(count: libc::size_t, size: libc::size_t) -> *mut T {
    c_xcalloc(count, size) as *mut T
}

pub unsafe fn xrealloc<S, T>(ptr: *mut S, size: libc::size_t) -> *mut T {
    c_xrealloc(ptr as *mut libc::c_void, size) as *mut T
}

pub unsafe fn xmallocz<T: Into<libc::size_t>, U>(size: T) -> *mut U {
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
