extern "C" {
    // C std
    #[link_name = "memcpy"]
    pub fn c_memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::size_t,
    ) -> *mut libc::c_void;

    // memory.c
    pub fn xmalloc(size: libc::size_t) -> *mut libc::c_void;
    #[link_name = "xfree"]
    fn c_xfree(ptr: *mut libc::c_void);
    pub fn xcalloc(count: libc::size_t, size: libc::size_t) -> *mut libc::c_void;
    #[link_name = "xrealloc"]
    fn c_xrealloc(ptr: *mut libc::c_void, size: libc::size_t) -> *mut libc::c_void;
    #[link_name = "xmallocz"]
    fn c_xmallocz(size: libc::size_t) -> *mut libc::c_void;
    pub fn xstpcpy(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
    #[link_name = "xstrdup"]
    fn c_xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
}

#[allow(non_snake_case)]
pub unsafe fn XFREE_CLEAR<T>(ptr: &mut *mut T) {
    xfree(*ptr as *mut libc::c_void);
    *ptr = std::ptr::null_mut();
}

pub unsafe fn memcpy<T, U>(dest: *mut T, src: *const U, count: libc::size_t) -> *mut libc::c_void {
    c_memcpy(dest as *mut libc::c_void, src as *const libc::c_void, count)
}

pub unsafe fn memset<T>(dest: *mut T, c: libc::c_int, n: libc::size_t) -> *mut T {
    libc::memset(dest as *mut libc::c_void, c, n) as *mut T
}

pub unsafe fn xfree<T>(ptr: *mut T) {
    c_xfree(ptr as *mut libc::c_void);
}

pub unsafe fn xrealloc<S, T>(ptr: *mut S, size: libc::size_t) -> *mut T {
    c_xrealloc(ptr as *mut libc::c_void, size) as *mut T
}

pub unsafe fn xmallocz<T: Into<libc::size_t>, U>(size: T) -> *mut U {
    c_xmallocz(size.into()) as *mut U
}

pub unsafe fn xstrdup(str: &str) -> *mut libc::c_char {
    let str = std::ffi::CString::new(str)
        .expect("CString::new failed")
        .as_ptr();
    c_xstrdup(str)
}
