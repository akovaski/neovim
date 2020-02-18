#ifndef NVIM_EVENT_MULTIQUEUE_H
#define NVIM_EVENT_MULTIQUEUE_H

#include <uv.h>
#include <stdbool.h>

#include "nvim/event/defs.h"
#include "nvim/lib/queue.h"

typedef struct multiqueue MultiQueue;
typedef void (*put_callback)(MultiQueue *multiq, void *data);

#define multiqueue_put(q, h, ...) \
  multiqueue_put_event(q, event_create(h, __VA_ARGS__));

MultiQueue *multiqueue_new_parent(put_callback put_cb, void *data);
MultiQueue *multiqueue_new_child(MultiQueue *parent);
void multiqueue_free(MultiQueue *this);
Event multiqueue_get(MultiQueue *this);
void multiqueue_put_event(MultiQueue *this, Event event);
void multiqueue_process_events(MultiQueue *this);
void multiqueue_purge_events(MultiQueue *this);
bool multiqueue_empty(MultiQueue *this);
void multiqueue_replace_parent(MultiQueue *this, MultiQueue *new_parent);
size_t multiqueue_size(MultiQueue *this);
Event event_create_oneshot(Event ev, int num);

#endif  // NVIM_EVENT_MULTIQUEUE_H
