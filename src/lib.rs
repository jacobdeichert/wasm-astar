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
        [("tile_bg", 0), ("main", 1)].iter().cloned().collect()
    };
}

// All imported js functions
// NOTE: some are used in other modules. See utils.
extern "C" {
    fn js_random_range(min: c_int, max: c_int);
    fn js_clear_screen(renderer_id: c_int);
    fn js_set_canvas_size(width: c_int, height: c_int);
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
    // Requires block curlies so lifetime of world ends which causes unlock
    // and allows draw_background() to gain control of the lock.
    // Otherwise, this generic client error occurs:
    //      "RuntimeError: unreachable executed"
    // QUESTION: is there a better way to do this?
    {
        let world = &mut WORLD_STATE.lock().unwrap();
        world.debug = if debug == 1 { true } else { false };
        unsafe {
            if world.debug {
                js_start_interval_tick(render_interval_ms);
            } else {
                js_request_tick();
            }
        }
    }
    initial_draw();
}

#[no_mangle]
pub extern "C" fn tick() {
    clear_screen("main");
    update();
    draw();
    unsafe {
        js_request_tick();
    }
}

fn clear_screen(renderer: &str) {
    unsafe {
        js_clear_screen(*RENDERER_MAP.get(renderer).unwrap());
    }
}

fn update() {
    // let world = &mut WORLD_STATE.lock().unwrap();
    // for t in world.tiles.iter_mut() {
    //     t.update();
    // }
    unsafe {
        js_update();
    }
}

fn initial_draw() {
    unsafe {
        let world = &mut WORLD_STATE.lock().unwrap();
        js_set_canvas_size(world.width as i32, world.height as i32);
    }
    draw_background();
}

fn draw() {
    let world = &mut WORLD_STATE.lock().unwrap();
    draw_tile("main", &world.start_target);
    draw_tile("main", &world.end_target);
    world.calc_path();
    for t in world.path.iter() {
        draw_tile("main", &t);
    }
}

fn draw_background() {
    let world = WORLD_STATE.lock().unwrap();
    for t in world.tiles.iter() {
        draw_tile("tile_bg", &t);
    }
}

fn draw_tile(renderer: &str, t: &Tile) {
    unsafe {
        js_draw_tile(
            *RENDERER_MAP.get(renderer).unwrap(),
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
