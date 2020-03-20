use crate::c_helpers::*;
use std::ptr;
extern "C" {
    pub fn uv_signal_start(
        handle: *mut uv_signal_t,
        signal_cb: uv_signal_cb,
        signum: libc::c_int,
    ) -> libc::c_int;
    pub fn uv_signal_stop(handle: *mut uv_signal_t) -> libc::c_int;
    pub fn uv_loop_init(loop_0: *mut uv_loop_t) -> libc::c_int;
    pub fn uv_loop_close(loop_0: *mut uv_loop_t) -> libc::c_int;
    pub fn uv_run(_: *mut uv_loop_t, mode: uv_run_mode) -> libc::c_int;
    pub fn uv_stop(_: *mut uv_loop_t);
    pub fn uv_walk(_: *mut uv_loop_t, _: uv_walk_cb, _: *mut libc::c_void);
    pub fn uv_is_closing(_: *mut uv_handle_t) -> libc::c_int;
    #[link_name = "uv_close"]
    fn c_uv_close(handle: *mut uv_handle_t, close_cb: uv_close_cb);
    pub fn uv_async_init(
        _: *mut uv_loop_t,
        async_0: *mut uv_async_t,
        async_cb_0: uv_async_cb,
    ) -> libc::c_int;
    pub fn uv_async_send(async_0: *mut uv_async_t) -> libc::c_int;
    pub fn uv_timer_init(_: *mut uv_loop_t, handle: *mut uv_timer_t) -> libc::c_int;
    pub fn uv_timer_start(
        handle: *mut uv_timer_t,
        cb: uv_timer_cb,
        timeout: u64,
        repeat: u64,
    ) -> libc::c_int;
    pub fn uv_timer_stop(handle: *mut uv_timer_t) -> libc::c_int;
    pub fn uv_signal_init(loop_0: *mut uv_loop_t, handle: *mut uv_signal_t) -> libc::c_int;
    pub fn uv_now(_: *const uv_loop_t) -> u64;
    pub fn uv_hrtime() -> u64;
    pub fn uv_mutex_init(handle: *mut uv_mutex_t) -> libc::c_int;
    pub fn uv_mutex_destroy(handle: *mut uv_mutex_t);
    pub fn uv_mutex_lock(handle: *mut uv_mutex_t);
    pub fn uv_mutex_unlock(handle: *mut uv_mutex_t);
    pub fn uv_cond_init(cond: *mut uv_cond_t) -> libc::c_int;
    pub fn uv_cond_timedwait(
        cond: *mut uv_cond_t,
        mutex: *mut uv_mutex_t,
        timeout: u64,
    ) -> libc::c_int;
    pub fn uv_stream_get_write_queue_size(stream: uv_stream_mut) -> libc::size_t;
    pub fn uv_stream_set_blocking(handle: *mut uv_pipe_t, blocking: libc::c_int) -> libc::c_int;
    pub fn uv_guess_handle(file: uv_file) -> uv_handle_type;
    pub fn uv_listen(
        stream: uv_stream_mut,
        backlog: libc::c_int,
        cb: uv_connection_cb,
    ) -> libc::c_int;
    pub fn uv_pipe_init(_: *mut uv_loop_t, handle: *mut uv_pipe_t, ipc: libc::c_int)
        -> libc::c_int;
    pub fn uv_pipe_open(_: *mut uv_pipe_t, file: uv_file) -> libc::c_int;
    pub fn uv_pipe_connect(
        req: *mut uv_connect_t,
        handle: *mut uv_pipe_t,
        name: *const libc::c_char,
        cb: uv_connect_cb,
    );
    pub fn uv_pipe_bind(handle: *mut uv_pipe_t, name: *const libc::c_char) -> libc::c_int;
    pub fn uv_idle_init(_: *mut uv_loop_t, idle: *mut uv_idle_t) -> libc::c_int;
    #[link_name = "uv_write"]
    fn c_uv_write(
        req: *mut uv_write_t<libc::c_void>,
        handle: uv_stream_mut,
        bufs: *const uv_buf_t,
        nbufs: libc::c_uint,
        cb: uv_write_cb<libc::c_void>,
    ) -> libc::c_int;
    pub fn uv_read_start(
        _: uv_stream_mut,
        alloc_cb_0: uv_alloc_cb,
        read_cb_0: uv_read_cb,
    ) -> libc::c_int;
    pub fn uv_read_stop(_: uv_stream_mut) -> libc::c_int;
    pub fn uv_idle_start(idle: *mut uv_idle_t, cb: uv_idle_cb) -> libc::c_int;
    pub fn uv_idle_stop(idle: *mut uv_idle_t) -> libc::c_int;
    pub fn uv_fs_req_cleanup(req: *mut uv_fs_t);
    pub fn uv_fs_read(
        loop_0: *mut uv_loop_t,
        req: *mut uv_fs_t,
        file: uv_file,
        bufs: *const uv_buf_t,
        nbufs: libc::c_uint,
        offset: i64,
        cb: uv_fs_cb,
    ) -> libc::c_int;
    pub fn uv_err_name(err: libc::c_int) -> *const libc::c_char;
    pub fn uv_strerror(err: libc::c_int) -> *const libc::c_char;
    #[link_name = "uv_spawn"]
    pub fn c_uv_spawn(
        loop_0: *mut uv_loop_t,
        handle: *mut uv_process_t<libc::c_void>,
        options: *const uv_process_options_t<libc::c_void>,
    ) -> libc::c_int;
    #[link_name = "uv_recv_buffer_size"]
    pub fn c_uv_recv_buffer_size(handle: *mut uv_handle_t, value: *mut libc::c_int) -> libc::c_int;
    #[link_name = "uv_unref"]
    fn c_uv_unref(_: *mut uv_handle_t);
    pub fn uv_getaddrinfo(
        loop_0: *mut uv_loop_t,
        req: *mut uv_getaddrinfo_t,
        getaddrinfo_cb: uv_getaddrinfo_cb,
        node: *const libc::c_char,
        service: *const libc::c_char,
        hints: *const addrinfo,
    ) -> libc::c_int;
    pub fn uv_freeaddrinfo(ai: *mut addrinfo);
    pub fn uv_tcp_init(_: *mut uv_loop_t, handle: *mut uv_tcp_t) -> libc::c_int;
    pub fn uv_tcp_nodelay(handle: *mut uv_tcp_t, enable: libc::c_int) -> libc::c_int;
    pub fn uv_tcp_bind(
        handle: *mut uv_tcp_t,
        addr: *const sockaddr,
        flags: libc::c_uint,
    ) -> libc::c_int;
    pub fn uv_tcp_getsockname(
        handle: *const uv_tcp_t,
        name: *mut sockaddr_storage,
        namelen: *const libc::c_int,
    ) -> libc::c_int;
    pub fn uv_tcp_connect(
        req: *mut uv_connect_t,
        handle: *mut uv_tcp_t,
        addr: *const sockaddr,
        cb: uv_connect_cb,
    ) -> libc::c_int;
    pub fn uv_accept(server: uv_stream_mut, client: uv_stream_mut) -> libc::c_int;
}

pub unsafe fn uv_write<D>(
    req: *mut uv_write_t<D>,
    handle: uv_stream_mut,
    bufs: *const uv_buf_t,
    nbufs: libc::c_uint,
    cb: uv_write_cb<D>,
) -> libc::c_int {
    c_uv_write(
        req as *mut uv_write_t<libc::c_void>,
        handle,
        bufs,
        nbufs,
        std::mem::transmute(cb),
    )
}

pub trait UvHandler {}
impl UvHandler for uv_timer_t {}
impl UvHandler for uv_signal_s {}
impl UvHandler for uv_async_s {}
impl UvHandler for uv_pipe_t {}
impl UvHandler for uv_stream_s {}
impl UvHandler for uv_handle_s {}
impl UvHandler for uv_idle_s {}
impl<T> UvHandler for uv_process_s<T> {}
pub unsafe fn uv_close<T: UvHandler>(handle: *mut T, close_cb: uv_close_cb) {
    c_uv_close(handle as *mut uv_handle_t, close_cb);
}
pub unsafe fn uv_recv_buffer_size<T: UvHandler>(
    handle: *mut T,
    value: *mut libc::c_int,
) -> libc::c_int {
    c_uv_recv_buffer_size(handle as *mut uv_handle_t, value)
}
pub unsafe fn uv_unref<T: UvHandler>(handle: *mut T) {
    c_uv_unref(handle as *mut uv_handle_t);
}
pub unsafe fn uv_spawn<T>(
    loop_0: *mut uv_loop_t,
    handle: *mut uv_process_t<T>,
    options: *const uv_process_options_t<T>,
) -> libc::c_int {
    c_uv_spawn(
        loop_0,
        handle as *mut uv_process_t<libc::c_void>,
        options as *mut uv_process_options_t<libc::c_void>,
    )
}

pub type uv_async_cb = Option<unsafe extern "C" fn(_: *mut uv_async_t) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: libc::c_uint,
    pub __writers: libc::c_uint,
    pub __wrphase_futex: libc::c_uint,
    pub __writers_futex: libc::c_uint,
    pub __pad3: libc::c_uint,
    pub __pad4: libc::c_uint,
    pub __cur_writer: libc::c_int,
    pub __shared: libc::c_int,
    pub __rwelision: libc::c_schar,
    pub __pad1: [libc::c_uchar; 7],
    pub __pad2: libc::c_ulong,
    pub __flags: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[repr(C)]
pub struct uv_loop_s {
    pub data: *mut libc::c_void,
    pub active_handles: libc::c_uint,
    pub handle_queue: [*mut libc::c_void; 2],
    pub active_reqs: C2RustUnnamed_4,
    pub stop_flag: libc::c_uint,
    pub flags: libc::c_ulong,
    pub backend_fd: libc::c_int,
    pub pending_queue: [*mut libc::c_void; 2],
    pub watcher_queue: [*mut libc::c_void; 2],
    pub watchers: *mut *mut uv__io_t,
    pub nwatchers: libc::c_uint,
    pub nfds: libc::c_uint,
    pub wq: [*mut libc::c_void; 2],
    pub wq_mutex: uv_mutex_t,
    pub wq_async: uv_async_t,
    pub cloexec_lock: uv_rwlock_t,
    pub closing_handles: *mut uv_handle_t,
    pub process_handles: [*mut libc::c_void; 2],
    pub prepare_handles: [*mut libc::c_void; 2],
    pub check_handles: [*mut libc::c_void; 2],
    pub idle_handles: [*mut libc::c_void; 2],
    pub async_handles: [*mut libc::c_void; 2],
    pub async_unused: Option<unsafe extern "C" fn() -> ()>,
    pub async_io_watcher: uv__io_t,
    pub async_wfd: libc::c_int,
    pub timer_heap: C2RustUnnamed_2,
    pub timer_counter: u64,
    pub time: u64,
    pub signal_pipefd: [libc::c_int; 2],
    pub signal_io_watcher: uv__io_t,
    pub child_watcher: uv_signal_t,
    pub emfile_fd: libc::c_int,
    pub inotify_read_watcher: uv__io_t,
    pub inotify_watchers: *mut libc::c_void,
    pub inotify_fd: libc::c_int,
}
pub type uv__io_t = uv__io_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__io_s {
    pub cb: uv__io_cb,
    pub pending_queue: [*mut libc::c_void; 2],
    pub watcher_queue: [*mut libc::c_void; 2],
    pub pevents: libc::c_uint,
    pub events: libc::c_uint,
    pub fd: libc::c_int,
}
pub type uv__io_cb =
    Option<unsafe extern "C" fn(_: *mut uv_loop_s, _: *mut uv__io_s, _: libc::c_uint) -> ()>;
pub type uv_signal_t = uv_signal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_signal_s {
    pub data: *mut libc::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut libc::c_void; 2],
    pub u: Unnamed_uv_handle_fs_r,
    pub next_closing: *mut uv_handle_t,
    pub flags: libc::c_uint,
    pub signal_cb: uv_signal_cb,
    pub signum: libc::c_int,
    pub tree_entry: C2RustUnnamed,
    pub caught_signals: libc::c_uint,
    pub dispatched_signals: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub rbe_left: *mut uv_signal_s,
    pub rbe_right: *mut uv_signal_s,
    pub rbe_parent: *mut uv_signal_s,
    pub rbe_color: libc::c_int,
}
pub type uv_signal_cb = Option<unsafe extern "C" fn(_: &mut uv_signal_t, _: libc::c_int) -> ()>;
pub type uv_handle_t = uv_handle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_handle_s {
    pub data: *mut libc::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut libc::c_void; 2],
    pub u: Unnamed_uv_handle_fs_r,
    pub next_closing: *mut uv_handle_t,
    pub flags: libc::c_uint,
}
pub type uv_close_cb = Option<unsafe extern "C" fn(_: &mut uv_handle_t) -> ()>;
pub type uv_walk_cb =
    Option<unsafe extern "C" fn(_: &mut uv_handle_t, arg: *mut libc::c_void) -> ()>;
pub type uv_handle_type = libc::c_uint;
pub const UV_HANDLE_TYPE_MAX: uv_handle_type = 18;
pub const UV_FILE: uv_handle_type = 17;
pub const UV_SIGNAL: uv_handle_type = 16;
pub const UV_UDP: uv_handle_type = 15;
pub const UV_TTY: uv_handle_type = 14;
pub const UV_TIMER: uv_handle_type = 13;
pub const UV_TCP: uv_handle_type = 12;
pub const UV_STREAM: uv_handle_type = 11;
pub const UV_PROCESS: uv_handle_type = 10;
pub const UV_PREPARE: uv_handle_type = 9;
pub const UV_POLL: uv_handle_type = 8;
pub const UV_NAMED_PIPE: uv_handle_type = 7;
pub const UV_IDLE: uv_handle_type = 6;
pub const UV_HANDLE: uv_handle_type = 5;
pub const UV_FS_POLL: uv_handle_type = 4;
pub const UV_FS_EVENT: uv_handle_type = 3;
pub const UV_CHECK: uv_handle_type = 2;
pub const UV_ASYNC: uv_handle_type = 1;
pub const UV_UNKNOWN_HANDLE: uv_handle_type = 0;
pub type uv_loop_t = uv_loop_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub min: *mut libc::c_void,
    pub nelts: libc::c_uint,
}
pub type uv_rwlock_t = pthread_rwlock_t;
pub type uv_async_t = uv_async_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_async_s {
    pub data: *mut libc::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut libc::c_void; 2],
    pub u: Unnamed_uv_handle_fs_r,
    pub next_closing: *mut uv_handle_t,
    pub flags: libc::c_uint,
    pub async_cb: uv_async_cb,
    pub queue: [*mut libc::c_void; 2],
    pub pending: libc::c_int,
}
pub type uv_mutex_t = pthread_mutex_t;
impl Default for uv_mutex_t {
    fn default() -> uv_mutex_t {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub unused: [*mut libc::c_void; 2],
    pub count: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_buf_t {
    pub base: *mut libc::c_char,
    pub len: libc::size_t,
}
pub type uv_file = libc::c_int;
pub type uv_req_type = libc::c_uint;
pub const UV_REQ_TYPE_MAX: uv_req_type = 10;
pub const UV_GETNAMEINFO: uv_req_type = 9;
pub const UV_GETADDRINFO: uv_req_type = 8;
pub const UV_WORK: uv_req_type = 7;
pub const UV_FS: uv_req_type = 6;
pub const UV_UDP_SEND: uv_req_type = 5;
pub const UV_SHUTDOWN: uv_req_type = 4;
pub const UV_WRITE: uv_req_type = 3;
pub const UV_CONNECT: uv_req_type = 2;
pub const UV_REQ: uv_req_type = 1;
pub const UV_UNKNOWN_REQ: uv_req_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_stream_s {
    pub data: *mut libc::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut libc::c_void; 2],
    pub u: Unnamed_uv_handle_fs_r,
    pub next_closing: *mut uv_handle_t,
    pub flags: libc::c_uint,
    pub write_queue_size: libc::size_t,
    pub alloc_cb: uv_alloc_cb,
    pub read_cb: uv_read_cb,
    pub connect_req: *mut uv_connect_t,
    pub shutdown_req: *mut uv_shutdown_t,
    pub io_watcher: uv__io_t,
    pub write_queue: [*mut libc::c_void; 2],
    pub write_completed_queue: [*mut libc::c_void; 2],
    pub connection_cb: uv_connection_cb,
    pub delayed_error: libc::c_int,
    pub accepted_fd: libc::c_int,
    pub queued_fds: *mut libc::c_void,
}
pub type uv_stream_t = uv_stream_s;
pub type uv_connection_cb = Option<unsafe extern "C" fn(_: &mut uv_stream_t, _: libc::c_int) -> ()>;
pub type uv_shutdown_t = uv_shutdown_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_shutdown_s {
    pub data: *mut libc::c_void,
    pub type_0: uv_req_type,
    pub reserved: [*mut libc::c_void; 6],
    pub handle: *mut uv_stream_t,
    pub cb: uv_shutdown_cb,
}
pub type uv_shutdown_cb = Option<unsafe extern "C" fn(_: *mut uv_shutdown_t, _: libc::c_int) -> ()>;
pub type uv_connect_t = uv_connect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_connect_s {
    pub data: *mut libc::c_void,
    pub type_0: uv_req_type,
    pub reserved: [*mut libc::c_void; 6],
    pub cb: uv_connect_cb,
    pub handle: *mut uv_stream_t,
    pub queue: [*mut libc::c_void; 2],
}
impl Default for uv_connect_s {
    fn default() -> uv_connect_s {
        uv_connect_t {
            data: ptr::null_mut(),
            type_0: UV_UNKNOWN_REQ,
            reserved: [ptr::null_mut(); 6],
            cb: None,
            handle: ptr::null_mut(),
            queue: [ptr::null_mut(); 2],
        }
    }
}
pub type uv_connect_cb = Option<unsafe extern "C" fn(_: &mut uv_connect_t, _: libc::c_int) -> ()>;
pub type uv_read_cb =
    Option<unsafe extern "C" fn(_: uv_stream_mut, _: libc::ssize_t, _: *const uv_buf_t) -> ()>;
pub type uv_alloc_cb =
    Option<unsafe extern "C" fn(_: *mut uv_handle_t, _: libc::size_t, _: *mut uv_buf_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_tcp_s {
    pub data: *mut libc::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut libc::c_void; 2],
    pub u: Unnamed_uv_handle_fs_r,
    pub next_closing: *mut uv_handle_t,
    pub flags: libc::c_uint,
    pub write_queue_size: libc::size_t,
    pub alloc_cb: uv_alloc_cb,
    pub read_cb: uv_read_cb,
    pub connect_req: *mut uv_connect_t,
    pub shutdown_req: *mut uv_shutdown_t,
    pub io_watcher: uv__io_t,
    pub write_queue: [*mut libc::c_void; 2],
    pub write_completed_queue: [*mut libc::c_void; 2],
    pub connection_cb: uv_connection_cb,
    pub delayed_error: libc::c_int,
    pub accepted_fd: libc::c_int,
    pub queued_fds: *mut libc::c_void,
}
pub type uv_tcp_t = uv_tcp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_pipe_s {
    pub data: *mut libc::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut libc::c_void; 2],
    pub u: Unnamed_uv_handle_fs_r,
    pub next_closing: *mut uv_handle_t,
    pub flags: libc::c_uint,
    pub write_queue_size: libc::size_t,
    pub alloc_cb: uv_alloc_cb,
    pub read_cb: uv_read_cb,
    pub connect_req: *mut uv_connect_t,
    pub shutdown_req: *mut uv_shutdown_t,
    pub io_watcher: uv__io_t,
    pub write_queue: [*mut libc::c_void; 2],
    pub write_completed_queue: [*mut libc::c_void; 2],
    pub connection_cb: uv_connection_cb,
    pub delayed_error: libc::c_int,
    pub accepted_fd: libc::c_int,
    pub queued_fds: *mut libc::c_void,
    pub ipc: libc::c_int,
    pub pipe_fname: *const libc::c_char,
}
pub type uv_pipe_t = uv_pipe_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_timer_s {
    pub data: *mut libc::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut libc::c_void; 2],
    pub u: Unnamed_uv_handle_fs_r,
    pub next_closing: *mut uv_handle_t,
    pub flags: libc::c_uint,
    pub timer_cb: uv_timer_cb,
    pub heap_node: [*mut libc::c_void; 3],
    pub timeout: u64,
    pub repeat: u64,
    pub start_id: u64,
}
pub type uv_timer_cb = Option<unsafe extern "C" fn(_: &mut uv_timer_t) -> ()>;
pub type uv_timer_t = uv_timer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_idle_s {
    pub data: *mut libc::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut libc::c_void; 2],
    pub u: Unnamed_uv_handle_fs_r,
    pub next_closing: *mut uv_handle_t,
    pub flags: libc::c_uint,
    pub idle_cb: uv_idle_cb,
    pub queue: [*mut libc::c_void; 2],
}
pub type uv_idle_cb = Option<unsafe extern "C" fn(_: *mut uv_idle_t) -> ()>;
pub type uv_idle_t = uv_idle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Unnamed_uv_handle_fs_r {
    pub fd: libc::c_int,
    pub reserved: [*mut libc::c_void; 4],
}

pub use uv_run_mode::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub enum uv_run_mode {
    UV_RUN_DEFAULT,
    UV_RUN_ONCE,
    UV_RUN_NOWAIT,
}
pub type uv_cond_t = pthread_cond_t;
impl Default for uv_cond_t {
    fn default() -> pthread_cond_t {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: _cond_s_C2RustUnnamed_1,
    pub c2rust_unnamed_0: _cond_s_C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _cond_s_C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: _cond_s_C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cond_s_C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _cond_s_C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: _cond_s_C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cond_s_C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
pub type UvError = libc::c_int;
pub const UV_ERRNO_MAX: UvError = -4096;
pub const UV_EILSEQ: UvError = -84;
pub const UV_EFTYPE: UvError = -4028;
pub const UV_ENOTTY: UvError = -25;
pub const UV_EREMOTEIO: UvError = -121;
pub const UV_EHOSTDOWN: UvError = -112;
pub const UV_EMLINK: UvError = -31;
pub const UV_ENXIO: UvError = -6;
pub const UV_EOF: UvError = -4095;
pub const UV_UNKNOWN: UvError = -4094;
pub const UV_EXDEV: UvError = -18;
pub const UV_ETXTBSY: UvError = -26;
pub const UV_ETIMEDOUT: UvError = -110;
pub const UV_ESRCH: UvError = -3;
pub const UV_ESPIPE: UvError = -29;
pub const UV_ESHUTDOWN: UvError = -108;
pub const UV_EROFS: UvError = -30;
pub const UV_ERANGE: UvError = -34;
pub const UV_EPROTOTYPE: UvError = -91;
pub const UV_EPROTONOSUPPORT: UvError = -93;
pub const UV_EPROTO: UvError = -71;
pub const UV_EPIPE: UvError = -32;
pub const UV_EPERM: UvError = -1;
pub const UV_ENOTSUP: UvError = -95;
pub const UV_ENOTSOCK: UvError = -88;
pub const UV_ENOTEMPTY: UvError = -39;
pub const UV_ENOTDIR: UvError = -20;
pub const UV_ENOTCONN: UvError = -107;
pub const UV_ENOSYS: UvError = -38;
pub const UV_ENOSPC: UvError = -28;
pub const UV_ENOPROTOOPT: UvError = -92;
pub const UV_ENONET: UvError = -64;
pub const UV_ENOMEM: UvError = -12;
pub const UV_ENOENT: UvError = -2;
pub const UV_ENODEV: UvError = -19;
pub const UV_ENOBUFS: UvError = -105;
pub const UV_ENFILE: UvError = -23;
pub const UV_ENETUNREACH: UvError = -101;
pub const UV_ENETDOWN: UvError = -100;
pub const UV_ENAMETOOLONG: UvError = -36;
pub const UV_EMSGSIZE: UvError = -90;
pub const UV_EMFILE: UvError = -24;
pub const UV_ELOOP: UvError = -40;
pub const UV_EISDIR: UvError = -21;
pub const UV_EISCONN: UvError = -106;
pub const UV_EIO: UvError = -5;
pub const UV_EINVAL: UvError = -22;
pub const UV_EINTR: UvError = -4;
pub const UV_EHOSTUNREACH: UvError = -113;
pub const UV_EFBIG: UvError = -27;
pub const UV_EFAULT: UvError = -14;
pub const UV_EEXIST: UvError = -17;
pub const UV_EDESTADDRREQ: UvError = -89;
pub const UV_ECONNRESET: UvError = -104;
pub const UV_ECONNREFUSED: UvError = -111;
pub const UV_ECONNABORTED: UvError = -103;
pub const UV_ECHARSET: UvError = -4080;
pub const UV_ECANCELED: UvError = -125;
pub const UV_EBUSY: UvError = -16;
pub const UV_EBADF: UvError = -9;
pub const UV_EALREADY: UvError = -114;
pub const UV_EAI_SOCKTYPE: UvError = -3011;
pub const UV_EAI_SERVICE: UvError = -3010;
pub const UV_EAI_PROTOCOL: UvError = -3014;
pub const UV_EAI_OVERFLOW: UvError = -3009;
pub const UV_EAI_NONAME: UvError = -3008;
pub const UV_EAI_NODATA: UvError = -3007;
pub const UV_EAI_MEMORY: UvError = -3006;
pub const UV_EAI_FAMILY: UvError = -3005;
pub const UV_EAI_FAIL: UvError = -3004;
pub const UV_EAI_CANCELED: UvError = -3003;
pub const UV_EAI_BADHINTS: UvError = -3013;
pub const UV_EAI_BADFLAGS: UvError = -3002;
pub const UV_EAI_AGAIN: UvError = -3001;
pub const UV_EAI_ADDRFAMILY: UvError = -3000;
pub const UV_EAGAIN: UvError = -11;
pub const UV_EAFNOSUPPORT: UvError = -97;
pub const UV_EADDRNOTAVAIL: UvError = -99;
pub const UV_EADDRINUSE: UvError = -98;
pub const UV_EACCES: UvError = -13;
pub const UV_E2BIG: UvError = -7;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_write_s<D> {
    pub data: *mut D,
    pub type_0: uv_req_type,
    pub reserved: [*mut libc::c_void; 6],
    pub cb: uv_write_cb<D>,
    pub send_handle: *mut uv_stream_t,
    pub handle: *mut uv_stream_t,
    pub queue: [*mut libc::c_void; 2],
    pub write_index: libc::c_uint,
    pub bufs: *mut uv_buf_t,
    pub nbufs: libc::c_uint,
    pub error: libc::c_int,
    pub bufsml: [uv_buf_t; 4],
}
pub type uv_write_cb<D> = Option<unsafe extern "C" fn(_: *mut uv_write_t<D>, _: libc::c_int) -> ()>;
pub type uv_write_t<D> = uv_write_s<D>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_fs_t {
    pub data: *mut libc::c_void,
    pub type_0: uv_req_type,
    pub reserved: [*mut libc::c_void; 6],
    pub fs_type: uv_fs_type,
    pub loop_0: *mut uv_loop_t,
    pub cb: uv_fs_cb,
    pub result: libc::ssize_t,
    pub ptr: *mut libc::c_void,
    pub path: *const libc::c_char,
    pub statbuf: uv_stat_t,
    pub new_path: *const libc::c_char,
    pub file: uv_file,
    pub flags: libc::c_int,
    pub mode: mode_t,
    pub nbufs: libc::c_uint,
    pub bufs: *mut uv_buf_t,
    pub off: off_t,
    pub uid: uv_uid_t,
    pub gid: uv_gid_t,
    pub atime: libc::c_double,
    pub mtime: libc::c_double,
    pub work_req: uv__work,
    pub bufsml: [uv_buf_t; 4],
}
pub type uv_fs_cb = Option<unsafe extern "C" fn(_: *mut uv_fs_t) -> ()>;
pub type uv_fs_type = libc::c_int;
pub const UV_FS_STATFS: uv_fs_type = 34;
pub const UV_FS_CLOSEDIR: uv_fs_type = 33;
pub const UV_FS_READDIR: uv_fs_type = 32;
pub const UV_FS_OPENDIR: uv_fs_type = 31;
pub const UV_FS_LCHOWN: uv_fs_type = 30;
pub const UV_FS_COPYFILE: uv_fs_type = 29;
pub const UV_FS_REALPATH: uv_fs_type = 28;
pub const UV_FS_FCHOWN: uv_fs_type = 27;
pub const UV_FS_CHOWN: uv_fs_type = 26;
pub const UV_FS_READLINK: uv_fs_type = 25;
pub const UV_FS_SYMLINK: uv_fs_type = 24;
pub const UV_FS_LINK: uv_fs_type = 23;
pub const UV_FS_SCANDIR: uv_fs_type = 22;
pub const UV_FS_RENAME: uv_fs_type = 21;
pub const UV_FS_MKDTEMP: uv_fs_type = 20;
pub const UV_FS_MKDIR: uv_fs_type = 19;
pub const UV_FS_RMDIR: uv_fs_type = 18;
pub const UV_FS_UNLINK: uv_fs_type = 17;
pub const UV_FS_FDATASYNC: uv_fs_type = 16;
pub const UV_FS_FSYNC: uv_fs_type = 15;
pub const UV_FS_FCHMOD: uv_fs_type = 14;
pub const UV_FS_CHMOD: uv_fs_type = 13;
pub const UV_FS_ACCESS: uv_fs_type = 12;
pub const UV_FS_FUTIME: uv_fs_type = 11;
pub const UV_FS_UTIME: uv_fs_type = 10;
pub const UV_FS_FTRUNCATE: uv_fs_type = 9;
pub const UV_FS_FSTAT: uv_fs_type = 8;
pub const UV_FS_LSTAT: uv_fs_type = 7;
pub const UV_FS_STAT: uv_fs_type = 6;
pub const UV_FS_SENDFILE: uv_fs_type = 5;
pub const UV_FS_WRITE: uv_fs_type = 4;
pub const UV_FS_READ: uv_fs_type = 3;
pub const UV_FS_CLOSE: uv_fs_type = 2;
pub const UV_FS_OPEN: uv_fs_type = 1;
pub const UV_FS_CUSTOM: uv_fs_type = 0;
pub const UV_FS_UNKNOWN: uv_fs_type = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_stat_t {
    pub st_dev: u64,
    pub st_mode: u64,
    pub st_nlink: u64,
    pub st_uid: u64,
    pub st_gid: u64,
    pub st_rdev: u64,
    pub st_ino: u64,
    pub st_size: u64,
    pub st_blksize: u64,
    pub st_blocks: u64,
    pub st_flags: u64,
    pub st_gen: u64,
    pub st_atim: uv_timespec_t,
    pub st_mtim: uv_timespec_t,
    pub st_ctim: uv_timespec_t,
    pub st_birthtim: uv_timespec_t,
}
pub type mode_t = libc::c_uint;
pub type off_t = libc::c_long;
pub type uv_gid_t = libc::c_uint;
pub type uv_uid_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__work {
    pub work: Option<unsafe extern "C" fn(_: *mut uv__work) -> ()>,
    pub done: Option<unsafe extern "C" fn(_: *mut uv__work, _: libc::c_int) -> ()>,
    pub loop_0: *mut uv_loop_s,
    pub wq: [*mut libc::c_void; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_timespec_t {
    pub tv_sec: libc::c_long,
    pub tv_nsec: libc::c_long,
}
pub type uv_process_t<T> = uv_process_s<T>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_process_s<T> {
    pub data: *mut T,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut libc::c_void; 2],
    pub u: Unnamed_uv_handle_fs_r,
    pub next_closing: *mut uv_handle_t,
    pub flags: libc::c_uint,
    pub exit_cb: uv_exit_cb<T>,
    pub pid: libc::c_int,
    pub queue: [*mut libc::c_void; 2],
    pub status: libc::c_int,
}
pub type uv_exit_cb<T> =
    Option<unsafe extern "C" fn(_: *mut uv_process_t<T>, _: i64, _: libc::c_int) -> ()>;
pub type uv_stdio_flags = libc::c_uint;
pub const UV_OVERLAPPED_PIPE: uv_stdio_flags = 64;
pub const UV_WRITABLE_PIPE: uv_stdio_flags = 32;
pub const UV_READABLE_PIPE: uv_stdio_flags = 16;
pub const UV_INHERIT_STREAM: uv_stdio_flags = 4;
pub const UV_INHERIT_FD: uv_stdio_flags = 2;
pub const UV_CREATE_PIPE: uv_stdio_flags = 1;
pub const UV_IGNORE: uv_stdio_flags = 0;
pub type uv_stdio_container_t = uv_stdio_container_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_stdio_container_s {
    pub flags: uv_stdio_flags,
    pub data: Unnamed_uv_stdio_stream_union,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Unnamed_uv_stdio_stream_union {
    pub stream: uv_stream_mut,
    pub fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union uv_stream_mut {
    pub tcp: *mut uv_tcp_t,
    pub pipe: *mut uv_pipe_t,
    pub stream: *mut uv_stream_t,
    //pub tty: *mut uv_tty_t,
}
impl From<*mut uv_stream_t> for uv_stream_mut {
    fn from(stream: *mut uv_stream_t) -> uv_stream_mut {
        uv_stream_mut { stream: stream }
    }
}
impl From<&mut uv_pipe_t> for uv_stream_mut {
    fn from(pipe: &mut uv_pipe_t) -> uv_stream_mut {
        uv_stream_mut { pipe: pipe }
    }
}
impl From<&mut uv_tcp_t> for uv_stream_mut {
    fn from(tcp: &mut uv_tcp_t) -> uv_stream_mut {
        uv_stream_mut { tcp: tcp }
    }
}
impl From<()> for uv_stream_mut {
    fn from(_: ()) -> uv_stream_mut {
        uv_stream_mut {
            pipe: ptr::null_mut(),
        }
    }
}
impl uv_stream_mut {
    pub unsafe fn is_null(&self) -> bool {
        self.pipe.is_null()
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_process_options_s<T> {
    pub exit_cb: uv_exit_cb<T>,
    pub file: *const libc::c_char,
    pub args: *mut *mut libc::c_char,
    pub env: *mut *mut libc::c_char,
    pub cwd: *const libc::c_char,
    pub flags: libc::c_uint,
    pub stdio_count: libc::c_int,
    pub stdio: *mut uv_stdio_container_t,
    pub uid: uv_uid_t,
    pub gid: uv_gid_t,
}
pub type uv_process_options_t<T> = uv_process_options_s<T>;
pub type uv_process_flags = libc::c_uint;
pub const UV_PROCESS_WINDOWS_HIDE_GUI: uv_process_flags = 64;
pub const UV_PROCESS_WINDOWS_HIDE_CONSOLE: uv_process_flags = 32;
pub const UV_PROCESS_WINDOWS_HIDE: uv_process_flags = 16;
pub const UV_PROCESS_DETACHED: uv_process_flags = 8;
pub const UV_PROCESS_WINDOWS_VERBATIM_ARGUMENTS: uv_process_flags = 4;
pub const UV_PROCESS_SETGID: uv_process_flags = 2;
pub const UV_PROCESS_SETUID: uv_process_flags = 1;
pub type uv_process_signal = u8;
pub const SIGKILL: uv_process_signal = 9;
pub const SIGTERM: uv_process_signal = 15;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_getaddrinfo_s<'a> {
    pub data: *mut libc::c_void,
    pub type_0: uv_req_type,
    pub reserved: [*mut libc::c_void; 6],
    pub loop_0: *mut uv_loop_t,
    pub work_req: uv__work,
    pub cb: uv_getaddrinfo_cb,
    pub hints: *mut addrinfo<'a>,
    pub hostname: *mut libc::c_char,
    pub service: *mut libc::c_char,
    pub addrinfo: *mut addrinfo<'a>,
    pub retcode: libc::c_int,
}
impl<'a> Default for uv_getaddrinfo_s<'a> {
    fn default() -> uv_getaddrinfo_s<'a> {
        uv_getaddrinfo_t {
            data: ptr::null_mut(),
            type_0: UV_UNKNOWN_REQ,
            reserved: [ptr::null_mut(); 6],
            loop_0: ptr::null_mut(),
            work_req: uv__work {
                work: None,
                done: None,
                loop_0: ptr::null_mut(),
                wq: [ptr::null_mut(); 2],
            },
            cb: None,
            hints: ptr::null_mut(),
            hostname: ptr::null_mut(),
            service: ptr::null_mut(),
            addrinfo: ptr::null_mut(),
            retcode: 0,
        }
    }
}
pub type uv_getaddrinfo_cb =
    Option<unsafe extern "C" fn(_: *mut uv_getaddrinfo_t, _: libc::c_int, _: *mut addrinfo) -> ()>;
pub type uv_getaddrinfo_t<'a> = uv_getaddrinfo_s<'a>;
