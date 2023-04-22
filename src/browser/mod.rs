use std::os::raw::c_int;

extern "C" {
    fn js_create_layer(id_ptr: *const u8, id_len: u32, key: c_int);
    fn js_clear_screen(layer_id: c_int);
    fn js_set_screen_size(width: c_int, height: c_int, quality: c_int);
    fn js_set_layer_size(layer_id: c_int, width: c_int, height: c_int, quality: c_int);
    fn js_request_tick();
    fn js_start_interval_tick(ms: c_int);
}

pub unsafe fn create_layer(id: &str, key: i32) {
    unsafe {
        js_create_layer(id.as_ptr(), id.len() as u32, key);
    }
}

pub unsafe fn clear_screen(layer: i32) {
    unsafe {
        js_clear_screen(layer);
    }
}

pub unsafe fn set_layer_size(layer: i32, width: u32, height: u32, quality: u32) {
    unsafe {
        js_set_layer_size(layer, width as i32, height as i32, quality as i32);
    }
}

pub unsafe fn set_screen_size(width: u32, height: u32, quality: u32) {
    unsafe {
        js_set_screen_size(width as i32, height as i32, quality as i32);
    }
}

pub unsafe fn request_next_tick() {
    unsafe {
        js_request_tick();
    }
}

pub unsafe fn start_interval_tick(ms: i32) {
    unsafe {
        js_start_interval_tick(ms);
    }
}
