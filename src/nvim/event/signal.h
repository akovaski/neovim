#ifndef NVIM_EVENT_SIGNAL_H
#define NVIM_EVENT_SIGNAL_H

#include <uv.h>

#include "nvim/event/loop.h"

typedef struct signal_watcher SignalWatcher;
typedef void (*signal_cb)(SignalWatcher *watcher, int signum, void *data);
typedef void (*signal_close_cb)(SignalWatcher *watcher, void *data);

struct signal_watcher {
  uv_signal_t uv;
  void *data;
  signal_cb cb;
  signal_close_cb close_cb;
  MultiQueue *events;
};

void signal_watcher_init(Loop *loop, SignalWatcher *watcher, void *data);
void signal_watcher_start(SignalWatcher *watcher, signal_cb cb, int signum);
void signal_watcher_stop(SignalWatcher *watcher);
void signal_watcher_close(SignalWatcher *watcher, signal_close_cb cb);

#endif  // NVIM_EVENT_SIGNAL_H
