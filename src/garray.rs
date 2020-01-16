#[derive(Copy, Clone)]
#[repr(C)]
pub struct garray_T {
    pub ga_len: libc::c_int,
    pub ga_maxlen: libc::c_int,
    pub ga_itemsize: libc::c_int,
    pub ga_growsize: libc::c_int,
    pub ga_data: *mut libc::c_void,
}
