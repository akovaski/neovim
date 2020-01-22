use crate::*;

pub type time_cb = Option<unsafe extern "C" fn(_: *mut TimeWatcher, _: *mut libc::c_void) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TimeWatcher {
    pub uv: uv_timer_t,
    pub data: *mut libc::c_void,
    pub cb: time_cb,
    pub close_cb: time_cb,
    pub events: *mut MultiQueue,
    pub blockable: bool,
}

#[no_mangle]
pub unsafe extern "C" fn time_watcher_init(
    loop_0: *mut Loop,
    mut watcher: *mut TimeWatcher,
    data: *mut libc::c_void,
) {
    uv_timer_init(&mut (*loop_0).uv, &mut (*watcher).uv);
    (*watcher).uv.data = watcher as *mut libc::c_void;
    (*watcher).data = data;
    (*watcher).events = (*loop_0).fast_events;
    (*watcher).blockable = false;
}

#[no_mangle]
pub unsafe extern "C" fn time_watcher_start(
    mut watcher: *mut TimeWatcher,
    cb: time_cb,
    timeout: u64,
    repeat: u64,
) {
    (*watcher).cb = cb;
    uv_timer_start(&mut (*watcher).uv, Some(time_watcher_cb), timeout, repeat);
}

#[no_mangle]
pub unsafe extern "C" fn time_watcher_stop(watcher: *mut TimeWatcher) {
    uv_timer_stop(&mut (*watcher).uv);
}

#[no_mangle]
pub unsafe extern "C" fn time_watcher_close(mut watcher: *mut TimeWatcher, cb: time_cb) {
    (*watcher).close_cb = cb;
    uv_close(&mut (*watcher).uv, Some(close_cb));
}

unsafe extern "C" fn time_event(argv: *mut *mut libc::c_void) {
    let watcher: *mut TimeWatcher = *argv as *mut TimeWatcher;
    (*watcher).cb.expect("non-null function pointer")(watcher, (*watcher).data);
}

unsafe extern "C" fn time_watcher_cb(handle: *mut uv_timer_t) {
    let watcher: *mut TimeWatcher = (*handle).data as *mut TimeWatcher;
    if (*watcher).blockable && !multiqueue_empty((*watcher).events) {
        // the timer blocked and there already is an unprocessed event waiting
        return;
    }
    CREATE_EVENT((*watcher).events, time_event, vargs!(watcher));
}
unsafe extern "C" fn close_event(argv: *mut *mut libc::c_void) {
    let watcher: *mut TimeWatcher = *argv as *mut TimeWatcher;
    (*watcher).close_cb.expect("non-null function pointer")(watcher, (*watcher).data);
}
unsafe extern "C" fn close_cb(handle: *mut uv_handle_t) {
    let watcher: *mut TimeWatcher = (*handle).data as *mut TimeWatcher;
    if (*watcher).close_cb.is_some() {
        CREATE_EVENT((*watcher).events, close_event, vargs!(watcher));
    }
}
