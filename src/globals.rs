use windows::Win32::{Foundation::*, Graphics::Gdi::*};
use std::sync::OnceLock;

pub const WIDTH: i32 = 640;
pub const HEIGHT: i32 = 400;
pub const SCALE: i32 = 2;

#[derive(Copy, Clone)]
pub struct MainHwnd(pub HWND);

unsafe impl Sync for MainHwnd {}
unsafe impl Send for MainHwnd {}

pub static HWND_MAIN: OnceLock<MainHwnd> = OnceLock::new();
pub static BITMAP_INFO: OnceLock<BITMAPINFO> = OnceLock::new();

pub const KEY_RIGHTARROW: u8 = 0xae;
pub const KEY_LEFTARROW: u8 = 0xac;
pub const KEY_UPARROW: u8 = 0xad;
pub const KEY_DOWNARROW: u8 = 0xaf;
pub const KEY_STRAFE_L: u8 = 0xa0;
pub const KEY_STRAFE_R: u8 = 0xa1;
pub const KEY_USE: u8 = 0xa2;
pub const KEY_FIRE: u8 = 0xa3;
pub const KEY_ESCAPE: u8 = 27;
pub const KEY_TAB: u8 = 9;
pub const KEY_PAUSE: u8 = 0xff;
pub const KEY_RSHIFT: u8 = 0xb6;
pub const KEY_RALT: u8 = 0xb8;