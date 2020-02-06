use crate::*;

#[repr(C)]
pub struct SignalWatcher {
    pub uv: uv::uv_signal_t,
    pub data: *mut libc::c_void,
    pub cb: signal_cb,
    pub close_cb: signal_close_cb,
    pub events: *mut multiqueue::MultiQueue,
}

pub type signal_cb =
    Option<unsafe extern "C" fn(_: *mut SignalWatcher, _: libc::c_int, _: *mut libc::c_void) -> ()>;
pub type signal_close_cb =
    Option<unsafe extern "C" fn(_: *mut SignalWatcher, _: *mut libc::c_void) -> ()>;

#[no_mangle]
pub unsafe extern "C" fn signal_watcher_init(
    loop_0: &mut r#loop::Loop,
    watcher: &mut SignalWatcher,
    data: *mut libc::c_void,
) {
    uv::uv_signal_init(&mut loop_0.uv, &mut watcher.uv);
    watcher.uv.data = watcher as *mut _ as *mut libc::c_void;
    watcher.data = data;
    watcher.cb = None;
    watcher.events = loop_0.fast_events;
}

#[no_mangle]
pub unsafe extern "C" fn signal_watcher_start(
    watcher: &mut SignalWatcher,
    cb: signal_cb,
    signum: libc::c_int,
) {
    watcher.cb = cb;
    uv::uv_signal_start(&mut watcher.uv, Some(signal_watcher_cb), signum);
}

#[no_mangle]
pub unsafe extern "C" fn signal_watcher_stop(watcher: &mut SignalWatcher) {
    uv::uv_signal_stop(&mut watcher.uv);
}

#[no_mangle]
pub unsafe extern "C" fn signal_watcher_close(watcher: &mut SignalWatcher, cb: signal_close_cb) {
    watcher.close_cb = cb;
    uv::uv_close(&mut watcher.uv, Some(close_cb));
}

unsafe extern "C" fn signal_event(argv: *mut *mut libc::c_void) {
    let watcher = (*argv.offset(0)).cast::<SignalWatcher>().as_mut().unwrap();
    watcher.cb.expect("non-null function pointer")(watcher, watcher.uv.signum, watcher.data);
}

unsafe extern "C" fn signal_watcher_cb(handle: &mut uv::uv_signal_t, _signum: libc::c_int) {
    let watcher = handle.data.cast::<SignalWatcher>().as_mut().unwrap();
    CREATE_EVENT(
        watcher.events.as_mut(),
        signal_event,
        vargs!(watcher as *mut _),
    );
}

unsafe extern "C" fn close_cb(handle: &mut uv::uv_handle_t) {
    let watcher = handle.data.cast::<SignalWatcher>().as_mut().unwrap();
    if let Some(watcher_close_cb) = watcher.close_cb {
        watcher_close_cb(watcher, watcher.data);
    };
}
