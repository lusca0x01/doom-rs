#[unsafe(no_mangle)]
pub extern "C" fn rust_dg_exit(code: i32) {
    std::process::exit(code);
}
