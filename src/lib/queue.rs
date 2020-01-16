#[derive(Copy, Clone)]
#[repr(C)]
pub struct QUEUE {
    pub next: *mut QUEUE,
    pub prev: *mut QUEUE,
}
