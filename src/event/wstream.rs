use crate::*;
use std::ptr;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WBuffer {
    pub size: libc::size_t,
    pub refcount: libc::size_t,
    pub data: *mut libc::c_char,
    pub cb: wbuffer_data_finalizer,
}
pub type wbuffer_data_finalizer = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WRequest {
    pub stream: *mut Stream,
    pub buffer: *mut WBuffer,
    pub uv_req: uv_write_t<WRequest>,
}
const DEFAULT_MAXMEM: libc::size_t = 1024 * 1024 * 2000;

#[no_mangle]
pub unsafe extern "C" fn wstream_init_fd(
    loop_0: *mut Loop,
    stream: *mut Stream,
    fd: libc::c_int,
    maxmem: libc::size_t,
) {
    stream_init(loop_0, stream, fd, ptr::null_mut());
    wstream_init(stream, maxmem);
}

#[no_mangle]
pub unsafe extern "C" fn wstream_init_stream(
    stream: *mut Stream,
    uvstream: *mut uv_stream_t,
    maxmem: libc::size_t,
) {
    stream_init(ptr::null_mut(), stream, -1, uvstream);
    wstream_init(stream, maxmem);
}

#[no_mangle]
pub unsafe extern "C" fn wstream_init(mut stream: *mut Stream, maxmem: libc::size_t) {
    (*stream).maxmem = if maxmem != 0 { maxmem } else { DEFAULT_MAXMEM };
}

/// Sets a callback that will be called on completion of a write request,
/// indicating failure/success.
///
/// This affects all requests currently in-flight as well. Overwrites any
/// possible earlier callback.
///
/// @note This callback will not fire if the write request couldn't even be
///       queued properly (i.e.: when `wstream_write() returns an error`).
///
/// @param stream The `Stream` instance
/// @param cb The callback
#[no_mangle]
pub unsafe extern "C" fn wstream_set_write_cb(
    mut stream: *mut Stream,
    cb: stream_write_cb,
    data: *mut libc::c_void,
) {
    (*stream).write_cb = cb;
    (*stream).cb_data = data;
}

/// Queues data for writing to the backing file descriptor of a `Stream`
/// instance. This will fail if the write would cause the Stream use more
/// memory than specified by `maxmem`.
///
/// @param stream The `Stream` instance
/// @param buffer The buffer which contains data to be written
/// @return false if the write failed
#[no_mangle]
pub unsafe extern "C" fn wstream_write(mut stream: *mut Stream, buffer: *mut WBuffer) -> bool {
    c_assert!((*stream).maxmem != 0);
    // This should not be called after a stream was freed
    c_assert!(!(*stream).closed);

    if !((*stream).curmem > (*stream).maxmem) {
        (*stream).curmem = ((*stream).curmem).wrapping_add((*buffer).size);

        let mut data: *mut WRequest = xmalloc(std::mem::size_of::<WRequest>());
        (*data).stream = stream;
        (*data).buffer = buffer;
        (*data).uv_req.data = data;

        let uvbuf = uv_buf_t {
            base: (*buffer).data,
            len: (*buffer).size,
        };

        if uv_write(
            &mut (*data).uv_req,
            (*stream).uvstream,
            &uvbuf,
            1,
            Some(write_cb),
        ) != 0
        {
            xfree(data);
        } else {
            (*stream).pending_reqs = (*stream).pending_reqs.wrapping_add(1);
            return true;
        }
    }
    wstream_release_wbuffer(buffer);
    return false;
}

/// Creates a WBuffer object for holding output data. Instances of this
/// object can be reused across Stream instances, and the memory is freed
/// automatically when no longer needed(it tracks the number of references
/// internally)
///
/// @param data Data stored by the WBuffer
/// @param size The size of the data array
/// @param refcount The number of references for the WBuffer. This will be used
///        by Stream instances to decide when a WBuffer should be freed.
/// @param cb Pointer to function that will be responsible for freeing
///        the buffer data(passing 'free' will work as expected).
/// @return The allocated WBuffer instance
#[no_mangle]
pub unsafe extern "C" fn wstream_new_buffer(
    data: *mut libc::c_char,
    size: libc::size_t,
    refcount: libc::size_t,
    cb: wbuffer_data_finalizer,
) -> *mut WBuffer {
    let mut rv: *mut WBuffer = xmalloc(std::mem::size_of::<WBuffer>());
    (*rv).size = size;
    (*rv).refcount = refcount;
    (*rv).cb = cb;
    (*rv).data = data;
    return rv;
}

unsafe extern "C" fn write_cb(req: *mut uv_write_t<WRequest>, status: libc::c_int) {
    let mut data: *mut WRequest = (*req).data;

    (*(*data).stream).curmem = ((*(*data).stream).curmem).wrapping_sub((*(*data).buffer).size);

    wstream_release_wbuffer((*data).buffer);

    if let Some(write_cb) = (*(*data).stream).write_cb {
        write_cb((*data).stream, (*(*data).stream).cb_data, status);
    }

    (*(*data).stream).pending_reqs = (*(*data).stream).pending_reqs.wrapping_sub(1);

    if (*(*data).stream).closed && (*(*data).stream).pending_reqs == 0 {
        // Last pending write, free the stream;
        stream_close_handle((*data).stream);
    }
    xfree(data);
}

#[no_mangle]
pub unsafe extern "C" fn wstream_release_wbuffer(mut buffer: *mut WBuffer) {
    (*buffer).refcount = (*buffer).refcount.wrapping_sub(1);
    if (*buffer).refcount == 0 {
        if let Some(cb) = (*buffer).cb {
            cb((*buffer).data as *mut libc::c_void);
        }
        xfree(buffer);
    };
}
