#ifndef NVIM_EVENT_RSTREAM_H
#define NVIM_EVENT_RSTREAM_H

#include <stdbool.h>
#include <stddef.h>

#include <uv.h>

#include "nvim/event/loop.h"
#include "nvim/event/stream.h"

void rstream_init_fd(Loop *loop, Stream *stream, int fd, size_t bufsize);
void rstream_init_stream(Stream *stream, uv_stream_t *uvstream, size_t bufsize);
void rstream_init(Stream *stream, size_t bufsize);
void rstream_start(Stream *stream, stream_read_cb cb, void *data);
void rstream_stop(Stream *stream);

#endif  // NVIM_EVENT_RSTREAM_H
