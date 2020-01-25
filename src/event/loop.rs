use crate::*;
use std::ptr;

pub type WatcherPtr = *mut libc::c_void;
#[repr(C)]
pub struct Loop {
    pub uv: uv_loop_t,
    pub events: *mut MultiQueue,
    pub thread_events: *mut MultiQueue,
    // Immediate events:
    //    "Processed after exiting uv_run() (to avoid recursion), but before
    //    returning from loop_poll_events()." 502aee690c98
    // Practical consequence (for main_loop): these events are processed by
    //    state_enter()..os_inchar()
    // whereas "regular" events (main_loop.events) are processed by
    //    state_enter()..VimState.execute()
    // But state_enter()..os_inchar() can be "too early" if you want the event
    // to trigger UI updates and other user-activity-related side-effects.
    pub fast_events: *mut MultiQueue,

    // used by process/job-control subsystem
    pub children: *mut kl_t<WatcherPtr>,
    pub children_watcher: uv_signal_t,
    pub children_kill_timer: uv_timer_t,

    // generic timer, used by loop_poll_events()
    pub poll_timer: uv_timer_t,

    pub async_0: uv_async_t,
    pub mutex: uv_mutex_t,
    pub recursive: libc::c_int,
}
pub unsafe fn CREATE_EVENT(
    multiqueue: Option<&mut MultiQueue>,
    handler: valid_argv_callback,
    args: &[*mut libc::c_void],
) {
    if multiqueue.is_some() {
        multiqueue_put(multiqueue, Some(handler), args);
    } else {
        handler(args.as_ptr() as *mut *mut libc::c_void);
    };
}

// Poll for events until a condition or timeout
pub unsafe fn LOOP_PROCESS_EVENTS_UNTIL(
    loop_0: &mut Loop,
    multiqueue: &mut Option<MultiQueue>,
    timeout: libc::c_int,
    condition: fn() -> bool,
) {
    let mut remaining = timeout;
    let mut before: u64 = if remaining > 0 { os_hrtime() } else { 0 };
    while !condition() {
        LOOP_PROCESS_EVENTS(loop_0, multiqueue, remaining);
        if remaining == 0 {
            break;
        } else if remaining > 0 {
            let now: u64 = os_hrtime();
            remaining -= ((now - before) / 1000000) as i32;
            before = now;
            if remaining <= 0 {
                break;
            }
        }
    }
}

pub unsafe fn LOOP_PROCESS_EVENTS(
    loop_0: &mut Loop,
    multiqueue: &mut Option<MultiQueue>,
    timeout: libc::c_int,
) {
    if let Some(multiqueue) = multiqueue {
        if !multiqueue_empty(Some(multiqueue)) {
            multiqueue_process_events(Some(multiqueue));
        }
    } else {
        loop_poll_events(loop_0, timeout);
    }
}

#[no_mangle]
pub unsafe extern "C" fn loop_init(mut loop_0: *mut Loop, _data: *mut libc::c_void) {
    uv_loop_init(&mut (*loop_0).uv);
    (*loop_0).recursive = 0;
    (*loop_0).uv.data = loop_0 as *mut libc::c_void;
    (*loop_0).children = kl_init();
    (*loop_0).events = multiqueue_new_parent(Some(loop_on_put), loop_0 as *mut libc::c_void);
    (*loop_0).fast_events = multiqueue_new_child(&mut *(*loop_0).events);
    (*loop_0).thread_events = multiqueue_new_parent(None, ptr::null_mut());
    uv_mutex_init(&mut (*loop_0).mutex);
    uv_async_init(&mut (*loop_0).uv, &mut (*loop_0).async_0, Some(async_cb));
    uv_signal_init(&mut (*loop_0).uv, &mut (*loop_0).children_watcher);
    uv_timer_init(&mut (*loop_0).uv, &mut (*loop_0).children_kill_timer);
    uv_timer_init(&mut (*loop_0).uv, &mut (*loop_0).poll_timer);
    (*loop_0).poll_timer.data = xmalloc(std::mem::size_of::<bool>()); // "timeout expired" flag
}

/// Processes one `Loop.uv` event (at most).
/// Processes all `Loop.fast_events` events.
/// Does NOT process `Loop.events`, that is an application-specific decision.
///
/// @param loop
/// @param ms   0: non-blocking poll.
///            >0: timeout after `ms`.
///            <0: wait forever.
/// @returns true if `ms` timeout was reached
#[no_mangle]
pub unsafe extern "C" fn loop_poll_events(mut loop_0: *mut Loop, ms: libc::c_int) -> bool {
    let fresh2 = (*loop_0).recursive;
    (*loop_0).recursive += 1;
    if fresh2 != 0 {
        abort(); // Should not re-enter uv_run
    }

    let mut mode: uv_run_mode = UV_RUN_ONCE;
    let mut timeout_expired: bool = false;

    if ms > 0 {
        *((*loop_0).poll_timer.data as *mut bool) = false; // reset "timeout expired" flag
                                                           // Dummy timer to ensure UV_RUN_ONCE does not block indefinitely for I/O.
        uv_timer_start(
            &mut (*loop_0).poll_timer,
            Some(timer_cb),
            ms as u64,
            ms as u64,
        );
    } else if ms == 0 {
        // For ms == 0, do a non-blocking event poll.
        mode = UV_RUN_NOWAIT
    }

    uv_run(&mut (*loop_0).uv, mode);
    if ms > 0 {
        timeout_expired = *((*loop_0).poll_timer.data as *mut bool);
        uv_timer_stop(&mut (*loop_0).poll_timer);
    }
    (*loop_0).recursive -= 1; // Can re-enter uv_run now
    multiqueue_process_events((*loop_0).fast_events.as_mut());
    return timeout_expired;
}

/// Schedules a fast event from another thread.
///
/// @note Event is queued into `fast_events`, which is processed outside of the
///       primary `events` queue by loop_poll_events(). For `main_loop`, that
///       means `fast_events` is NOT processed in an "editor mode"
///       (VimState.execute), so redraw and other side-effects are likely to be
///       skipped.
/// @see loop_schedule_deferred
#[no_mangle]
pub unsafe extern "C" fn loop_schedule_fast(loop_0: *mut Loop, event: Event) {
    uv_mutex_lock(&mut (*loop_0).mutex);
    multiqueue_put_event((*loop_0).thread_events.as_mut(), event);
    uv_async_send(&mut (*loop_0).async_0);
    uv_mutex_unlock(&mut (*loop_0).mutex);
}

/// Schedules an event from another thread. Unlike loop_schedule_fast(), the
/// event is forwarded to `Loop.events`, instead of being processed immediately.
///
/// @see loop_schedule_fast
#[no_mangle]
pub unsafe extern "C" fn loop_schedule_deferred(loop_0: *mut Loop, event: Event) {
    let eventp: *mut Event = xmalloc(std::mem::size_of::<Event>());
    *eventp = event;
    loop_schedule_fast(
        loop_0,
        event_create(Some(loop_deferred_event), vargs!(loop_0, eventp)),
    );
}
unsafe extern "C" fn loop_deferred_event(argv: *mut *mut libc::c_void) {
    let loop_0: *mut Loop = *argv.offset(0) as *mut Loop;
    let eventp: *mut Event = *argv.offset(1) as *mut Event;
    multiqueue_put_event((*loop_0).events.as_mut(), *eventp);
    xfree(eventp);
}

#[no_mangle]
pub unsafe extern "C" fn loop_on_put(_queue: Option<&mut MultiQueue>, data: *mut libc::c_void) {
    let loop_0: *mut Loop = data as *mut Loop;
    // Sometimes libuv will run pending callbacks (timer for example) before
    // blocking for a poll. If this happens and the callback pushes a event to one
    // of the queues, the event would only be processed after the poll
    // returns (user hits a key for example). To avoid this scenario, we call
    // uv_stop when a event is enqueued.
    uv_stop(&mut (*loop_0).uv);
}

/// @returns false if the loop could not be closed gracefully
#[no_mangle]
pub unsafe extern "C" fn loop_close(loop_0: *mut Loop, wait: bool) -> bool {
    let mut rv: bool = true;
    uv_mutex_destroy(&mut (*loop_0).mutex);
    uv_close(&mut (*loop_0).children_watcher, None);
    uv_close(&mut (*loop_0).children_kill_timer, None);
    uv_close(&mut (*loop_0).poll_timer, Some(timer_close_cb));
    uv_close(&mut (*loop_0).async_0, None);
    let start: u64 = if wait { os_hrtime() } else { 0 };
    loop {
        uv_run(
            &mut (*loop_0).uv,
            if wait { UV_RUN_DEFAULT } else { UV_RUN_NOWAIT },
        );
        if uv_loop_close(&mut (*loop_0).uv) == 0 || !wait {
            break;
        }
        if (os_hrtime() - start) >= (2 * 1000000000) {
            // Some libuv resource was not correctly deref'd. Log and bail.
            rv = false;
            ELOG!("uv_loop_close() hang?");
            log_uv_handles(&mut (*loop_0).uv as *mut uv_loop_t as *mut libc::c_void);
            break;
        }
    }
    multiqueue_free((*loop_0).fast_events);
    multiqueue_free((*loop_0).thread_events);
    multiqueue_free((*loop_0).events);
    kl_destroy((*loop_0).children);
    return rv;
}

#[no_mangle]
pub unsafe extern "C" fn loop_purge(loop_0: *mut Loop) {
    uv_mutex_lock(&mut (*loop_0).mutex);
    multiqueue_purge_events((*loop_0).thread_events.as_mut());
    multiqueue_purge_events((*loop_0).fast_events.as_mut());
    uv_mutex_unlock(&mut (*loop_0).mutex);
}

#[no_mangle]
pub unsafe extern "C" fn loop_size(loop_0: *mut Loop) -> libc::size_t {
    uv_mutex_lock(&mut (*loop_0).mutex);
    let rv: libc::size_t = multiqueue_size(&mut *(*loop_0).thread_events);
    uv_mutex_unlock(&mut (*loop_0).mutex);
    return rv;
}

unsafe extern "C" fn async_cb(handle: *mut uv_async_t) {
    let l: *mut Loop = (*(*handle).loop_0).data as *mut Loop;
    uv_mutex_lock(&mut (*l).mutex);
    // Flush thread_events to fast_events for processing on main loop.
    while !multiqueue_empty((*l).thread_events.as_mut()) {
        let ev: Event = multiqueue_get((*l).thread_events.as_mut());
        multiqueue_put_event((*l).fast_events.as_mut(), ev);
    }
    uv_mutex_unlock(&mut (*l).mutex);
}

unsafe extern "C" fn timer_cb(handle: *mut uv_timer_t) {
    let timeout_expired: *mut bool = (*handle).data as *mut bool;
    *timeout_expired = true;
}

unsafe extern "C" fn timer_close_cb(handle: *mut uv_handle_t) {
    xfree((*handle).data);
}
