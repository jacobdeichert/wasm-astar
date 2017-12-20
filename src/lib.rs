#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::os::raw::{c_double, c_int};
use std::collections::HashMap;

mod world;
mod engine;
use world::{Tile, WorldState};

lazy_static! {
    static ref WORLD_STATE: Mutex<WorldState> = Mutex::new(WorldState::new());
    // Maps to RenderManager.renderers on the client side
    static ref RENDERER_MAP: HashMap<&'static str, i32> = {
        [("main", 0)].iter().cloned().collect()
    };
}

// imported js functions
extern "C" {
    fn js_clear_screen(renderer_id: c_int);
    fn js_update();
    fn js_request_tick();
    fn js_start_interval_tick(ms: c_int);
    fn js_draw_tile(
        renderer_id: c_int,
        px: c_double,
        py: c_double,
        size: c_double,
        ch: c_int,
        cs: c_int,
        cl: c_int,
        ca: c_int,
    );
}

#[no_mangle]
pub extern "C" fn init(debug: i32, render_interval_ms: i32) {
    let world = &mut WORLD_STATE.lock().unwrap();
    world.debug = if debug == 1 { true } else { false };
    unsafe {
        if world.debug {
            js_start_interval_tick(render_interval_ms);
            return;
        };
        js_request_tick();
    }
}

#[no_mangle]
pub extern "C" fn tick() {
    clear_screens();
    update();
    draw();
    unsafe {
        js_request_tick();
    }
}

pub fn clear_screens() {
    unsafe {
        js_clear_screen(*RENDERER_MAP.get(&"main").unwrap());
    }
}

pub fn update() {
    unsafe {
        js_update();
    }
}

pub fn draw() {
    let world = &mut WORLD_STATE.lock().unwrap();
    for t in world.tiles.iter() {
        draw_tile(&t);
    }
}


fn draw_tile(t: &Tile) {
    unsafe {
        js_draw_tile(
            *RENDERER_MAP.get(&"main").unwrap(),
            t.transform.pos_x,
            t.transform.pos_y,
            t.transform.scale_x,
            t.color.h as i32,
            t.color.s as i32,
            t.color.l as i32,
            t.color.a as i32,
        );
    }
}
