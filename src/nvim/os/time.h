#ifndef NVIM_OS_TIME_H
#define NVIM_OS_TIME_H

#include <stdint.h>
#include <stdbool.h>
#include <time.h>

typedef uint64_t Timestamp;

void time_init(void);
uint64_t os_hrtime(void);
uint64_t os_now(void);
void os_delay(uint64_t ms, bool ignoreinput);
void os_microdelay(uint64_t us, bool ignoreinput);
struct tm *os_localtime_r(const time_t *restrict clock,
                          struct tm *restrict result);
struct tm *os_localtime(struct tm *result);
Timestamp os_time(void);
#endif  // NVIM_OS_TIME_H
