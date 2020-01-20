#ifndef NVIM_EVENT_TIME_H
#define NVIM_EVENT_TIME_H

#include <uv.h>

#include "nvim/event/loop.h"

typedef struct time_watcher TimeWatcher;
typedef void (*time_cb)(TimeWatcher *watcher, void *data);

struct time_watcher {
  uv_timer_t uv;
  void *data;
  time_cb cb, close_cb;
  MultiQueue *events;
  bool blockable;
};

void time_watcher_init(Loop *loop, TimeWatcher *watcher, void *data);
void time_watcher_start(TimeWatcher *watcher, time_cb cb, uint64_t timeout,
    uint64_t repeat);
void time_watcher_stop(TimeWatcher *watcher);
void time_watcher_close(TimeWatcher *watcher, time_cb cb);
#endif  // NVIM_EVENT_TIME_H
