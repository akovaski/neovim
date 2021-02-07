use crate::*;

extern "C" {
    pub fn fs_init();
    pub fn os_chdir(path: *const i8) -> i32;
    pub fn os_dirname(buf: *mut u8, len: size_t) -> i32;
    pub fn os_isrealdir(name: *const i8) -> bool;
    pub fn os_isdir(name: *const u8) -> bool;
    pub fn os_isdir_executable(name: *const i8) -> bool;
    pub fn os_nodetype(name: *const i8) -> i32;
    pub fn os_exepath(buffer: *mut i8, size: *mut size_t) -> i32;
    pub fn os_can_exe(name: *const i8, abspath: *mut *mut i8, use_path: bool) -> bool;
    pub fn os_open(path: *const i8, flags: i32, mode: i32) -> i32;
    pub fn os_fopen(path: *const i8, flags: *const i8) -> *mut FILE;
    pub fn os_set_cloexec(fd: i32) -> i32;
    pub fn os_close(fd: i32) -> i32;
    pub fn os_dup(fd: i32) -> i32;
    pub fn os_read(
        fd: i32,
        ret_eof: *mut bool,
        ret_buf: *mut i8,
        size: size_t,
        non_blocking: bool,
    ) -> ptrdiff_t;
    pub fn os_readv(
        fd: i32,
        ret_eof: *mut bool,
        iov: *mut iovec,
        iov_size: size_t,
        non_blocking: bool,
    ) -> ptrdiff_t;
    pub fn os_write(fd: i32, buf: *const i8, size: size_t, non_blocking: bool) -> ptrdiff_t;
    pub fn os_copy(path: *const i8, new_path: *const i8, flags: i32) -> i32;
    pub fn os_fsync(fd: i32) -> i32;
    pub fn os_getperm(name: *const i8) -> i32;
    pub fn os_setperm(name: *const i8, perm: i32) -> i32;
    pub fn os_chown(path: *const i8, owner: uv_uid_t, group: uv_gid_t) -> i32;
    pub fn os_fchown(fd: i32, owner: uv_uid_t, group: uv_gid_t) -> i32;
    pub fn os_path_exists(path: *const u8) -> bool;
    pub fn os_file_settime(path: *const i8, atime: libc::c_double, mtime: libc::c_double) -> i32;
    pub fn os_file_is_readable(name: *const i8) -> bool;
    pub fn os_file_is_writable(name: *const i8) -> i32;
    pub fn os_rename(path: *const u8, new_path: *const u8) -> i32;
    pub fn os_mkdir(path: *const i8, mode: i32) -> i32;
    pub fn os_mkdir_recurse(dir: *const i8, mode: i32, failed_dir: *mut *mut i8) -> i32;
    pub fn os_mkdtemp(template: *const i8, path: *mut i8) -> i32;
    pub fn os_rmdir(path: *const i8) -> i32;
    pub fn os_scandir(dir: *mut Directory, path: *const i8) -> bool;
    pub fn os_scandir_next(dir: *mut Directory) -> *const i8;
    pub fn os_closedir(dir: *mut Directory);
    pub fn os_remove(path: *const i8) -> i32;
    pub fn os_fileinfo(path: *const i8, file_info: *mut FileInfo) -> bool;
    pub fn os_fileinfo_link(path: *const i8, file_info: *mut FileInfo) -> bool;
    pub fn os_fileinfo_fd(file_descriptor: i32, file_info: *mut FileInfo) -> bool;
    pub fn os_fileinfo_id_equal(file_info_1: *const FileInfo, file_info_2: *const FileInfo)
        -> bool;
    pub fn os_fileinfo_id(file_info: *const FileInfo, file_id: *mut FileID);
    pub fn os_fileinfo_inode(file_info: *const FileInfo) -> u64;
    pub fn os_fileinfo_size(file_info: *const FileInfo) -> u64;
    pub fn os_fileinfo_hardlinks(file_info: *const FileInfo) -> u64;
    pub fn os_fileinfo_blocksize(file_info: *const FileInfo) -> u64;
    pub fn os_fileid(path: *const i8, file_id: *mut FileID) -> bool;
    pub fn os_fileid_equal(file_id_1: *const FileID, file_id_2: *const FileID) -> bool;
    pub fn os_fileid_equal_fileinfo(file_id: *const FileID, file_info: *const FileInfo) -> bool;
}
