#[allow(dead_code)]
unsafe extern "C" fn xdl_diff(
    mf1: *mut xdiff::mmfile_t,
    mf2: *mut xdiff::mmfile_t,
    xpp: *const xdiff::xpparam_t,
    xecfg: *const xdiff::xdemitconf_t,
    ecb: *mut xdiff::xdemitcb_t,
) -> libc::c_int {
    xdiff::xdl_diff(mf1, mf2, xpp, xecfg, ecb)
}
