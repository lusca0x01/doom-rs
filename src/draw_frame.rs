use windows::Win32::Graphics::Gdi::*;

use crate::globals::{BITMAP_INFO, HEIGHT, HWND_MAIN, SCALE, WIDTH};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_dg_draw_frame(buffer: *mut u32) {
    if let (Some(hwnd), Some(bitmap_info)) = (HWND_MAIN.get(), BITMAP_INFO.get()) {
        unsafe {
            let hdc = GetDC(Some(hwnd.0));
            StretchDIBits(
                hdc,
                0,
                0,
                WIDTH * SCALE,
                HEIGHT * SCALE,
                0,
                0,
                WIDTH,
                HEIGHT,
                Some(buffer as *const _),
                bitmap_info,
                DIB_RGB_COLORS,
                SRCCOPY,
            );
            ReleaseDC(Some(hwnd.0), hdc);
        }
    }
}
