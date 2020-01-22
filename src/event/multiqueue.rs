use crate::*;

extern "C" {
    pub type MultiQueue;
    pub fn multiqueue_new_parent(put_cb: put_callback, data: *mut libc::c_void) -> *mut MultiQueue;
    pub fn multiqueue_new_child(parent: *mut MultiQueue) -> *mut MultiQueue;
    pub fn multiqueue_free(this: *mut MultiQueue);
    pub fn multiqueue_get(this: *mut MultiQueue) -> Event;
    pub fn multiqueue_put_event(this: *mut MultiQueue, event: Event);
    pub fn multiqueue_process_events(this: *mut MultiQueue);
    pub fn multiqueue_empty(this: *mut MultiQueue) -> bool;
    pub fn multiqueue_size(this: *mut MultiQueue) -> libc::size_t;
    pub fn multiqueue_purge_events(this: *mut MultiQueue);
}

pub type put_callback =
    Option<unsafe extern "C" fn(_: *mut MultiQueue, _: *mut libc::c_void) -> ()>;
pub unsafe fn multiqueue_put(this: *mut MultiQueue, cb: argv_callback, args: &[*mut libc::c_void]) {
    multiqueue_put_event(this, event_create(cb, args));
}
