use std::os::raw::c_int;

extern "C" {
    fn js_random_range(min: c_int, max: c_int) -> c_int;
}

pub fn random_range(min: i32, max: i32) {
    unsafe { js_random_range(min, max) }
}
