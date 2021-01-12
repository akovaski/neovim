extern "C" {
    #[doc(hidden)]
    pub fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    pub fn abort() -> !;
    pub fn exit_free() -> bool;
}

pub type uintmax_t = libc::c_ulong;
pub type intmax_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;

macro_rules! vargs {
    ( $($arg:expr),* $(,)?) => {
        &[ $($arg as *mut libc::c_void),* ];
    }
}

macro_rules! offset_of {
    ( $type:ty, $field: ident ) => {{
        type tt = $type;
        let x: tt = std::mem::zeroed();
        let tt { ref $field, .. } = x;
        let offset = ($field as *const _ as usize) - (&x as *const _ as usize);
        std::mem::forget(x);
        offset as isize
    }};
}

#[allow(dead_code)]
unsafe extern "C" fn xdl_diff(
    mf1: *mut xdiff::mmfile_t,
    mf2: *mut xdiff::mmfile_t,
    xpp: *const xdiff::xpparam_t,
    xecfg: *const xdiff::xdemitconf_t,
    ecb: *mut xdiff::xdemitcb_t,
) -> libc::c_int {
    xdiff::xdl_diff(mf1, mf2, xpp, xecfg, ecb)
}

pub mod stdio_h {
    extern "C" {
        pub fn snprintf(
            _: *mut libc::c_char,
            _: libc::c_ulong,
            _: *const libc::c_char,
            _: ...
        ) -> libc::c_int;
    }
}
pub use stdio_h::*;

#[allow(unused_macros)]
macro_rules! snprintf {
    ($dest: expr, $n: expr, $str: literal, $($e: expr),+ $(,)?) => {{
        snprintf($dest as *mut i8, $n as u64, $str as *const u8 as *const i8, $($e),+)
    }}
}

pub mod socket_h {
    pub type socklen_t = libc::c_uint;
    pub type sa_family_t = libc::c_ushort;
    pub const AF_UNSPEC: sa_family_t = 0;
    pub const AF_INET: sa_family_t = 2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        pub __ss_padding: [libc::c_char; 118],
        pub __ss_align: libc::c_ulong,
    }
    impl Default for sockaddr_storage {
        fn default() -> sockaddr_storage {
            sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            }
        }
    }
    pub type __socket_type = libc::c_uint;
    pub const SOCK_NONBLOCK: __socket_type = 2048;
    pub const SOCK_CLOEXEC: __socket_type = 524288;
    pub const SOCK_PACKET: __socket_type = 10;
    pub const SOCK_DCCP: __socket_type = 6;
    pub const SOCK_SEQPACKET: __socket_type = 5;
    pub const SOCK_RDM: __socket_type = 4;
    pub const SOCK_RAW: __socket_type = 3;
    pub const SOCK_DGRAM: __socket_type = 2;
    pub const SOCK_STREAM: __socket_type = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: u32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct in6_addr {
        pub __in6_u: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2RustUnnamed {
        pub __u6_addr8: [u8; 16],
        pub __u6_addr16: [u16; 8],
        pub __u6_addr32: [u32; 4],
    }
    pub type in_port_t = u16;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [libc::c_uchar; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    pub type in_addr_t = u32;
    extern "C" {
        pub fn ntohs(__netshort: u16) -> u16;
    }
}
pub use socket_h::*;
pub mod netdb_h {
    use super::*;
    use std::ptr;

    pub const AI_NUMERICSERV: libc::c_int = 0x400;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct addrinfo<'a> {
        pub ai_flags: libc::c_int,
        pub ai_family: libc::c_int,
        pub ai_socktype: libc::c_int,
        pub ai_protocol: libc::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_addr: *const sockaddr,
        pub ai_canonname: *const libc::c_char,
        pub ai_next: Option<&'a addrinfo<'a>>,
    }
    impl<'a> addrinfo<'a> {
        pub fn p_iter(ai_p: Option<&'a addrinfo<'a>>) -> addrinfoIter<'a> {
            addrinfoIter { ai_p: ai_p }
        }
    }
    impl<'a> Default for addrinfo<'a> {
        fn default() -> Self {
            addrinfo {
                ai_flags: AI_NUMERICSERV,
                ai_family: AF_UNSPEC as i32,
                ai_socktype: SOCK_STREAM as libc::c_int,
                ai_protocol: 0,
                ai_addrlen: 0,
                ai_addr: ptr::null_mut(),
                ai_canonname: ptr::null_mut(),
                ai_next: None,
            }
        }
    }

    pub struct addrinfoIter<'a> {
        ai_p: Option<&'a addrinfo<'a>>,
    }
    impl<'a> Iterator for addrinfoIter<'a> {
        type Item = &'a addrinfo<'a>;
        fn next(&mut self) -> Option<Self::Item> {
            let ret = self.ai_p;
            self.ai_p = ret?.ai_next;
            ret
        }
    }
}
pub use netdb_h::*;
pub mod iconv_h {
    pub type iconv_t = *mut libc::c_void;
}
pub use iconv_h::*;
pub mod inttypes_h {
    use crate::*;
    extern "C" {
        pub fn strtoimax(
            __nptr: *const libc::c_char,
            __endptr: *mut *const libc::c_char,
            __base: libc::c_int,
        ) -> intmax_t;
    }
}
pub use inttypes_h::*;
pub mod errno_h {
    extern "C" {
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
pub use errno_h::*;
