use crate::*;
use std::{mem, ptr};

#[allow(dead_code)]
const ADDRESS_MAX_SIZE: usize = 256;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SocketWatcher<'a> {
    pub addr: [libc::c_char; ADDRESS_MAX_SIZE],
    pub uv: sw_uv<'a>,
    pub stream: uv_stream_mut,
    pub data: *mut libc::c_void,
    pub cb: socket_cb,
    pub close_cb: socket_close_cb,
    pub events: *mut MultiQueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sw_uv<'a> {
    pub tcp: sw_uv_tcp<'a>,
    pub pipe: sw_uv_pipe,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sw_uv_pipe {
    pub handle: uv_pipe_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sw_uv_tcp<'a> {
    pub handle: uv_tcp_t,
    pub addrinfo: *mut addrinfo<'a>,
}

pub type socket_close_cb =
    Option<unsafe extern "C" fn(_: *mut SocketWatcher, _: *mut libc::c_void) -> ()>;
pub type socket_cb =
    Option<unsafe extern "C" fn(_: *mut SocketWatcher, _: libc::c_int, _: *mut libc::c_void) -> ()>;

#[no_mangle]
pub unsafe extern "C" fn socket_watcher_init(
    loop_0: &mut Loop,
    mut watcher: &mut SocketWatcher,
    endpoint: *const libc::c_char,
) -> libc::c_int {
    xstrlcpy(
        (*watcher).addr.as_mut_ptr(),
        endpoint,
        mem::size_of::<[libc::c_char; ADDRESS_MAX_SIZE]>(),
    );
    let addr: *mut libc::c_char = watcher.addr.as_mut_ptr();
    let host_end: *mut libc::c_char = strrchr(addr, ':');
    if !host_end.is_null() && addr != host_end {
        // Split user specified address into two strings, addr(hostname) and port.
        // The port part in watcher->addr will be updated later.
        *host_end = 0;
        let mut port: *mut libc::c_char = host_end.offset(1);
        let mut iport: intmax_t = 0;

        let ok = try_getdigits(&mut (port as *mut libc::c_uchar), &mut iport);
        if !ok || iport < 0 || iport > u16::max_value() as i64 {
            ELOG!("Invalid port: %s", port);
            return UV_EINVAL as libc::c_int;
        }

        if *port == 0 {
            // When no port is given, (uv_)getaddrinfo expects NULL otherwise the
            // implementation may attempt to lookup the service by name (and fail)
            port = ptr::null_mut()
        }

        let mut request: uv_getaddrinfo_t = Default::default();

        let retval: libc::c_int =
            uv_getaddrinfo(&mut loop_0.uv, &mut request, None, addr, port, &mut {
                let mut ai = addrinfo::default();
                ai.ai_family = AF_UNSPEC as i32;
                ai.ai_socktype = SOCK_STREAM as libc::c_int;
                ai
            });

        if retval != 0 {
            ELOG!("Host lookup failed: %s", endpoint);
            return retval;
        }
        watcher.uv.tcp.addrinfo = request.addrinfo;

        uv_tcp_init(&mut loop_0.uv, &mut watcher.uv.tcp.handle);
        uv_tcp_nodelay(&mut watcher.uv.tcp.handle, 1);
        watcher.stream = (&mut watcher.uv.tcp.handle).into()
    } else {
        uv_pipe_init(&mut loop_0.uv, &mut watcher.uv.pipe.handle, 0);
        watcher.stream = (&mut watcher.uv.pipe.handle).into()
    }

    (*watcher.stream.stream).data = watcher as *mut _ as *mut libc::c_void;
    watcher.cb = None;
    watcher.close_cb = None;
    watcher.events = ptr::null_mut();
    watcher.data = ptr::null_mut();
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn socket_watcher_start(
    mut watcher: &mut SocketWatcher,
    backlog: libc::c_int,
    cb: socket_cb,
) -> libc::c_int {
    watcher.cb = cb;
    let mut result = UV_EINVAL as libc::c_int;

    if (*watcher.stream.stream).type_0 == UV_TCP {
        for ai in addrinfo::p_iter(watcher.uv.tcp.addrinfo.as_ref()) {
            result = uv_tcp_bind(&mut watcher.uv.tcp.handle, ai.ai_addr, 0);
            if result != 0 {
                continue;
            }
            result = uv_listen(watcher.stream, backlog, Some(connection_cb));
            if result == 0 {
                let mut sas = sockaddr_storage::default();

                // When the endpoint in socket_watcher_init() didn't specify a port
                // number, a free random port number will be assigned. sin_port will
                // contain 0 in this case, unless uv_tcp_getsockname() is used first.
                let namelen = mem::size_of::<sockaddr_storage>() as libc::c_int;
                uv_tcp_getsockname(&mut watcher.uv.tcp.handle, &mut sas, &namelen);
                let port: u16 = if sas.ss_family == AF_INET {
                    (*(&mut sas as *mut _ as *mut sockaddr_in)).sin_port
                } else {
                    (*(&mut sas as *mut sockaddr_storage as *mut sockaddr_in6)).sin6_port
                };
                // v:servername uses the string from watcher->addr
                let len = strlen(watcher.addr.as_mut_ptr());
                snprintf(
                    watcher.addr.as_mut_ptr().offset(len as isize),
                    (mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong).wrapping_sub(len),
                    std::ffi::CString::new(":%u")
                        .expect("CString::new failed")
                        .as_ptr(),
                    ntohs(port) as libc::c_int,
                );
                break;
            }
        }
        uv_freeaddrinfo(watcher.uv.tcp.addrinfo);
    } else {
        result = uv_pipe_bind(&mut watcher.uv.pipe.handle, watcher.addr.as_mut_ptr());
        if result == 0 {
            result = uv_listen(watcher.stream, backlog, Some(connection_cb))
        }
    }

    c_assert!(result <= 0); // libuv should return negative error code or zero.
    if result < 0 {
        if result == UV_EACCES as libc::c_int {
            // Libuv converts ENOENT to EACCES for Windows compatibility, but if
            // the parent directory does not exist, ENOENT would be more accurate.
            *path_tail(watcher.addr.as_mut_ptr() as *mut libc::c_uchar) = 0;
            if !os_path_exists(watcher.addr.as_mut_ptr() as *mut libc::c_uchar) {
                result = UV_ENOENT as libc::c_int
            }
        }
        return result;
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn socket_watcher_accept(
    watcher: &mut SocketWatcher,
    stream: &mut Stream,
) -> libc::c_int {
    let client: uv_stream_mut;

    if (*watcher.stream.stream).type_0 == UV_TCP {
        client = (&mut stream.uv.tcp).into();
        uv_tcp_init(watcher.uv.tcp.handle.loop_0, client.tcp);
        uv_tcp_nodelay(client.tcp, 1);
    } else {
        client = (&mut stream.uv.pipe).into();
        uv_pipe_init(watcher.uv.pipe.handle.loop_0, client.pipe, 0);
    }

    let result: libc::c_int = uv_accept(watcher.stream, client);

    if result != 0 {
        uv_close(client.stream, None);
        return result;
    }

    stream_init(ptr::null_mut(), stream, -1, client.into());
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn socket_watcher_close(
    mut watcher: &mut SocketWatcher,
    cb: socket_close_cb,
) {
    watcher.close_cb = cb;
    uv_close(watcher.stream.stream, Some(close_cb));
}

unsafe extern "C" fn connection_event(argv: *mut *mut libc::c_void) {
    let watcher = (*argv.offset(0)).cast::<SocketWatcher>().as_mut().unwrap();
    let status: libc::c_int = *argv.offset(1) as uintptr_t as libc::c_int;
    watcher.cb.expect("non-null function pointer")(watcher, status, watcher.data);
}

unsafe extern "C" fn connection_cb(handle: &mut uv_stream_t, status: libc::c_int) {
    let watcher = handle.data.cast::<SocketWatcher>().as_mut().unwrap();
    CREATE_EVENT(
        watcher.events.as_mut(),
        connection_event,
        vargs!(watcher as *mut _, status as uintptr_t),
    );
}

unsafe extern "C" fn close_cb(handle: &mut uv_handle_t) {
    let watcher = handle.data.cast::<SocketWatcher>().as_mut().unwrap();
    if let Some(cb) = watcher.close_cb {
        cb(watcher, watcher.data);
    };
}

unsafe extern "C" fn connect_cb(req: &mut uv_connect_t, status: libc::c_int) {
    let ret_status = req.data.cast::<libc::c_int>().as_mut().unwrap();
    *ret_status = status;
    if status != 0 {
        uv_close(req.handle, None);
    };
}

#[no_mangle]
pub unsafe extern "C" fn socket_connect(
    loop_0: &mut Loop,
    stream: &mut Stream,
    is_tcp: bool,
    address: *const libc::c_char,
    timeout: libc::c_int,
    error: &mut *const u8,
) -> bool {
    let mut success: bool = false;
    let mut status: libc::c_int = 0;
    let mut req: uv_connect_t = Default::default();
    req.data = &mut status as *mut libc::c_int as *mut libc::c_void;
    let mut uv_stream = uv_stream_mut {
        stream: ptr::null_mut(),
    };

    let mut addr_req: uv_getaddrinfo_t = Default::default();
    addr_req.addrinfo = ptr::null_mut();
    let mut addr: *mut libc::c_char = ptr::null_mut();

    enum TcpRetry<'a> {
        No,
        Yes(&'a addrinfo<'a>),
    }
    impl<'a> TcpRetry<'a> {
        fn tcp_retry_next(&self) -> Option<&'a addrinfo<'a>> {
            match self {
                TcpRetry::No => None,
                TcpRetry::Yes(addrinfo) => addrinfo.ai_next,
            }
        }
    }
    enum goto<'a> {
        Cleanup,
        LoopProcess(TcpRetry<'a>),
    }

    let mut finalize: goto = (|| {
        if is_tcp {
            addr = c_xstrdup(address);
            let host_end = strrchr(addr, ':');
            if host_end.is_null() {
                *error = gettext("tcp address must be host:port");
                return goto::Cleanup;
            }
            *host_end = 0;

            let hints = {
                let mut hints = addrinfo::default();
                hints.ai_flags = AI_NUMERICSERV;
                hints.ai_family = AF_UNSPEC as i32;
                hints.ai_socktype = SOCK_STREAM as libc::c_int;
                hints
            };
            let retval = uv_getaddrinfo(
                &mut loop_0.uv,
                &mut addr_req,
                None,
                addr,
                host_end.offset(1),
                &hints,
            );
            if retval != 0 {
                *error = gettext("failed to lookup host or port");
                return goto::Cleanup;
            }
            let addrinfo = addr_req.addrinfo.as_ref().unwrap();
            return goto::LoopProcess(TcpRetry::Yes(addrinfo));
        } else {
            let pipe: &mut uv_pipe_t = &mut stream.uv.pipe;
            uv_pipe_init(&mut loop_0.uv, pipe, 0);
            uv_pipe_connect(&mut req, pipe, address, Some(connect_cb));
            uv_stream.pipe = pipe;
            return goto::LoopProcess(TcpRetry::No);
        }
    })();
    loop {
        finalize = match finalize {
            goto::LoopProcess(tcp_retry) => {
                if let TcpRetry::Yes(addrinfo) = tcp_retry {
                    let tcp: &mut uv_tcp_t = &mut stream.uv.tcp;
                    uv_tcp_init(&mut loop_0.uv, tcp);
                    uv_tcp_nodelay(tcp, 1);
                    uv_tcp_connect(&mut req, tcp, (*addrinfo).ai_addr, Some(connect_cb));
                    uv_stream.tcp = tcp;
                }
                status = 1;
                LOOP_PROCESS_EVENTS_UNTIL(&mut main_loop, None, timeout, |_| *&status != 1);
                if status == 0 {
                    // -V547
                    stream_init(ptr::null_mut(), stream, -1, uv_stream.into());
                    success = true;
                    goto::Cleanup
                } else if let Some(addrinfo) = tcp_retry.tcp_retry_next() {
                    goto::LoopProcess(TcpRetry::Yes(addrinfo))
                } else {
                    *error = gettext("connection refused");
                    goto::Cleanup
                }
            }
            goto::Cleanup => break,
        }
    }
    xfree(addr);
    uv_freeaddrinfo(addr_req.addrinfo);
    return success;
}
