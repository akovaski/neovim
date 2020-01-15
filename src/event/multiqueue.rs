use super::defs;

extern "C" {
    pub type MultiQueue;
    pub fn multiqueue_put_event(this: *mut MultiQueue, event: defs::Event);
}
