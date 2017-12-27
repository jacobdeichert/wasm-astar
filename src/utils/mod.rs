use std::os::raw::{c_float, c_int};

extern "C" {
    fn js_random() -> c_float;
    fn js_random_range(min: c_int, max: c_int) -> c_int;
    fn js_log(ptr: *const u8, len: u32);
}

// TODO: apparently the rand crate now works with wasm.
// Switch to that!

pub fn random_range(min: i32, max: i32) -> i32 {
    unsafe { js_random_range(min, max) }
}

pub fn random() -> f32 {
    unsafe { js_random() }
}

pub fn log(msg: &str) {
    unsafe {
        js_log(msg.as_ptr(), msg.len() as u32);
    }
}

pub fn log_fmt(msg: String) {
    unsafe {
        js_log(msg.as_ptr(), msg.len() as u32);
    }
}
