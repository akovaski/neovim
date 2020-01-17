extern "C" {
    pub fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    pub fn xmalloc(size: libc::size_t) -> *mut libc::c_void;
    pub fn xfree(ptr: *mut libc::c_void);
    pub fn xcalloc(count: libc::size_t, size: libc::size_t) -> *mut libc::c_void;
}

#[allow(non_snake_case)]
pub unsafe fn XFREE_CLEAR<T>(ptr: &mut *mut T) {
    xfree(*ptr as *mut libc::c_void);
    *ptr = std::ptr::null_mut();
}
