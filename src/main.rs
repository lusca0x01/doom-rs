mod globals;
mod window;
mod draw_frame;
mod dg_get_key;
mod get_ticks;
mod dg_init;
mod exit;
mod sound;

use std::ptr;

unsafe extern "C" {
    fn doomgeneric_Create(argc: i32, argv: *const *const i8);
    fn doomgeneric_Tick();
}

fn main() {
    call_doom_create();
    loop {
        call_doom_tick();
    }
}

#[inline]
fn call_doom_create() {
    unsafe { doomgeneric_Create(0, ptr::null()) }
}

#[inline]
fn call_doom_tick() {
    unsafe { doomgeneric_Tick() }
}
