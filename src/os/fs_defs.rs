use crate::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileID {
    pub inode: u64,
    pub device_id: u64,
}

impl Default for FileID {
    fn default() -> Self {
        FileID {
            inode: 0,
            device_id: 0,
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileInfo {
    pub stat: uv_stat_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Directory {
    pub request: uv_fs_t,
    pub ent: uv_dirent_t,
}
