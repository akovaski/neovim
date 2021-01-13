use crate::*;

extern "C" {
    pub fn uc_clear(gap: *mut garray_T);
    pub fn tabpage_new();
    pub fn post_chdir(scope: CdScope, trigger_dirchanged: bool);
    pub fn alist_unlink(al: *mut alist_T);
}
