extern "C" {
    pub fn xdl_diff(
        mf1: *mut mmfile_t,
        mf2: *mut mmfile_t,
        xpp: *const xpparam_t,
        xecfg: *const xdemitconf_t,
        ecb: *mut xdemitcb_t,
    ) -> libc::c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_mmfile {
    pub ptr: *mut libc::c_char,
    pub size: libc::c_long,
}
#[allow(non_camel_case_types)]
pub type mmfile_t = s_mmfile;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_mmbuffer {
    pub ptr: *mut libc::c_char,
    pub size: libc::c_long,
}
#[allow(non_camel_case_types)]
pub type mmbuffer_t = s_mmbuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_xpparam {
    pub flags: libc::c_ulong,
    pub anchors: *mut *mut libc::c_char,
    pub anchors_nr: libc::size_t,
}
#[allow(non_camel_case_types)]
pub type xpparam_t = s_xpparam;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_xdemitconf {
    pub ctxlen: libc::c_long,
    pub interhunkctxlen: libc::c_long,
    pub flags: libc::c_ulong,
    pub find_func: find_func_t,
    pub find_func_priv: *mut libc::c_void,
    pub hunk_func: xdl_emit_hunk_consume_func_t,
}
#[allow(non_camel_case_types)]
pub type xdemitconf_t = s_xdemitconf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_xdemitcb {
    pub priv_0: *mut libc::c_void,
    pub outf: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *mut mmbuffer_t,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
}
#[allow(non_camel_case_types)]
pub type xdemitcb_t = s_xdemitcb;
#[allow(non_camel_case_types)]
pub type find_func_t = Option<
    unsafe extern "C" fn(
        _: *const libc::c_char,
        _: libc::c_long,
        _: *mut libc::c_char,
        _: libc::c_long,
        _: *mut libc::c_void,
    ) -> libc::c_long,
>;
#[allow(non_camel_case_types)]
pub type xdl_emit_hunk_consume_func_t = Option<
    unsafe extern "C" fn(
        _: libc::c_long,
        _: libc::c_long,
        _: libc::c_long,
        _: libc::c_long,
        _: *mut libc::c_void,
    ) -> libc::c_int,
>;
