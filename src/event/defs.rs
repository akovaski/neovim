use std::ptr;

pub type valid_argv_callback = unsafe extern "C" fn(_: *mut *mut libc::c_void) -> ();
pub type argv_callback = Option<valid_argv_callback>;

const EVENT_HANDLER_MAX_ARGC: usize = 10;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Event {
    pub handler: argv_callback,
    pub argv: [*mut libc::c_void; EVENT_HANDLER_MAX_ARGC],
}

#[inline]
pub unsafe fn event_create(cb: argv_callback, args: &[*mut libc::c_void]) -> Event {
    c_assert!(args.len() <= EVENT_HANDLER_MAX_ARGC);
    let mut event = Event {
        handler: cb,
        argv: [ptr::null_mut(); EVENT_HANDLER_MAX_ARGC],
    };
    for (i, &arg) in args.iter().enumerate() {
        event.argv[i] = arg;
    }
    return event;
}
