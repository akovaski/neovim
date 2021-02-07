use crate::*;

extern "C" {
    pub fn stdpaths_get_xdg_var(idx: XDGVarType) -> *mut i8;
    pub fn get_xdg_home(idx: XDGVarType) -> *mut i8;
    pub fn stdpaths_user_conf_subpath(fname: *const i8) -> *mut i8;
    pub fn stdpaths_user_data_subpath(
        fname: *const i8,
        trailing_pathseps: size_t,
        escape_commas: bool,
    ) -> *mut i8;
}
