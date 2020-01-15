use libc;

extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}

pub type argv_callback = Option<unsafe extern "C" fn(_: *mut *mut libc::c_void) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Event {
    pub handler: argv_callback,
    pub argv: [*mut libc::c_void; 10],
}

#[inline]
pub unsafe extern "C" fn event_create(cb: argv_callback, argc: libc::c_int, args: ...) -> Event {
    if argc <= 10 as libc::c_int {
    } else {
        __assert_fail(
            b"argc <= EVENT_HANDLER_MAX_ARGC\x00" as *const u8 as *const libc::c_char,
            b"../src/nvim/event/defs.h\x00" as *const u8 as *const libc::c_char,
            32 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"Event event_create(argv_callback, int, ...)\x00",
            ))
            .as_ptr(),
        );
    }
    let mut event = Event {
        handler: None,
        argv: [0 as *mut libc::c_void; 10],
    };
    if argc <= 10 as libc::c_int {
    } else {
        __assert_fail(
            b"argc <= EVENT_HANDLER_MAX_ARGC\x00" as *const u8 as *const libc::c_char,
            b"../src/nvim/event/defs.h\x00" as *const u8 as *const libc::c_char,
            34 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"Event event_create(argv_callback, int, ...)\x00",
            ))
            .as_ptr(),
        );
    }
    event.handler = cb;
    if argc != 0 {
        let mut args_0: ::std::ffi::VaListImpl;
        args_0 = args.clone();
        let mut i = 0 as libc::c_int;
        while i < argc {
            event.argv[i as usize] = args_0.as_va_list().arg::<*mut libc::c_void>();
            i += 1
        }
    }
    return event;
}
