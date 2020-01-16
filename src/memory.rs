extern "C" {
    pub fn xmalloc(size: libc::size_t) -> *mut libc::c_void;
    pub fn xfree(ptr: *mut libc::c_void);
}
