// imported js functions
extern "C" {
    fn js_clear_screen();
    fn js_update();
    fn js_draw();
// fn draw_player(_: c_double, _: c_double, _: c_double);
}

#[no_mangle]
pub extern "C" fn tick() {
    unsafe {
        js_clear_screen();
        js_update();
        js_draw();
    }
}
