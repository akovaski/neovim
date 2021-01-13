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
