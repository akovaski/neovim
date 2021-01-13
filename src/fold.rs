use crate::*;

extern "C" {
    pub fn clearFolding(win: *mut win_T);
    pub fn foldUpdateAll(win: *mut win_T);
    pub fn cloneFoldGrowArray(from: *mut garray_T, to: *mut garray_T);
    pub fn deleteFoldRecurse(gap: *mut garray_T);
}
