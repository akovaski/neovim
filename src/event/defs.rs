use std::ptr;

extern "C" {
    #[doc(hidden)]
    pub fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}

#[macro_export]
macro_rules! c_assert {
    ( $x:expr ) => {
        if !$x {
            __assert_fail(
                std::ffi::CString::new(stringify!($x))
                    .expect("CString::new failed")
                    .as_ptr(),
                std::ffi::CString::new(file!())
                    .expect("CString::new failed")
                    .as_ptr(),
                line!(),
                std::ffi::CString::new("(a rust function)")
                    .expect("CString::new failed")
                    .as_ptr(),
            );
        }
    };
}

pub type argv_callback = Option<unsafe extern "C" fn(_: *mut *mut libc::c_void) -> ()>;

const EVENT_HANDLER_MAX_ARGC: usize = 10;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Event {
    pub handler: argv_callback,
    pub argv: [*mut libc::c_void; EVENT_HANDLER_MAX_ARGC],
}

#[inline]
pub unsafe extern "C" fn event_create(cb: argv_callback, argc: libc::c_int, args: ...) -> Event {
    c_assert!(argc <= EVENT_HANDLER_MAX_ARGC as i32);
    let mut event = Event {
        handler: None,
        argv: [ptr::null_mut(); EVENT_HANDLER_MAX_ARGC],
    };
    event.handler = cb;
    if argc != 0 {
        let mut args_0: ::std::ffi::VaListImpl;
        args_0 = args.clone();
        let mut i = 0;
        while i < argc {
            event.argv[i as usize] = args_0.as_va_list().arg::<*mut libc::c_void>();
            i += 1
        }
    }
    return event;
}
