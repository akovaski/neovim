extern "C" {
    pub type uv_signal_s;
    pub fn uv_close(handle: *mut uv_handle_t, close_cb_0: uv_close_cb);
    pub fn uv_signal_init(loop_0: *mut uv_loop_t, handle: *mut uv_signal_t) -> libc::c_int;
    pub fn uv_signal_start(
        handle: *mut uv_signal_t,
        signal_cb: uv_signal_cb,
        signum: libc::c_int,
    ) -> libc::c_int;
    pub fn uv_signal_stop(handle: *mut uv_signal_t) -> libc::c_int;
    pub fn uv_timer_init(_: *mut uv_loop_t, handle: *mut uv_timer_t) -> libc::c_int;
    pub fn uv_timer_start(
        handle: *mut uv_timer_t,
        cb: uv_timer_cb,
        timeout: u64,
        repeat: u64,
    ) -> libc::c_int;
    pub fn uv_timer_stop(handle: *mut uv_timer_t) -> libc::c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_handle_t {
    pub data: *mut libc::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut libc::c_void; 2],
    pub u: C2RustUnnamed_0,
    pub next_closing: *mut uv_handle_t,
    pub flags: libc::c_uint,
}

pub type uv_signal_cb = Option<unsafe extern "C" fn(_: *mut uv_signal_t, _: libc::c_int) -> ()>;
pub type uv_close_cb = Option<unsafe extern "C" fn(_: *mut uv_handle_t) -> ()>;
pub type uv_timer_cb = Option<unsafe extern "C" fn(_: *mut uv_timer_t) -> ()>;
pub type uv_async_cb = Option<unsafe extern "C" fn(_: *mut uv_async_t) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_signal_t {
    pub data: *mut libc::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut libc::c_void; 2],
    u: C2RustUnnamed_1,
    pub next_closing: *mut uv_handle_t,
    pub flags: libc::c_uint,
    pub signal_cb: uv_signal_cb,
    pub signum: libc::c_int,
    tree_entry: C2RustUnnamed,
    pub caught_signals: libc::c_uint,
    pub dispatched_signals: libc::c_uint,
}
pub type uv_handle_type = libc::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_timer_t {
    pub data: *mut libc::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut libc::c_void; 2],
    u: C2RustUnnamed_5,
    pub next_closing: *mut uv_handle_t,
    pub flags: libc::c_uint,
    pub timer_cb: uv_timer_cb,
    pub heap_node: [*mut libc::c_void; 3],
    pub timeout: u64,
    pub repeat: u64,
    pub start_id: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_async_t {
    pub data: *mut libc::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut libc::c_void; 2],
    u: C2RustUnnamed_3,
    pub next_closing: *mut uv_handle_t,
    pub flags: libc::c_uint,
    pub async_cb: uv_async_cb,
    pub queue: [*mut libc::c_void; 2],
    pub pending: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union uv_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_loop_t {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__io_t {
    pub cb: uv__io_cb,
    pub pending_queue: [*mut libc::c_void; 2],
    pub watcher_queue: [*mut libc::c_void; 2],
    pub pevents: libc::c_uint,
    pub events: libc::c_uint,
    pub fd: libc::c_int,
}
pub type uv__io_cb =
    Option<unsafe extern "C" fn(_: *mut uv_loop_t, _: *mut uv__io_t, _: libc::c_uint) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
struct C2RustUnnamed {
    pub rbe_left: *mut uv_signal_s,
    pub rbe_right: *mut uv_signal_s,
    pub rbe_parent: *mut uv_signal_s,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub fd: libc::c_int,
    pub reserved: [*mut libc::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
union C2RustUnnamed_1 {
    pub fd: libc::c_int,
    pub reserved: [*mut libc::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub min: *mut libc::c_void,
    pub nelts: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub fd: libc::c_int,
    pub reserved: [*mut libc::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub unused: [*mut libc::c_void; 2],
    pub count: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub fd: libc::c_int,
    pub reserved: [*mut libc::c_void; 4],
}
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
    __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct __pthread_list_t {
    __prev: *mut __pthread_list_t,
    __next: *mut __pthread_list_t,
}
type uv_rwlock_t = pthread_rwlock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
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
