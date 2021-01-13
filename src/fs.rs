use crate::*;

extern "C" {
    pub fn os_getperm(name: *const libc::c_char) -> i32;
    pub fn os_fileid_equal(file_id_1: *const FileID, file_id_2: *const FileID) -> bool;
    pub fn os_fileid(path: *const libc::c_char, file_id: *mut FileID) -> bool;
}
