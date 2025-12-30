use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use crate::globals::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_dg_get_key(pressed: *mut i32, key: *mut u8) -> i32 {
    let mut msg = MSG::default();

    unsafe {
        if PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() {
            match msg.message {
                WM_KEYDOWN | WM_KEYUP => {
                    *pressed = if msg.message == WM_KEYDOWN { 1 } else { 0 };

                    let vk = VIRTUAL_KEY(msg.wParam.0 as u16);
                    *key = match vk {
                        // Move
                        VK_UP => KEY_UPARROW,
                        VK_DOWN => KEY_DOWNARROW,
                        VK_LEFT => KEY_LEFTARROW,
                        VK_RIGHT => KEY_RIGHTARROW,
                        // WASD
                        VK_W => KEY_UPARROW,
                        VK_S => KEY_DOWNARROW,
                        VK_A => KEY_LEFTARROW,
                        VK_D => KEY_RIGHTARROW,
                        // Strafe
                        VK_MENU | VK_LMENU | VK_RMENU => KEY_RALT,
                        VK_OEM_COMMA => KEY_STRAFE_L,
                        VK_OEM_PERIOD => KEY_STRAFE_R,
                        // Fire
                        VK_CONTROL | VK_LCONTROL | VK_RCONTROL => KEY_FIRE,
                        // Use/Open
                        VK_SPACE => KEY_USE,
                        // Run
                        VK_SHIFT | VK_LSHIFT | VK_RSHIFT => KEY_RSHIFT,
                        // Map
                        VK_TAB => KEY_TAB,
                        // Pause
                        VK_PAUSE => KEY_PAUSE,
                        // Menu
                        VK_ESCAPE => KEY_ESCAPE,
                        // Weapon selection (1-7)
                        VK_1 => b'1',
                        VK_2 => b'2',
                        VK_3 => b'3',
                        VK_4 => b'4',
                        VK_5 => b'5',
                        VK_6 => b'6',
                        VK_7 => b'7',
                        // Yes/No/Enter
                        VK_Y => b'y',
                        VK_N => b'n',
                        VK_RETURN => b'\r',
                        _ => return 0,
                    };

                    return 1;
                }
                WM_QUIT => std::process::exit(0),
                _ => {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
    }
    0
}
