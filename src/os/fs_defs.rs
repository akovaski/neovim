#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileID {
    pub inode: u64,
    pub device_id: u64,
}
