#include "doomgeneric.h"
#include "dg_rust.h"

void DG_Init(void) {
    rust_dg_init();
}

void DG_DrawFrame(void) {
    rust_dg_draw_frame(DG_ScreenBuffer);
}

uint32_t DG_GetTicksMs(void) {
    return rust_dg_get_ticks();
}

void DG_SleepMs(uint32_t ms) {
    rust_dg_sleep(ms);
}

int DG_GetKey(int* pressed, unsigned char* key) {
    return rust_dg_get_key(pressed, key);
}

void DG_Exit(int code) {
    rust_dg_exit(code);
}

void DG_SetWindowTitle(const char* title) {
    (void)title;
}
