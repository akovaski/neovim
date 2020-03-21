#[derive(Copy, Clone)]
#[repr(C)]
pub struct VirtTextChunk {
    pub text: *mut libc::c_char,
    pub hl_id: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VirtText {
    pub size: libc::size_t,
    pub capacity: libc::size_t,
    pub items: *mut VirtTextChunk,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtmarkItem {
    pub ns_id: u64,
    pub mark_id: u64,
    pub hl_id: libc::c_int,
    pub virt_text: VirtText,
}
