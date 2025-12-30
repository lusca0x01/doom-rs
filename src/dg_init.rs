use windows::Win32::{
    Graphics::Gdi::*, System::LibraryLoader::GetModuleHandleW, UI::WindowsAndMessaging::*,
};
use windows::core::w;

use crate::globals::MainHwnd;
use crate::globals::{BITMAP_INFO, HEIGHT, HWND_MAIN, SCALE, WIDTH};
use crate::window::wndproc;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_dg_init() {
    let hinstance = unsafe {
        match GetModuleHandleW(None) {
            Ok(h) => h,
            Err(_) => {
                eprintln!("[doom-rs] Error: GetModuleHandleW failed");
                return;
            }
        }
    };

    let class_name = w!("RUST_DOOM_GENERIC");

    let wc = WNDCLASSW {
        lpfnWndProc: Some(wndproc),
        hInstance: hinstance.into(),
        lpszClassName: class_name,
        ..Default::default()
    };

    unsafe {
        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            Default::default(),
            class_name,
            w!("Rustecean Doom"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            WIDTH * SCALE,
            HEIGHT * SCALE,
            None,
            None,
            Some(hinstance.into()),
            None,
        )
        .expect("[doom-rs] Error: Failed to create window");

        HWND_MAIN.set(MainHwnd(hwnd)).ok();
    }

    let mut bmi = BITMAPINFO::default();
    bmi.bmiHeader = BITMAPINFOHEADER {
        biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: WIDTH,
        biHeight: -HEIGHT,
        biPlanes: 1,
        biBitCount: 32,
        biCompression: BI_RGB.0,
        ..Default::default()
    };
    BITMAP_INFO.set(bmi).ok();
}
