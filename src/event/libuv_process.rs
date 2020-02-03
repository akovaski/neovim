use crate::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LibuvProcess {
    pub process: Process,
    pub uv: uv_process_t<Process>,
    pub uvopts: uv_process_options_t<Process>,
    pub uvstdio: [uv_stdio_container_t; 3],
}

/// @returns zero on success, or negative error code
#[no_mangle]
pub unsafe extern "C" fn libuv_process_spawn(mut uvproc: *mut LibuvProcess) -> libc::c_int {
    let mut proc_0: *mut Process = uvproc as *mut Process;
    (*uvproc).uvopts.file = *(*proc_0).argv.offset(0);
    (*uvproc).uvopts.args = (*proc_0).argv;
    (*uvproc).uvopts.flags = UV_PROCESS_WINDOWS_HIDE;
    if cfg!(windows) {
        unimplemented!();
    } else {
        // Always setsid() on unix-likes. #8107
        (*uvproc).uvopts.flags |= UV_PROCESS_DETACHED;
    }
    (*uvproc).uvopts.exit_cb = Some(exit_cb);
    (*uvproc).uvopts.cwd = (*proc_0).cwd;
    (*uvproc).uvopts.env = (*proc_0).env;
    (*uvproc).uvopts.stdio = (*uvproc).uvstdio.as_mut_ptr();
    (*uvproc).uvopts.stdio_count = 3;
    (*uvproc).uvstdio[0].flags = UV_IGNORE;
    (*uvproc).uvstdio[1].flags = UV_IGNORE;
    (*uvproc).uvstdio[2].flags = UV_IGNORE;
    (*uvproc).uv.data = proc_0;

    if !(*proc_0).in_0.closed {
        (*uvproc).uvstdio[0].flags = UV_CREATE_PIPE | UV_READABLE_PIPE;
        if cfg!(windows) {
            unimplemented!();
        }
        (*uvproc).uvstdio[0].data.stream.pipe = &mut (*proc_0).in_0.uv.pipe;
    }

    if !(*proc_0).out.closed {
        (*uvproc).uvstdio[1].flags = UV_CREATE_PIPE | UV_WRITABLE_PIPE;
        if cfg!(windows) {
            unimplemented!();
        }
        (*uvproc).uvstdio[1].data.stream.pipe = &mut (*proc_0).out.uv.pipe;
    }

    if !(*proc_0).err.closed {
        (*uvproc).uvstdio[2].flags = UV_CREATE_PIPE | UV_WRITABLE_PIPE;
        (*uvproc).uvstdio[2].data.stream.pipe = &mut (*proc_0).err.uv.pipe;
    }

    let status: libc::c_int = uv_spawn(
        &mut (*(*proc_0).loop_0).uv,
        &mut (*uvproc).uv,
        &mut (*uvproc).uvopts,
    );
    if status != 0 {
        ELOG!("uv_spawn failed: %s", uv_strerror(status));
        return status;
    }

    (*proc_0).pid = (*uvproc).uv.pid;
    return status;
}

#[no_mangle]
pub unsafe extern "C" fn libuv_process_close(uvproc: *mut LibuvProcess) {
    uv_close(&mut (*uvproc).uv, Some(close_cb));
}

unsafe extern "C" fn close_cb(handle: &mut uv_handle_t) {
    let proc_0: *mut Process = handle.data as *mut Process;
    if let Some(internal_close_cb) = (*proc_0).internal_close_cb {
        internal_close_cb(proc_0);
    };
}

unsafe extern "C" fn exit_cb(
    handle: *mut uv_process_t<Process>,
    status: i64,
    term_signal: libc::c_int,
) {
    let mut proc_0: *mut Process = (*handle).data;
    if cfg!(windows) {
        unimplemented!();
    }
    (*proc_0).status = if term_signal != 0 {
        (128) + term_signal
    } else {
        status as libc::c_int
    };
    (*proc_0)
        .internal_exit_cb
        .expect("non-null function pointer")(proc_0);
}
