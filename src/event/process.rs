use crate::*;
use std::ptr;

#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub enum ProcessType {
    kProcessTypeUv,
    kProcessTypePty,
}
use ProcessType::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Process {
    pub type_0: ProcessType,
    pub loop_0: *mut Loop,
    pub data: *mut libc::c_void,
    pub pid: libc::c_int,
    pub status: libc::c_int,
    pub refcount: libc::c_int,
    pub exit_signal: uv_process_signal, // Signal used when killing (on Windows).
    pub stopped_time: u64,              // process_stop() timestamp
    pub cwd: *const libc::c_char,
    pub argv: *mut *mut libc::c_char,
    pub env: *mut *mut libc::c_char,
    pub in_0: Stream,
    pub out: Stream,
    pub err: Stream,
    pub cb: process_exit_cb,
    pub internal_exit_cb: internal_process_cb,
    pub internal_close_cb: internal_process_cb,
    pub closed: bool,
    pub detach: bool,
    pub events: *mut MultiQueue,
}

pub type internal_process_cb = Option<unsafe extern "C" fn(_: *mut Process) -> ()>;
pub type process_exit_cb =
    Option<unsafe extern "C" fn(_: *mut Process, _: libc::c_int, _: *mut libc::c_void) -> ()>;

//TODO process_init, process_is_stopped

// Time for a process to exit cleanly before we send KILL.
// For PTY processes SIGTERM is sent first (in case SIGHUP was not enough).
const KILL_TIMEOUT_MS: u64 = 2000;

static mut process_is_tearing_down: bool = false;

/// @returns zero on success, or negative error code
#[no_mangle]
pub unsafe extern "C" fn process_spawn(
    mut proc_0: &mut Process,
    in_0: bool,
    out: bool,
    err: bool,
) -> libc::c_int {
    if in_0 {
        uv_pipe_init(&mut (*(*proc_0).loop_0).uv, &mut (*proc_0).in_0.uv.pipe, 0);
    } else {
        (*proc_0).in_0.closed = true
    }
    if out {
        uv_pipe_init(&mut (*(*proc_0).loop_0).uv, &mut (*proc_0).out.uv.pipe, 0);
    } else {
        (*proc_0).out.closed = true
    }
    if err {
        uv_pipe_init(&mut (*(*proc_0).loop_0).uv, &mut (*proc_0).err.uv.pipe, 0);
    } else {
        (*proc_0).err.closed = true
    }

    let status: libc::c_int = match (*proc_0).type_0 {
        kProcessTypeUv => libuv_process_spawn(proc_0 as *mut _ as *mut LibuvProcess),
        kProcessTypePty => pty_process_spawn(proc_0 as *mut _ as *mut PtyProcess),
    };

    if status != 0 {
        if in_0 {
            uv_close(&mut (*proc_0).in_0.uv.pipe, None);
        }
        if out {
            uv_close(&mut (*proc_0).out.uv.pipe, None);
        }
        if err {
            uv_close(&mut (*proc_0).err.uv.pipe, None);
        }
        if (*proc_0).type_0 == kProcessTypeUv {
            uv_close(&mut (*(proc_0 as *mut _ as *mut LibuvProcess)).uv, None);
        } else {
            process_close(proc_0);
        }
        shell_free_argv((*proc_0).argv);
        (*proc_0).status = -1;
        return status;
    }

    if in_0 {
        stream_init(
            ptr::null_mut(),
            &mut (*proc_0).in_0,
            -1,
            (&mut (*proc_0).in_0.uv.pipe).into(),
        );
        (*proc_0).in_0.internal_data = proc_0 as *mut _ as *mut libc::c_void;
        (*proc_0).in_0.internal_close_cb = Some(on_process_stream_close);
        (*proc_0).refcount += 1
    }

    if out {
        stream_init(
            ptr::null_mut(),
            &mut (*proc_0).out,
            -1,
            (&mut (*proc_0).out.uv.pipe).into(),
        );
        (*proc_0).out.internal_data = proc_0 as *mut _ as *mut libc::c_void;
        (*proc_0).out.internal_close_cb = Some(on_process_stream_close);
        (*proc_0).refcount += 1
    }

    if err {
        stream_init(
            ptr::null_mut(),
            &mut (*proc_0).err,
            -1,
            (&mut (*proc_0).err.uv.pipe).into(),
        );
        (*proc_0).err.internal_data = proc_0 as *mut _ as *mut libc::c_void;
        (*proc_0).err.internal_close_cb = Some(on_process_stream_close);
        (*proc_0).refcount += 1
    }

    (*proc_0).internal_exit_cb = Some(on_process_exit);
    (*proc_0).internal_close_cb = Some(decref);
    (*proc_0).refcount += 1;
    kl_push((*(*proc_0).loop_0).children, &mut proc_0);
    DLOG!("new: pid=%d argv=[%s]", (*proc_0).pid, *(*proc_0).argv);
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn process_teardown(loop_0: &mut Loop) {
    process_is_tearing_down = true;
    for current in (&*(*loop_0).children).iter() {
        let proc_0: &mut Process = ((**current).data as *mut Process).as_mut().unwrap();
        if (*proc_0).detach || (*proc_0).type_0 == kProcessTypePty {
            // Close handles to process without killing it.
            CREATE_EVENT(
                (*loop_0).events.as_mut(),
                process_close_handles,
                vargs!(proc_0 as *mut _),
            );
        } else {
            process_stop(proc_0);
        }
    }

    // Wait until all children exit and all close events are processed.
    LOOP_PROCESS_EVENTS_UNTIL(loop_0, (*loop_0).events.as_mut(), -1, |loop_0| {
        kl_empty((*loop_0).children) && multiqueue_empty((*loop_0).events.as_ref().unwrap())
    });
    pty_process_teardown(loop_0);
}

#[no_mangle]
pub unsafe extern "C" fn process_close_streams(proc_0: &mut Process) {
    stream_may_close(&mut (*proc_0).in_0);
    stream_may_close(&mut (*proc_0).out);
    stream_may_close(&mut (*proc_0).err);
}

/// Synchronously wait for a process to finish
///
/// @param process  Process instance
/// @param ms       Time in milliseconds to wait for the process.
///                 0 for no wait. -1 to wait until the process quits.
/// @return Exit code of the process. proc->status will have the same value.
///         -1 if the timeout expired while the process is still running.
///         -2 if the user interrupted the wait.
#[no_mangle]
pub unsafe extern "C" fn process_wait(
    mut proc_0: &mut Process,
    ms: libc::c_int,
    mut events: *mut MultiQueue,
) -> libc::c_int {
    let loop_0 = (*proc_0).loop_0.as_mut().unwrap();
    if (*proc_0).refcount == 0 {
        let status: libc::c_int = (*proc_0).status;
        LOOP_PROCESS_EVENTS(loop_0, &mut ((*proc_0).events.as_mut()), 0);
        return status;
    }

    if events.is_null() {
        events = (*proc_0).events
    }

    // Increase refcount to stop the exit callback from being called (and possibly
    // freed) before we have a chance to get the status.
    (*proc_0).refcount += 1;
    LOOP_PROCESS_EVENTS_UNTIL(loop_0, events.as_mut(), ms, |_| {
        // Until...
        got_int != 0 ||             // interrupted by the user
            (*proc_0).refcount == 1 // job exited
    });

    // Assume that a user hitting CTRL-C does not like the current job.  Kill it.
    if got_int != 0 {
        got_int = 0;
        process_stop(proc_0);
        if ms == -1 {
            // We can only return if all streams/handles are closed and the job
            // exited.
            LOOP_PROCESS_EVENTS_UNTIL(loop_0, events.as_mut(), -1, |_| (*proc_0).refcount == 1);
        } else {
            LOOP_PROCESS_EVENTS(loop_0, &mut (events.as_mut()), 0);
        }

        (*proc_0).status = -2
    }

    if (*proc_0).refcount == 1 {
        // Job exited, free its resources.
        decref(proc_0);
        if let Some(events) = events.as_mut() {
            // the decref call created an exit event, process it now
            multiqueue_process_events(events);
        }
    } else {
        (*proc_0).refcount -= 1
    }
    return (*proc_0).status;
}

/// Ask a process to terminate and eventually kill if it doesn't respond
#[no_mangle]
pub unsafe extern "C" fn process_stop(mut proc_0: &mut Process) {
    let exited: bool = (*proc_0).status >= 0;
    if exited || (*proc_0).stopped_time != 0 {
        return;
    }
    (*proc_0).stopped_time = os_hrtime();
    (*proc_0).exit_signal = SIGTERM;

    match (*proc_0).type_0 {
        kProcessTypeUv => {
            os_proc_tree_kill((*proc_0).pid, SIGTERM as i32);
        }
        kProcessTypePty => {
            // close all streams for pty processes to send SIGHUP to the process
            process_close_streams(proc_0);
            pty_process_close_master(proc_0 as *mut _ as *mut PtyProcess);
        }
    }

    // (Re)start timer to verify that stopped process(es) died.
    uv_timer_start(
        &mut (*(*proc_0).loop_0).children_kill_timer,
        Some(children_kill_cb),
        KILL_TIMEOUT_MS,
        0,
    );
}

/// Sends SIGKILL (or SIGTERM..SIGKILL for PTY jobs) to processes that did
/// not terminate after process_stop().
unsafe extern "C" fn children_kill_cb(handle: *mut uv_timer_t) {
    let loop_0: *mut Loop = (*(*handle).loop_0).data as *mut Loop;

    for current in (&*(*loop_0).children).iter() {
        let mut proc_0: *mut Process = (**current).data as *mut Process;
        let exited: bool = (*proc_0).status >= 0;
        if exited || (*proc_0).stopped_time == 0 {
            continue;
        }
        let term_sent: bool = u64::max_value() == (*proc_0).stopped_time;
        if kProcessTypePty != (*proc_0).type_0 || term_sent {
            (*proc_0).exit_signal = SIGKILL;
            os_proc_tree_kill((*proc_0).pid, SIGKILL as i32);
        } else {
            (*proc_0).exit_signal = SIGTERM;
            os_proc_tree_kill((*proc_0).pid, SIGTERM as i32);
            (*proc_0).stopped_time = u64::max_value(); // Flag: SIGTERM was sent.

            // Restart timer.
            uv_timer_start(
                &mut (*(*proc_0).loop_0).children_kill_timer,
                Some(children_kill_cb),
                KILL_TIMEOUT_MS,
                0,
            );
        }
    }
}

unsafe extern "C" fn process_close_event(argv: *mut *mut libc::c_void) {
    let proc_0: *mut Process = *argv.offset(0) as *mut Process;
    shell_free_argv((*proc_0).argv);
    if (*proc_0).type_0 == kProcessTypePty {
        xfree((*(proc_0 as *mut PtyProcess)).term_name);
    }
    if let Some(cb) = (*proc_0).cb {
        // "on_exit" for jobstart(). See channel_job_start().
        cb(proc_0, (*proc_0).status, (*proc_0).data);
    };
}

unsafe extern "C" fn decref(mut proc_0: *mut Process) {
    (*proc_0).refcount -= 1;
    if (*proc_0).refcount != 0 {
        return;
    }

    let loop_0: *mut Loop = (*proc_0).loop_0;
    let mut node: *mut *mut kl1<WatcherPtr> = ptr::null_mut();
    for current in (&*(*loop_0).children).iter() {
        if (**current).data == proc_0 as WatcherPtr {
            node = current;
            break;
        }
    }
    c_assert!(!node.is_null());
    kl_shift_at((*loop_0).children, node);
    CREATE_EVENT(
        (*proc_0).events.as_mut(),
        process_close_event,
        vargs!(proc_0),
    );
}

unsafe extern "C" fn process_close(mut proc_0: &mut Process) {
    if process_is_tearing_down
        && ((*proc_0).detach || (*proc_0).type_0 == kProcessTypePty)
        && (*proc_0).closed
    {
        // If a detached/pty process dies while tearing down it might get closed
        // twice.
        return;
    }
    c_assert!(!(*proc_0).closed);
    (*proc_0).closed = true;

    if (*proc_0).detach {
        if (*proc_0).type_0 == kProcessTypeUv {
            uv_unref(&mut (*(proc_0 as *mut _ as *mut LibuvProcess)).uv);
        }
    }
    match (*proc_0).type_0 {
        kProcessTypeUv => {
            libuv_process_close(proc_0 as *mut _ as *mut LibuvProcess);
        }
        kProcessTypePty => {
            pty_process_close(proc_0 as *mut _ as *mut PtyProcess);
        }
    };
}

/// Flush output stream.
///
/// @param loop     Loop, for which an output stream should be flushed.
/// @param stream   Stream to flush.
unsafe extern "C" fn flush_stream(loop_0: &mut Loop, stream: *mut Stream) {
    if stream.is_null() || (*stream).closed {
        return;
    }

    // Maximal remaining data size of terminated process is system
    // buffer size.
    // Also helps with a child process that keeps the output streams open. If it
    // keeps sending data, we only accept as much data as the system buffer size.
    // Otherwise this would block cleanup/teardown.
    let mut system_buffer_size: libc::c_int = 0;
    let err: libc::c_int = uv_recv_buffer_size(
        &mut (*stream).uv.pipe as *mut uv_pipe_t as *mut uv_handle_t,
        &mut system_buffer_size,
    );
    if err != 0 {
        system_buffer_size = rbuffer_capacity((*stream).buffer) as libc::c_int
    }

    let max_bytes: libc::size_t = (*stream).num_bytes + system_buffer_size as libc::size_t;

    // Read remaining data.
    while !(*stream).closed && (*stream).num_bytes < max_bytes {
        // Remember number of bytes before polling
        let num_bytes: libc::size_t = (*stream).num_bytes;

        // Poll for data and process the generated events.
        loop_poll_events(loop_0, 0);
        if let Some(events) = (*stream).events.as_mut() {
            multiqueue_process_events(events);
        }

        // Stream can be closed if it is empty.
        if num_bytes == (*stream).num_bytes {
            // -V547
            match (*stream).read_cb {
                Some(read_cb) if !(*stream).did_eof => {
                    // Stream callback could miss EOF handling if a child keeps the stream
                    // open. But only send EOF if we haven't already.
                    read_cb(stream, (*stream).buffer, 0, (*stream).cb_data, true);
                }
                _ => (),
            }
            break;
        }
    }
}

unsafe extern "C" fn process_close_handles(argv: *mut *mut libc::c_void) {
    let proc_0: &mut Process = (*argv.offset(0) as *mut Process).as_mut().unwrap();
    let loop_0: &mut Loop = (*proc_0).loop_0.as_mut().unwrap();

    flush_stream(loop_0, &mut (*proc_0).out);
    flush_stream(loop_0, &mut (*proc_0).err);

    process_close_streams(proc_0);
    process_close(proc_0);
}

unsafe extern "C" fn on_process_exit(proc_0: *mut Process) {
    let loop_0: *mut Loop = (*proc_0).loop_0;
    ILOG!(
        "exited: pid=%d status=%d stoptime=%lu",
        (*proc_0).pid,
        (*proc_0).status,
        (*proc_0).stopped_time
    );

    // Process has terminated, but there could still be data to be read from the
    // OS. We are still in the libuv loop, so we cannot call code that polls for
    // more data directly. Instead delay the reading after the libuv loop by
    // queueing process_close_handles() as an event.
    let queue: Option<&mut MultiQueue> = (*proc_0).events.as_mut().or((*loop_0).events.as_mut());
    CREATE_EVENT(queue, process_close_handles, vargs!(proc_0));
}

unsafe extern "C" fn on_process_stream_close(_stream: *mut Stream, data: *mut libc::c_void) {
    let proc_0: *mut Process = data as *mut Process;
    decref(proc_0);
}
