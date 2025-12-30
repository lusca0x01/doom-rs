use std::ptr::addr_of_mut;
use std::time::{Duration, Instant};

static mut START: Option<Instant> = None;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_dg_get_ticks() -> u32 {
    unsafe {
        let start_ptr = addr_of_mut!(START);
        if (*start_ptr).is_none() {
            *start_ptr = Some(Instant::now());
        }
        (*start_ptr).unwrap().elapsed().as_millis() as u32
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_dg_sleep(ms: u32) {
    std::thread::sleep(Duration::from_millis(ms as u64));
}
