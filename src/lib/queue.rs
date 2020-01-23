// Queue implemented by circularly-linked list.
//
// Adapted from libuv. Simpler and more efficient than klist.h for implementing
// queues that support arbitrary insertion/removal.
//
// Copyright (c) 2013, Ben Noordhuis <info@bnoordhuis.nl>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

#[derive(Copy, Clone)]
#[repr(C)]
pub struct QUEUE {
    pub next: *mut QUEUE,
    pub prev: *mut QUEUE,
}

// Public macros.
macro_rules! QUEUE_DATA {
    ($q:expr, $type:ty, $field:ident) => {
        ($q as *mut u8).offset(-offset_of!($type, $field)) as *mut $type
    };
}

// Important note: mutating the list while QUEUE_FOREACH is
// iterating over its elements results in undefined behavior.

#[inline]
pub unsafe fn QUEUE_EMPTY(q: *const QUEUE) -> bool {
    return q == (*q).next;
}

pub unsafe fn QUEUE_HEAD(q: QUEUE) -> *mut QUEUE {
    q.next
}

#[inline]
pub unsafe fn QUEUE_INIT(q: *mut QUEUE) {
    (*q).next = q;
    (*q).prev = q;
}
#[inline]
pub unsafe fn QUEUE_INSERT_TAIL(h: *mut QUEUE, q: *mut QUEUE) {
    (*q).next = h;
    (*q).prev = (*h).prev;
    (*(*q).prev).next = q;
    (*h).prev = q;
}
#[inline]
pub unsafe fn QUEUE_REMOVE(q: *mut QUEUE) {
    (*(*q).prev).next = (*q).next;
    (*(*q).next).prev = (*q).prev;
}
