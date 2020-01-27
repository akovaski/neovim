//! Specialized ring buffer. This is basically an array !that wraps read/write
//! pointers around the memory region. It should be more efficient than the old
//! RBuffer which required memmove() calls to relocate read/write positions.
//!
//! The main purpose of RBuffer is simplify memory management when reading from
//! uv_stream_t instances:
//!
//! - The event loop writes data to a RBuffer, advancing the write pointer
//! - The main loop reads data, advancing the read pointer
//! - If the buffer becomes full(size == capacity) the rstream is temporarily
//!   stopped(automatic backpressure handling)
//!
//! Reference: http://en.wikipedia.org/wiki/Circular_buffer

use crate::memory::*;
use std::cmp::min;
use std::ptr;

mod ffi {
    use super::*;
    #[no_mangle]
    unsafe extern "C" fn rbuffer_size(buf: *mut RBuffer<libc::c_void>) -> libc::size_t {
        super::rbuffer_size(buf)
    }
    #[no_mangle]
    unsafe extern "C" fn rbuffer_new(capacity: libc::size_t) -> *mut RBuffer<libc::c_void> {
        super::rbuffer_new(capacity)
    }
    #[no_mangle]
    unsafe extern "C" fn rbuffer_free(buf: *mut RBuffer<libc::c_void>) {
        super::rbuffer_free(buf)
    }
    #[no_mangle]
    unsafe extern "C" fn rbuffer_capacity(buf: *mut RBuffer<libc::c_void>) -> libc::size_t {
        super::rbuffer_capacity(buf)
    }
    #[no_mangle]
    unsafe extern "C" fn rbuffer_space(buf: *mut RBuffer<libc::c_void>) -> libc::size_t {
        super::rbuffer_space(buf)
    }
    #[no_mangle]
    unsafe extern "C" fn rbuffer_write_ptr(
        buf: *mut RBuffer<libc::c_void>,
        write_count: *mut libc::size_t,
    ) -> *mut libc::c_char {
        super::rbuffer_write_ptr(buf, write_count)
    }
    #[no_mangle]
    unsafe extern "C" fn rbuffer_reset(buf: *mut RBuffer<libc::c_void>) {
        super::rbuffer_reset(buf)
    }
    #[no_mangle]
    unsafe extern "C" fn rbuffer_produced(buf: *mut RBuffer<libc::c_void>, count: libc::size_t) {
        super::rbuffer_produced(buf, count)
    }
    #[no_mangle]
    unsafe extern "C" fn rbuffer_read_ptr(
        buf: *mut RBuffer<libc::c_void>,
        read_count: *mut libc::size_t,
    ) -> *mut libc::c_char {
        super::rbuffer_read_ptr(buf, read_count)
    }
    #[no_mangle]
    unsafe extern "C" fn rbuffer_consumed(buf: *mut RBuffer<libc::c_void>, count: libc::size_t) {
        super::rbuffer_consumed(buf, count)
    }
    #[no_mangle]
    unsafe extern "C" fn rbuffer_write(
        buf: *mut RBuffer<libc::c_void>,
        src: *const libc::c_char,
        src_size: libc::size_t,
    ) -> libc::size_t {
        super::rbuffer_write(buf, src, src_size)
    }
    #[no_mangle]
    unsafe extern "C" fn rbuffer_read(
        buf: *mut RBuffer<libc::c_void>,
        dst: *mut libc::c_char,
        dst_size: libc::size_t,
    ) -> libc::size_t {
        super::rbuffer_read(buf, dst, dst_size)
    }
    #[no_mangle]
    unsafe extern "C" fn rbuffer_get(
        buf: *mut RBuffer<libc::c_void>,
        index: libc::size_t,
    ) -> *mut libc::c_char {
        super::rbuffer_get(buf, index)
    }
    #[no_mangle]
    unsafe extern "C" fn rbuffer_cmp(
        buf: *mut RBuffer<libc::c_void>,
        str: *const libc::c_char,
        count: libc::size_t,
    ) -> libc::c_int {
        super::rbuffer_cmp(buf, str, count)
    }
}

/// Macros that simplify working with the read/write pointers directly by hiding
/// ring buffer wrap logic. Some examples:
///
/// - Pass the write pointer to a function(write_data) that incrementally
///   produces data, returning the number of bytes actually written to the
///   ring buffer:
///
///       RBUFFER_UNTIL_FULL(rbuf, ptr, cnt)
///         rbuffer_produced(rbuf, write_data(state, ptr, cnt));
///
/// - Pass the read pointer to a function(read_data) that incrementally
///   consumes data, returning the number of bytes actually read from the
///   ring buffer:
///
///       RBUFFER_UNTIL_EMPTY(rbuf, ptr, cnt)
///         rbuffer_consumed(rbuf, read_data(state, ptr, cnt));
///
/// Note that the rbuffer_{produced,consumed} calls are necessary or these macros
/// create infinite loops
macro_rules! RBUFFER_UNTIL_EMPTY {
    ( $buf:ident, $rptr:ident, $rcnt:ident, $loop_inner:expr) => {
        let mut $rcnt: libc::size_t = 0;
        let mut _r: libc::size_t = 1;
        while _r != 0 {
            let mut $rptr: *mut libc::c_char = rbuffer_read_ptr($buf, &mut $rcnt);
            while (*$buf).size != 0 {
                $loop_inner
                $rptr = rbuffer_read_ptr($buf, &mut $rcnt)
            }
            _r = 0
        }
    }
}
macro_rules! RBUFFER_UNTIL_FULL {
    ( $buf:ident, $wptr:ident, $wcnt:ident, $loop_inner:expr) => {
        let mut $wcnt: libc::size_t = 0;
        let mut _r: libc::size_t = 1;
        while _r != 0 {
            let mut $wptr: *mut libc::c_char = rbuffer_write_ptr($buf, &mut $wcnt);
            while rbuffer_space($buf) != 0 {
                $loop_inner
                $wptr = rbuffer_write_ptr($buf, &mut $wcnt)
            }
            _r = 0 as libc::c_int as libc::size_t;
        }
    }
}

// Iteration
// TODO: RBUFFER_EACH, RBUFFER_EACH_REVERSE
//       These are only used in os/input.c and tui/input.c

pub type RBuffer<T> = rbuffer<T>;
/// Type of function invoked during certain events:
///   - When the RBuffer switches to the full state
///   - When the RBuffer switches to the non-full state
pub type rbuffer_callback<T> = Option<unsafe extern "C" fn(_: *mut RBuffer<T>, _: *mut T) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbuffer<T> {
    pub full_cb: rbuffer_callback<T>,
    pub nonfull_cb: rbuffer_callback<T>,
    pub data: *mut T,
    pub size: libc::size_t,
    pub temp: *mut libc::c_char, // helper memory used to by rbuffer_reset if required
    pub end_ptr: *mut libc::c_char,
    pub read_ptr: *mut libc::c_char,
    pub write_ptr: *mut libc::c_char,
    pub start_ptr: [libc::c_char; 0],
}

/// Creates a new `RBuffer` instance.
pub unsafe fn rbuffer_new<T>(mut capacity: libc::size_t) -> *mut RBuffer<T> {
    if capacity == 0 {
        capacity = 0x10000;
    }
    let mut rv: *mut RBuffer<T> = xcalloc(1, std::mem::size_of::<RBuffer<T>>() + capacity);
    (*rv).nonfull_cb = None;
    (*rv).full_cb = None;
    (*rv).data = ptr::null_mut();
    (*rv).size = 0;
    (*rv).read_ptr = (*rv).start_ptr.as_mut_ptr();
    (*rv).write_ptr = (*rv).start_ptr.as_mut_ptr();
    (*rv).end_ptr = (*rv).start_ptr.as_mut_ptr().offset(capacity as isize);
    (*rv).temp = ptr::null_mut();
    return rv;
}

pub unsafe fn rbuffer_free<T>(buf: *mut RBuffer<T>) {
    xfree((*buf).temp);
    xfree(buf);
}
pub unsafe fn rbuffer_size<T>(buf: *mut RBuffer<T>) -> libc::size_t {
    return (*buf).size;
}

pub unsafe fn rbuffer_capacity<T>(buf: *mut RBuffer<T>) -> libc::size_t {
    return (*buf).end_ptr.offset_from((*buf).start_ptr.as_mut_ptr()) as libc::size_t;
}

pub unsafe fn rbuffer_space<T>(buf: *mut RBuffer<T>) -> libc::size_t {
    return rbuffer_capacity(buf) - (*buf).size;
}

/// Return a pointer to a raw buffer containing the first empty slot available
/// for writing. The second argument is a pointer to the maximum number of
/// bytes that could be written.
///
/// It is necessary to call this function twice to ensure all empty space was
/// used. See RBUFFER_UNTIL_FULL for a macro that simplifies this task.
pub unsafe fn rbuffer_write_ptr<T>(
    buf: *mut RBuffer<T>,
    write_count: *mut libc::size_t,
) -> *mut libc::c_char {
    if (*buf).size == rbuffer_capacity(buf) {
        *write_count = 0;
        return ptr::null_mut();
    }

    if (*buf).write_ptr >= (*buf).read_ptr {
        *write_count = (*buf).end_ptr.offset_from((*buf).write_ptr) as libc::size_t
    } else {
        *write_count = (*buf).read_ptr.offset_from((*buf).write_ptr) as libc::size_t
    }

    return (*buf).write_ptr;
}

/// Reset an RBuffer so read_ptr is at the beginning of the memory. If
/// necessary, this moves existing data by allocating temporary memory.
pub unsafe fn rbuffer_reset<T>(mut buf: *mut RBuffer<T>) {
    let temp_size: libc::size_t = rbuffer_size(buf);
    if temp_size != 0 {
        if (*buf).temp.is_null() {
            (*buf).temp = xcalloc(1, rbuffer_capacity(buf));
        }
        rbuffer_read(buf, (*buf).temp, (*buf).size);
    }
    (*buf).write_ptr = (*buf).start_ptr.as_mut_ptr();
    (*buf).read_ptr = (*buf).start_ptr.as_mut_ptr();
    if temp_size != 0 {
        rbuffer_write(buf, (*buf).temp, temp_size);
    };
}

/// Adjust `rbuffer` write pointer to reflect produced data. This is called
/// automatically by `rbuffer_write`, but when using `rbuffer_write_ptr`
/// directly, this needs to called after the data was copied to the internal
/// buffer. The write pointer will be wrapped if required.
pub unsafe fn rbuffer_produced<T>(mut buf: *mut RBuffer<T>, count: libc::size_t) {
    c_assert!(count != 0 && count <= rbuffer_space(buf));

    (*buf).write_ptr = (*buf).write_ptr.offset(count as isize);
    if (*buf).write_ptr >= (*buf).end_ptr {
        // wrap around
        (*buf).write_ptr = (*buf).write_ptr.offset(-(rbuffer_capacity(buf) as isize))
    }

    (*buf).size += count;
    if rbuffer_space(buf) == 0 {
        if let Some(full_cb) = (*buf).full_cb {
            full_cb(buf, (*buf).data);
        }
    }
}

/// Return a pointer to a raw buffer containing the first byte available
/// for reading. The second argument is a pointer to the maximum number of
/// bytes that could be read.
///
/// It is necessary to call this function twice to ensure all available bytes
/// were read. See RBUFFER_UNTIL_EMPTY for a macro that simplifies this task.
pub unsafe fn rbuffer_read_ptr<T>(
    buf: *mut RBuffer<T>,
    read_count: *mut libc::size_t,
) -> *mut libc::c_char {
    if (*buf).size == 0 {
        *read_count = 0;
        return (*buf).read_ptr;
    }

    if (*buf).read_ptr < (*buf).write_ptr {
        *read_count = (*buf).write_ptr.offset_from((*buf).read_ptr) as libc::size_t
    } else {
        *read_count = (*buf).end_ptr.offset_from((*buf).read_ptr) as libc::size_t
    }

    return (*buf).read_ptr;
}

/// Adjust `rbuffer` read pointer to reflect consumed data. This is called
/// automatically by `rbuffer_read`, but when using `rbuffer_read_ptr`
/// directly, this needs to called after the data was copied from the internal
/// buffer. The read pointer will be wrapped if required.
pub unsafe fn rbuffer_consumed<T>(mut buf: *mut RBuffer<T>, count: libc::size_t) {
    c_assert!(count != 0 && count <= (*buf).size);

    (*buf).read_ptr = (*buf).read_ptr.offset(count as isize);
    if (*buf).read_ptr >= (*buf).end_ptr {
        (*buf).read_ptr = (*buf).read_ptr.offset(-(rbuffer_capacity(buf) as isize))
    }

    let was_full: bool = (*buf).size == rbuffer_capacity(buf);
    (*buf).size -= count;
    if was_full {
        if let Some(nonfull_cb) = (*buf).nonfull_cb {
            nonfull_cb(buf, (*buf).data);
        }
    }
}

/// Higher level functions for copying from/to RBuffer instances and data
/// pointers
pub unsafe fn rbuffer_write<T>(
    buf: *mut RBuffer<T>,
    mut src: *const libc::c_char,
    mut src_size: libc::size_t,
) -> libc::size_t {
    let size: libc::size_t = src_size;

    RBUFFER_UNTIL_FULL!(buf, wptr, wcnt, {
        let copy_count: libc::size_t = min(src_size, wcnt);
        memcpy(wptr, src, copy_count);
        rbuffer_produced(buf, copy_count);

        src_size -= copy_count;
        if src_size == 0 {
            return size;
        }

        src = src.offset(copy_count as isize);
    });

    return size - src_size;
}

pub unsafe fn rbuffer_read<T>(
    buf: *mut RBuffer<T>,
    mut dst: *mut libc::c_char,
    mut dst_size: libc::size_t,
) -> libc::size_t {
    let size: libc::size_t = dst_size;

    RBUFFER_UNTIL_EMPTY!(buf, rptr, rcnt, {
        let copy_count: libc::size_t = min(dst_size, rcnt);
        memcpy(dst, rptr, copy_count);
        rbuffer_consumed(buf, copy_count);

        dst_size -= copy_count;
        if dst_size == 0 {
            return size;
        }

        dst = dst.offset(copy_count as isize);
    });

    return size - dst_size;
}

pub unsafe fn rbuffer_get<T>(buf: *mut RBuffer<T>, index: libc::size_t) -> *mut libc::c_char {
    c_assert!(index < (*buf).size);
    let mut rptr: *mut libc::c_char = (*buf).read_ptr.offset(index as isize);
    if rptr >= (*buf).end_ptr {
        rptr = rptr.offset(-(rbuffer_capacity(buf) as isize))
    }
    return rptr;
}

pub unsafe fn rbuffer_cmp<T>(
    buf: *mut RBuffer<T>,
    str: *const libc::c_char,
    mut count: libc::size_t,
) -> libc::c_int {
    c_assert!(count <= (*buf).size);
    let mut rcnt: libc::size_t = 0;
    rbuffer_read_ptr(buf, &mut rcnt);
    let n: libc::size_t = min(count, rcnt);
    let rv: libc::c_int = memcmp(str, (*buf).read_ptr, n);
    count -= n;
    let remaining: libc::size_t = (*buf).size - rcnt;

    if rv != 0 || count == 0 || remaining == 0 {
        return rv;
    }

    return memcmp(str.offset(n as isize), (*buf).start_ptr.as_mut_ptr(), count);
}
