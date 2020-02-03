use crate::*;
use std::ptr;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream {
    pub closed: bool,
    pub did_eof: bool,
    pub uv: stream_uv_union,
    pub uvstream: uv_stream_mut,
    pub uvbuf: uv_buf_t,
    pub buffer: *mut RBuffer<Stream>,
    pub fd: uv_file,
    pub read_cb: stream_read_cb,
    pub write_cb: stream_write_cb,
    pub cb_data: *mut libc::c_void,
    pub close_cb: stream_close_cb,
    pub internal_close_cb: stream_close_cb,
    pub close_cb_data: *mut libc::c_void,
    pub internal_data: *mut libc::c_void,
    pub fpos: libc::size_t,
    pub curmem: libc::size_t,
    pub maxmem: libc::size_t,
    pub pending_reqs: libc::size_t,
    pub num_bytes: libc::size_t,
    pub events: *mut MultiQueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union stream_uv_union {
    pub pipe: uv_pipe_t,
    pub tcp: uv_tcp_t,
    pub idle: uv_idle_t,
    #[cfg(windows)]
    pub tty: uv_tty_t,
}

/// Type of function called when the Stream buffer is filled with data
///
/// @param stream The Stream instance
/// @param buf The associated RBuffer instance
/// @param count Number of bytes that was read.
/// @param data User-defined data
/// @param eof If the stream reached EOF.
pub type stream_read_cb = Option<
    unsafe extern "C" fn(
        _: *mut Stream,
        _: *mut RBuffer<Stream>,
        _: libc::size_t,
        _: *mut libc::c_void,
        _: bool,
    ) -> (),
>;

/// Type of function called when the Stream has information about a write
/// request.
///
/// @param stream The Stream instance
/// @param data User-defined data
/// @param status 0 on success, anything else indicates failure
pub type stream_write_cb =
    Option<unsafe extern "C" fn(_: *mut Stream, _: *mut libc::c_void, _: libc::c_int) -> ()>;
pub type stream_close_cb = Option<unsafe extern "C" fn(_: *mut Stream, _: *mut libc::c_void) -> ()>;

/// Sets the stream associated with `fd` to "blocking" mode.
///
/// @return `0` on success, or libuv error code on failure.
#[no_mangle]
pub unsafe extern "C" fn stream_set_blocking(fd: libc::c_int, blocking: bool) -> libc::c_int {
    // Private loop to avoid conflict with existing watcher(s):
    //    uv__io_stop: Assertion `loop->watchers[w->fd] == w' failed.
    let mut loop_0: uv_loop_t = std::mem::zeroed();
    let mut stream: uv_pipe_t = std::mem::zeroed();
    uv_loop_init(&mut loop_0);
    uv_pipe_init(&mut loop_0, &mut stream, 0);
    uv_pipe_open(&mut stream, fd);
    let retval: libc::c_int = uv_stream_set_blocking(&mut stream, blocking as libc::c_int);
    uv_close(&mut stream, None);
    uv_run(&mut loop_0, UV_RUN_NOWAIT); // not necessary, but couldn't hurt.
    uv_loop_close(&mut loop_0);
    return retval;
}

#[no_mangle]
pub unsafe extern "C" fn stream_init(
    loop_0: *mut Loop,
    mut stream: *mut Stream,
    fd: libc::c_int,
    uvstream: uv_stream_mut,
) {
    (*stream).uvstream = uvstream;

    if fd >= 0 {
        let type_0: uv_handle_type = uv_guess_handle(fd);
        (*stream).fd = fd;
        if type_0 == UV_FILE {
            // Non-blocking file reads are simulated with an idle handle that reads in
            // chunks of the ring buffer size, giving time for other events to be
            // processed between reads.
            uv_idle_init(&mut (*loop_0).uv, &mut (*stream).uv.idle);
            (*stream).uv.idle.data = stream as *mut libc::c_void
        } else {
            c_assert!(type_0 == UV_NAMED_PIPE || type_0 == UV_TTY);

            #[cfg(windows)]
            unimplemented!();

            uv_pipe_init(&mut (*loop_0).uv, &mut (*stream).uv.pipe, 0);
            uv_pipe_open(&mut (*stream).uv.pipe, fd);
            (*stream).uvstream = (&mut (*stream).uv.pipe).into();
        }
    }

    if !(*stream).uvstream.is_null() {
        (*(*stream).uvstream.stream).data = stream as *mut libc::c_void
    }

    (*stream).internal_data = ptr::null_mut();
    (*stream).fpos = 0;
    (*stream).curmem = 0;
    (*stream).maxmem = 0;
    (*stream).pending_reqs = 0;
    (*stream).read_cb = None;
    (*stream).write_cb = None;
    (*stream).close_cb = None;
    (*stream).internal_close_cb = None;
    (*stream).closed = false;
    (*stream).buffer = ptr::null_mut();
    (*stream).events = ptr::null_mut();
    (*stream).num_bytes = 0;
}

#[no_mangle]
pub unsafe extern "C" fn stream_close(
    mut stream: *mut Stream,
    on_stream_close: stream_close_cb,
    data: *mut libc::c_void,
) {
    c_assert!(!(*stream).closed);
    DLOG!("closing Stream: %p", stream);
    (*stream).closed = true;
    (*stream).close_cb = on_stream_close;
    (*stream).close_cb_data = data;

    #[cfg(windows)]
    unimplemented!();

    if (*stream).pending_reqs == 0 {
        stream_close_handle(stream);
    };
}

#[no_mangle]
pub unsafe extern "C" fn stream_may_close(stream: *mut Stream) {
    if !(*stream).closed {
        stream_close(stream, None, ptr::null_mut());
    };
}

#[no_mangle]
pub unsafe extern "C" fn stream_close_handle(stream: *mut Stream) {
    if !(*stream).uvstream.is_null() {
        if uv_stream_get_write_queue_size((*stream).uvstream) > 0 {
            WLOG!(
                "closed Stream (%p) with %zu unwritten bytes",
                stream,
                uv_stream_get_write_queue_size((*stream).uvstream)
            );
        }
        uv_close((*stream).uvstream.stream, Some(close_cb));
    } else {
        uv_close(&mut (*stream).uv.idle, Some(close_cb));
    };
}

unsafe extern "C" fn close_cb(handle: &mut uv_handle_t) {
    let stream: *mut Stream = handle.data as *mut Stream;
    if !(*stream).buffer.is_null() {
        rbuffer_free((*stream).buffer);
    }
    if let Some(close_cb) = (*stream).close_cb {
        close_cb(stream, (*stream).close_cb_data);
    }
    if let Some(internal_close_cb) = (*stream).internal_close_cb {
        internal_close_cb(stream, (*stream).internal_data);
    };
}
