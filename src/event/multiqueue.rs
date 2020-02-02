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
use std::mem;
use std::ptr;

pub type put_callback =
    Option<unsafe extern "C" fn(_: Option<&mut MultiQueue>, _: *mut libc::c_void) -> ()>;

#[repr(C)]
pub struct MultiQueueItem {
    data: mq_item_data,
    link: bool, // true: current item is just a link to a node in a child queue
    node: QUEUE,
}
impl Drop for MultiQueueItem {
    fn drop(&mut self) {
        unsafe {
            if self.link {
                // remove the child node
                match self.data.queue.as_mut() {
                    Some(linked) => {
                        let mut child = Box::from_raw(multiqueue_node_data(
                            QUEUE_HEAD(linked.headtail).as_mut().unwrap(),
                        ));
                        // prevent the child from dropping the current node
                        child.data.item.parent_item = ptr::null_mut();
                        mem::drop(child);
                    }
                    _ => (),
                }
            } else {
                // remove the parent node
                match self.data.item.parent_item {
                    parent_item if !parent_item.is_null() => {
                        let mut parent_item = Box::from_raw(parent_item);
                        // prevent the parent from dropping the current node
                        parent_item.data.queue = ptr::null_mut();
                        mem::drop(parent_item);
                    }
                    _ => (),
                }
            }
            QUEUE_REMOVE(&mut self.node);
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
union mq_item_data {
    queue: *mut MultiQueue,
    item: mq_item_data_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct mq_item_data_item {
    event: Event,
    parent_item: *mut MultiQueueItem,
}

#[repr(C)]
pub struct MultiQueue {
    parent: *mut MultiQueue,
    headtail: QUEUE, // circularly-linked
    put_cb: put_callback,
    data: *mut libc::c_void,
    size: libc::size_t,
}
impl Drop for MultiQueue {
    fn drop(&mut self) {
        unsafe {
            while !QUEUE_EMPTY(&mut self.headtail) {
                let q: &mut QUEUE = self.headtail.next.as_mut().unwrap();
                mem::drop(Box::from_raw(multiqueue_node_data(q)));
            }
        }
    }
}

/// Event present on multiple queues.
#[repr(C)]
struct MulticastEvent {
    event: Event,
    fired: bool,
    refcount: libc::c_int,
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
    let mut rv: Box<MultiQueue> = Box::new(MultiQueue {
        parent: parent,
        headtail: QUEUE {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        },
        put_cb: put_cb,
        data: data,
        size: 0,
    });
    QUEUE_INIT(&mut rv.headtail);
    return Box::into_raw(rv);
}

#[no_mangle]
pub unsafe extern "C" fn multiqueue_free(this: *mut MultiQueue) {
    c_assert!(!this.is_null());
    mem::drop(Box::from_raw(this));
}

/// Removes the next item and returns its Event.
#[no_mangle]
pub unsafe extern "C" fn multiqueue_get(this: &mut MultiQueue) -> Event {
    return if multiqueue_empty(this) {
        NILEVENT
    } else {
        multiqueue_remove(this)
    };
}

pub unsafe fn multiqueue_put(this: &mut MultiQueue, cb: argv_callback, args: &[*mut libc::c_void]) {
    multiqueue_put_event(this, event_create(cb, args));
}
#[no_mangle]
pub unsafe extern "C" fn multiqueue_put_event(this: &mut MultiQueue, event: Event) {
    multiqueue_push(this, event);
    if !(*this).parent.is_null() {
        if let Some(put_cb) = (*(*this).parent).put_cb {
            put_cb((*this).parent.as_mut(), (*(*this).parent).data);
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn multiqueue_process_events(this: &mut MultiQueue) {
    while !multiqueue_empty(this) {
        let mut event: Event = multiqueue_remove(this);
        if let Some(handler) = event.handler {
            handler(event.argv.as_mut_ptr());
        }
    }
}

/// Removes all events without processing them.
#[no_mangle]
pub unsafe extern "C" fn multiqueue_purge_events(this: &mut MultiQueue) {
    while !multiqueue_empty(this) {
        multiqueue_remove(this);
    }
}

#[no_mangle]
pub unsafe extern "C" fn multiqueue_empty(this: &MultiQueue) -> bool {
    return QUEUE_EMPTY(&(*this).headtail);
}

#[no_mangle]
pub unsafe extern "C" fn multiqueue_replace_parent(
    this: &mut MultiQueue,
    new_parent: *mut MultiQueue,
) {
    c_assert!(multiqueue_empty(this));
    (*this).parent = new_parent;
}

/// Gets the count of all events currently in the queue.
#[no_mangle]
pub unsafe extern "C" fn multiqueue_size(this: &MultiQueue) -> libc::size_t {
    return this.size;
}

/// Gets an Event from an item.
unsafe fn multiqueueitem_get_event(item: &mut MultiQueueItem) -> Event {
    let ev: Event;
    if item.link {
        // get the next node in the linked queue
        let linked: *mut MultiQueue = item.data.queue;
        c_assert!(!multiqueue_empty(linked.as_ref().unwrap()));
        let child: *mut MultiQueueItem =
            multiqueue_node_data(QUEUE_HEAD((*linked).headtail).as_mut().unwrap());
        ev = (*child).data.item.event;
    } else {
        ev = item.data.item.event;
    }
    return ev;
}

unsafe fn multiqueue_remove(this: &mut MultiQueue) -> Event {
    c_assert!(!multiqueue_empty(this));
    let h: &mut QUEUE = QUEUE_HEAD(this.headtail).as_mut().unwrap();
    let mut item: Box<MultiQueueItem> = Box::from_raw(multiqueue_node_data(h));
    c_assert!(!item.link || this.parent.is_null()); // Only a parent queue has link-nodes
    let ev: Event = multiqueueitem_get_event(&mut item);
    this.size = this.size.wrapping_sub(1);
    mem::drop(item);
    return ev;
}

unsafe fn multiqueue_push(this: &mut MultiQueue, event: Event) {
    let mut item: Box<MultiQueueItem> = Box::new(MultiQueueItem {
        link: false,
        node: Default::default(),
        data: mq_item_data {
            item: mq_item_data_item {
                event: event,
                parent_item: ptr::null_mut(),
            },
        },
    });
    QUEUE_INSERT_TAIL(&mut this.headtail, &mut item.node);
    if !this.parent.is_null() {
        // push link node to the parent queue
        let mut parent_item = Box::new(MultiQueueItem {
            link: true,
            node: Default::default(),
            data: mq_item_data { queue: this },
        });
        QUEUE_INSERT_TAIL(&mut (*this.parent).headtail, &mut parent_item.node);
        item.data.item.parent_item = Box::into_raw(parent_item);
    }
    this.size = this.size.wrapping_add(1);
    Box::into_raw(item);
}

unsafe fn multiqueue_node_data(q: &mut QUEUE) -> *mut MultiQueueItem {
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
    let data = Box::new(MulticastEvent {
        event: ev,
        fired: false,
        refcount: num,
    });
    return event_create(Some(multiqueue_oneshot_event), vargs!(Box::into_raw(data)));
}
unsafe extern "C" fn multiqueue_oneshot_event(argv: *mut *mut libc::c_void) {
    let mut data: Box<MulticastEvent> = Box::from_raw(*argv.offset(0) as *mut _);
    if !data.fired {
        data.fired = true;
        if let Some(handler) = data.event.handler {
            handler(data.event.argv.as_mut_ptr());
        }
    }
    data.refcount -= 1;
    if data.refcount == 0 {
        mem::drop(data);
    } else {
        // keep 'data' allocated for the next call of this function
        Box::into_raw(data);
    }
}
