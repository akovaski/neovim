use crate::*;
use std::ptr;

#[no_mangle]
pub unsafe extern "C" fn rstream_init_fd(
    loop_0: *mut Loop,
    stream: &mut Stream,
    fd: libc::c_int,
    bufsize: libc::size_t,
) {
    stream_init(loop_0, stream, fd, ().into());
    rstream_init(stream, bufsize);
}

#[no_mangle]
pub unsafe extern "C" fn rstream_init_stream(
    stream: &mut Stream,
    uvstream: uv_stream_mut,
    bufsize: libc::size_t,
) {
    stream_init(ptr::null_mut(), stream, -1, uvstream);
    rstream_init(stream, bufsize);
}

#[no_mangle]
pub unsafe extern "C" fn rstream_init(mut stream: &mut Stream, bufsize: libc::size_t) {
    (*stream).buffer = rbuffer_new(bufsize);
    (*(*stream).buffer).data = stream;
    (*(*stream).buffer).full_cb = Some(on_rbuffer_full);
    (*(*stream).buffer).nonfull_cb = Some(on_rbuffer_nonfull);
}

/// Starts watching for events from a `Stream` instance.
///
/// @param stream The `Stream` instance
#[no_mangle]
pub unsafe extern "C" fn rstream_start(
    mut stream: &mut Stream,
    cb: stream_read_cb,
    data: *mut libc::c_void,
) {
    (*stream).read_cb = cb;
    (*stream).cb_data = data;
    if !(*stream).uvstream.is_null() {
        uv_read_start((*stream).uvstream, Some(alloc_cb), Some(read_cb));
    } else {
        uv_idle_start(&mut (*stream).uv.idle, Some(fread_idle_cb));
    };
}

/// Stops watching for events from a `Stream` instance.
///
/// @param stream The `Stream` instance
#[no_mangle]
pub unsafe extern "C" fn rstream_stop(stream: *mut Stream) {
    if !(*stream).uvstream.is_null() {
        uv_read_stop((*stream).uvstream);
    } else {
        uv_idle_stop(&mut (*stream).uv.idle);
    };
}

unsafe extern "C" fn on_rbuffer_full(_buf: *mut RBuffer<Stream>, data: *mut Stream) {
    rstream_stop(data);
}

unsafe extern "C" fn on_rbuffer_nonfull(_buf: *mut RBuffer<Stream>, data: *mut Stream) {
    let stream: &mut Stream = data.as_mut().unwrap();
    assert!((*stream).read_cb.is_some());
    rstream_start(stream, (*stream).read_cb, (*stream).cb_data);
}

// Callbacks used by libuv

/// Called by libuv to allocate memory for reading.
unsafe extern "C" fn alloc_cb(
    handle: *mut uv_handle_t,
    _suggested: libc::size_t,
    mut buf: *mut uv_buf_t,
) {
    let stream: *mut Stream = (*handle).data as *mut Stream;
    let mut write_count: libc::size_t = 0;
    (*buf).base = rbuffer_write_ptr((*stream).buffer, &mut write_count);
    (*buf).len = write_count;
}

/// Callback invoked by libuv after it copies the data into the buffer provided
/// by `alloc_cb`. This is also called on EOF or when `alloc_cb` returns a
/// 0-length buffer.
unsafe extern "C" fn read_cb(uvstream: uv_stream_mut, cnt: libc::ssize_t, _buf: *const uv_buf_t) {
    let mut stream: *mut Stream = (*uvstream.stream).data as *mut Stream;

    if cnt <= 0 {
        // cnt == 0 means libuv asked for a buffer and decided it wasn't needed:
        // http://docs.libuv.org/en/latest/stream.html#c.uv_read_start.
        //
        // We don't need to do anything with the RBuffer because the next call
        // to `alloc_cb` will return the same unused pointer(`rbuffer_produced`
        // won't be called)
        if cnt == UV_ENOBUFS as isize || cnt == 0 {
            return;
        } else if cnt == UV_EOF as isize && (*uvstream.stream).type_0 == UV_TTY {
            // The TTY driver might signal EOF without closing the stream
            invoke_read_cb(stream, 0, true);
        } else {
            DLOG!(
                "closing Stream (%p): %s (%s)",
                stream,
                uv_err_name(cnt as i32),
                uv_strerror(cnt as i32)
            );
            // Read error or EOF, either way stop the stream and invoke the callback
            // with eof == true
            uv_read_stop(uvstream);
            invoke_read_cb(stream, 0, true);
        }
        return;
    }

    // at this point we're sure that cnt is positive, no error occurred
    let nread: libc::size_t = cnt as libc::size_t;
    (*stream).num_bytes = ((*stream).num_bytes).wrapping_add(nread);
    // Data was already written, so all we need is to update 'wpos' to reflect
    // the space actually used in the buffer.
    rbuffer_produced((*stream).buffer, nread);
    invoke_read_cb(stream, nread, false);
}

/// Called by the by the 'idle' handle to emulate a reading event
///
/// Idle callbacks are invoked once per event loop:
///  - to perform some very low priority activity.
///  - to keep the loop "alive" (so there is always an event to process)
unsafe extern "C" fn fread_idle_cb(handle: *mut uv_idle_t) {
    let mut req: uv_fs_t = std::mem::zeroed();
    let mut stream: *mut Stream = (*handle).data as *mut Stream;

    let mut write_count: libc::size_t = 0;
    (*stream).uvbuf.base = rbuffer_write_ptr((*stream).buffer, &mut write_count);
    (*stream).uvbuf.len = write_count;

    // the offset argument to uv_fs_read is i64, could someone really try
    // to read more than 9 quintillion (9e18) bytes?
    // upcast is meant to avoid tautological condition warning on 32 bits
    let fpos_intmax: uintmax_t = (*stream).fpos as u64;
    if fpos_intmax > i64::max_value() as u64 {
        ELOG!("stream offset overflow");
        preserve_exit();
    }

    // Synchronous read
    uv_fs_read(
        (*handle).loop_0,
        &mut req,
        (*stream).fd,
        &mut (*stream).uvbuf as *const uv_buf_t,
        1,
        (*stream).fpos as i64,
        None,
    );

    uv_fs_req_cleanup(&mut req);

    if req.result <= 0 {
        uv_idle_stop(&mut (*stream).uv.idle);
        invoke_read_cb(stream, 0, true);
        return;
    }

    // no errors (req.result (libc::ssize_t) is positive), it's safe to cast.
    let nread: libc::size_t = req.result as libc::size_t;
    rbuffer_produced((*stream).buffer, nread);
    (*stream).fpos = ((*stream).fpos).wrapping_add(nread);
    invoke_read_cb(stream, nread, false);
}

unsafe extern "C" fn read_event(argv: *mut *mut libc::c_void) {
    let mut stream: *mut Stream = *argv.offset(0) as *mut Stream;
    if let Some(read_cb) = (*stream).read_cb {
        let count: libc::size_t = *argv.offset(1) as usize;
        let eof: bool = *argv.offset(2) as uintptr_t != 0;
        (*stream).did_eof = eof;
        read_cb(stream, (*stream).buffer, count, (*stream).cb_data, eof);
    }
    (*stream).pending_reqs = (*stream).pending_reqs.wrapping_sub(1);
    if (*stream).closed && (*stream).pending_reqs == 0 {
        stream_close_handle(stream);
    };
}

unsafe extern "C" fn invoke_read_cb(mut stream: *mut Stream, count: libc::size_t, eof: bool) {
    // Don't let the stream be closed before the event is processed.
    (*stream).pending_reqs = (*stream).pending_reqs.wrapping_add(1);

    CREATE_EVENT(
        (*stream).events.as_mut(),
        read_event,
        vargs!(stream, count as *mut uintptr_t, eof as uintptr_t),
    );
}
