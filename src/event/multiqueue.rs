//! Multi-level queue for selective async event processing.
//! Not threadsafe; access must be synchronized externally.
//!
//! Multiqueue supports a parent-child relationship with these properties:
//! - pushing a node to a child queue will push a corresponding link node to the
//!   parent queue
//! - removing a link node from a parent queue will remove the next node
//!   in the linked child queue
//! - removing a node from a child queue will remove the corresponding link node
//!   in the parent queue
//!
//! These properties allow Nvim to organize and process events from different
//! sources with a certain degree of control. How the multiqueue is used:
//!
//!                         +----------------+
//!                         |   Main loop    |
//!                         +----------------+
//!
//!                         +----------------+
//!         +-------------->|   Event loop   |<------------+
//!         |               +--+-------------+             |
//!         |                  ^           ^               |
//!         |                  |           |               |
//!    +-----------+   +-----------+    +---------+    +---------+
//!    | Channel 1 |   | Channel 2 |    |  Job 1  |    |  Job 2  |
//!    +-----------+   +-----------+    +---------+    +---------+
//!
//!
//! The lower boxes represent event emitters, each with its own private queue
//! having the event loop queue as the parent.
//!
//! When idle, the main loop spins the event loop which queues events from many
//! sources (channels, jobs, user...). Each event emitter pushes events to its
//! private queue which is propagated to the event loop queue. When the main loop
//! consumes an event, the corresponding event is removed from the emitter's
//! queue.
//!
//! The main reason for this queue hierarchy is to allow focusing on a single
//! event emitter while blocking the main loop. For example, if the `jobwait`
//! VimL function is called on job1, the main loop will temporarily stop polling
//! the event loop queue and poll job1 queue instead. Same with channels, when
//! calling `rpcrequest` we want to temporarily stop processing events from
//! other sources and focus on a specific channel.

use crate::*;
use std::ptr;

pub type put_callback =
    Option<unsafe extern "C" fn(_: Option<&mut MultiQueue>, _: *mut libc::c_void) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MultiQueueItem {
    pub data: mq_item_data,
    pub link: bool, // true: current item is just a link to a node in a child queue
    pub node: QUEUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union mq_item_data {
    pub queue: *mut MultiQueue,
    pub item: mq_item_data_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mq_item_data_item {
    pub event: Event,
    pub parent_item: *mut MultiQueueItem,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MultiQueue {
    pub parent: *mut MultiQueue,
    pub headtail: QUEUE, // circularly-linked
    pub put_cb: put_callback,
    pub data: *mut libc::c_void,
    pub size: libc::size_t,
}

/// Event present on multiple queues.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MulticastEvent {
    pub event: Event,
    pub fired: bool,
    pub refcount: libc::c_int,
}

static mut NILEVENT: Event = Event {
    handler: None,
    argv: [ptr::null_mut(); 10],
};

#[no_mangle]
pub unsafe extern "C" fn multiqueue_new_parent(
    put_cb: put_callback,
    data: *mut libc::c_void,
) -> *mut MultiQueue {
    return multiqueue_new(ptr::null_mut(), put_cb, data);
}

#[no_mangle]
pub unsafe extern "C" fn multiqueue_new_child(parent: &mut MultiQueue) -> *mut MultiQueue {
    c_assert!((*parent).parent.is_null()); // parent cannot have a parent, more like a "root"
    (*parent).size = (*parent).size.wrapping_add(1);
    return multiqueue_new(parent, None, ptr::null_mut());
}

unsafe extern "C" fn multiqueue_new(
    parent: *mut MultiQueue,
    put_cb: put_callback,
    data: *mut libc::c_void,
) -> *mut MultiQueue {
    let mut rv: *mut MultiQueue = xmalloc(std::mem::size_of::<MultiQueue>());
    QUEUE_INIT(&mut (*rv).headtail);
    (*rv).size = 0;
    (*rv).parent = parent;
    (*rv).put_cb = put_cb;
    (*rv).data = data;
    return rv;
}

#[no_mangle]
pub unsafe extern "C" fn multiqueue_free(this: *mut MultiQueue) {
    c_assert!(!this.is_null());
    while !QUEUE_EMPTY(&mut (*this).headtail) {
        let q: *mut QUEUE = (*this).headtail.next;
        let item: *mut MultiQueueItem = multiqueue_node_data(q.as_mut());
        if !(*this).parent.is_null() {
            QUEUE_REMOVE(&mut (*(*item).data.item.parent_item).node);
            xfree((*item).data.item.parent_item);
        }
        QUEUE_REMOVE(q);
        xfree(item);
    }
    xfree(this);
}

/// Removes the next item and returns its Event.
#[no_mangle]
pub unsafe extern "C" fn multiqueue_get(this: Option<&mut MultiQueue>) -> Event {
    let this = this.unwrap();
    return if multiqueue_empty(Some(this)) {
        NILEVENT
    } else {
        multiqueue_remove(Some(this))
    };
}

pub unsafe fn multiqueue_put(
    this: Option<&mut MultiQueue>,
    cb: argv_callback,
    args: &[*mut libc::c_void],
) {
    multiqueue_put_event(this, event_create(cb, args));
}
#[no_mangle]
pub unsafe extern "C" fn multiqueue_put_event(this: Option<&mut MultiQueue>, event: Event) {
    let this = this.unwrap();
    multiqueue_push(this, event);
    if !(*this).parent.is_null() {
        if let Some(put_cb) = (*(*this).parent).put_cb {
            put_cb((*this).parent.as_mut(), (*(*this).parent).data);
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn multiqueue_process_events(this: Option<&mut MultiQueue>) {
    c_assert!(this.is_some());
    let this = this.unwrap();
    while !multiqueue_empty(Some(this)) {
        let mut event: Event = multiqueue_remove(Some(this));
        if let Some(handler) = event.handler {
            handler(event.argv.as_mut_ptr());
        }
    }
}

/// Removes all events without processing them.
#[no_mangle]
pub unsafe extern "C" fn multiqueue_purge_events(this: Option<&mut MultiQueue>) {
    c_assert!(this.is_some());
    let this = this.unwrap();
    while !multiqueue_empty(Some(this)) {
        multiqueue_remove(Some(this));
    }
}

#[no_mangle]
pub unsafe extern "C" fn multiqueue_empty(this: Option<&MultiQueue>) -> bool {
    c_assert!(this.is_some());
    let this = this.unwrap();
    return QUEUE_EMPTY(&(*this).headtail);
}

#[no_mangle]
pub unsafe extern "C" fn multiqueue_replace_parent(
    this: Option<&mut MultiQueue>,
    new_parent: Option<&mut MultiQueue>,
) {
    let this = this.unwrap();
    c_assert!(multiqueue_empty(Some(this)));
    (*this).parent = new_parent.map_or(ptr::null_mut(), |x| x as *mut _);
}

/// Gets the count of all events currently in the queue.
#[no_mangle]
pub unsafe extern "C" fn multiqueue_size(this: &mut MultiQueue) -> libc::size_t {
    return (*this).size;
}

/// Gets an Event from an item.
///
/// @param remove   Remove the node from its queue, and free it.
unsafe extern "C" fn multiqueueitem_get_event(
    item: Option<&mut MultiQueueItem>,
    remove: bool,
) -> Event {
    c_assert!(item.is_some());
    let item = item.unwrap();
    let ev: Event;
    if (*item).link {
        // get the next node in the linked queue
        let linked: *mut MultiQueue = (*item).data.queue;
        c_assert!(!multiqueue_empty(linked.as_ref()));
        let child: *mut MultiQueueItem =
            multiqueue_node_data(QUEUE_HEAD((*linked).headtail).as_mut());
        ev = (*child).data.item.event;
        // remove the child node
        if remove {
            QUEUE_REMOVE(&mut (*child).node);
            xfree(child);
        }
    } else {
        // remove the corresponding link node in the parent queue
        if remove && !(*item).data.item.parent_item.is_null() {
            QUEUE_REMOVE(&mut (*(*item).data.item.parent_item).node);
            xfree((*item).data.item.parent_item);
            (*item).data.item.parent_item = ptr::null_mut();
        }
        ev = (*item).data.item.event;
    }
    return ev;
}

unsafe extern "C" fn multiqueue_remove(this: Option<&mut MultiQueue>) -> Event {
    let this = this.unwrap();
    c_assert!(!multiqueue_empty(Some(this)));
    let h: *mut QUEUE = QUEUE_HEAD((*this).headtail);
    QUEUE_REMOVE(h);
    let item: *mut MultiQueueItem = multiqueue_node_data(h.as_mut());
    c_assert!(!(*item).link || (*this).parent.is_null()); // Only a parent queue has link-nodes
    let ev: Event = multiqueueitem_get_event(item.as_mut(), true);
    (*this).size = (*this).size.wrapping_sub(1);
    xfree(item);
    return ev;
}

unsafe fn multiqueue_push(mut this: &mut MultiQueue, event: Event) {
    let mut item: *mut MultiQueueItem = xmalloc(std::mem::size_of::<MultiQueueItem>());
    (*item).link = false;
    (*item).data.item.event = event;
    (*item).data.item.parent_item = ptr::null_mut();
    QUEUE_INSERT_TAIL(&mut (*this).headtail, &mut (*item).node);
    if !(*this).parent.is_null() {
        // push link node to the parent queue
        (*item).data.item.parent_item = xmalloc(std::mem::size_of::<MultiQueueItem>());
        (*(*item).data.item.parent_item).link = true;
        (*(*item).data.item.parent_item).data.queue = this;
        QUEUE_INSERT_TAIL(
            &mut (*(*this).parent).headtail,
            &mut (*(*item).data.item.parent_item).node,
        );
    }
    (*this).size = (*this).size.wrapping_add(1);
}

unsafe fn multiqueue_node_data(q: Option<&mut QUEUE>) -> *mut MultiQueueItem {
    let q = q.unwrap() as *mut QUEUE;
    QUEUE_DATA!(q, MultiQueueItem, node)
}

/// Multicasts a one-shot event to multiple queues.
///
/// The handler will be invoked once by the _first_ queue that consumes the
/// event. Later processing will do nothing (just memory cleanup).
///
/// @param ev  Event
/// @param num  Number of queues that the event will be put on
/// @return Event that is safe to put onto `num` queues
#[no_mangle]
pub unsafe extern "C" fn event_create_oneshot(ev: Event, num: libc::c_int) -> Event {
    let mut data: *mut MulticastEvent = xmalloc(std::mem::size_of::<MulticastEvent>());
    (*data).event = ev;
    (*data).fired = false;
    (*data).refcount = num;
    return event_create(Some(multiqueue_oneshot_event), vargs!(data));
}
unsafe extern "C" fn multiqueue_oneshot_event(argv: *mut *mut libc::c_void) {
    let mut data: *mut MulticastEvent = *argv.offset(0) as *mut MulticastEvent;
    if !(*data).fired {
        (*data).fired = true;
        if let Some(handler) = (*data).event.handler {
            handler((*data).event.argv.as_mut_ptr());
        }
    }
    (*data).refcount = (*data).refcount.wrapping_sub(1);
    if (*data).refcount == 0 {
        xfree(data);
    };
}
