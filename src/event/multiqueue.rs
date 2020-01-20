use crate::*;

extern "C" {
    pub type MultiQueue;
    pub fn multiqueue_put_event(this: *mut MultiQueue, event: Event);
    pub fn multiqueue_empty(this: *mut MultiQueue) -> bool;
}

pub unsafe fn multiqueue_put(this: *mut MultiQueue, cb: argv_callback, args: &[*mut libc::c_void]) {
    multiqueue_put_event(this, event_create(cb, args));
}
