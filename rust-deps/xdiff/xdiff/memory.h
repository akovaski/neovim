#ifndef NVIM_MEMORY_H
#define NVIM_MEMORY_H

void try_to_free_memory(void);
void *try_malloc(size_t size);
void *verbose_try_malloc(size_t size);
void *xmalloc(size_t size);
void xfree(void *ptr);
void *xcalloc(size_t count, size_t size);
void *xrealloc(void *ptr, size_t size);
void *xmallocz(size_t size);

#endif  // NVIM_MEMORY_H
