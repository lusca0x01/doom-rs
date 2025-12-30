#pragma once
#include <stdint.h>

void rust_dg_init(void);
void rust_dg_draw_frame(uint32_t* buffer);
uint32_t rust_dg_get_ticks(void);
void rust_dg_sleep(uint32_t ms);
int rust_dg_get_key(int* pressed, unsigned char* key);
void rust_dg_exit(int code);
