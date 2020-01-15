use super::*;
use crate::lib::uv;
use libc;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Loop {
    pub uv: uv::uv_loop_t,
    pub events: *mut multiqueue::MultiQueue,
    pub thread_events: *mut multiqueue::MultiQueue,
    pub fast_events: *mut multiqueue::MultiQueue,
    pub children: *mut kl_WatcherPtr_t,
    pub children_watcher: uv::uv_signal_t,
    pub children_kill_timer: uv::uv_timer_t,
    pub poll_timer: uv::uv_timer_t,
    pub async_0: uv::uv_async_t,
    pub mutex: uv::uv_mutex_t,
    pub recursive: libc::c_int,
}

pub type WatcherPtr = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kl_WatcherPtr_t {
    pub head: *mut kl1_WatcherPtr,
    pub tail: *mut kl1_WatcherPtr,
    pub mp: *mut kmp_WatcherPtr_t,
    pub size: libc::size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kl1_WatcherPtr {
    pub data: WatcherPtr,
    pub next: *mut kl1_WatcherPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmp_WatcherPtr_t {
    pub cnt: libc::size_t,
    pub n: libc::size_t,
    pub max: libc::size_t,
    pub buf: *mut *mut kl1_WatcherPtr,
}
