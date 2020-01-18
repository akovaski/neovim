#ifndef NVIM_AUCMD_H
#define NVIM_AUCMD_H

#include <stdint.h>

void do_autocmd_uienter(uint64_t chanid, bool attached);
void aucmd_schedule_focusgained(bool gained);

#endif  // NVIM_AUCMD_H

