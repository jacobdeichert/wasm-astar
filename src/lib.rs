#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::os::raw::{c_double, c_int};
use std::collections::HashMap;

mod engine;
mod browser;
mod utils;
mod world;
use world::{Tile, WorldState};
use engine::EngineState;

lazy_static! {
    static ref WORLD_STATE: Mutex<WorldState> = Mutex::new(WorldState::new());
    static ref ENGINE_STATE: Mutex<EngineState> = Mutex::new(EngineState::new());
    // Maps to RenderManager.renderers on the client side
    static ref RENDERER_MAP: HashMap<&'static str, i32> = {
        [("tile_bg", 0), ("main", 1), ("fps", 2)].iter().cloned().collect()
    };
}

fn get_layer(layer: &str) -> i32 {
    *RENDERER_MAP.get(layer).unwrap()
}

// Imported js functions. Note, some are used in other modules (browser, utils).
extern "C" {
    fn js_update();
    fn js_draw_fps(renderer_id: c_int, fps: c_double);
    fn js_draw_circle(
        renderer_id: c_int,
        px: c_double,
        py: c_double,
        radius: c_double,
        ch: c_int,
        cs: c_int,
        cl: c_int,
        ca: c_int,
    );
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
    utils::log("Initializing Rust/WASM");
    // Requires block curlies so lifetime of world ends which causes unlock
    // and allows draw_background() to gain control of the lock.
    // Otherwise, this generic client error occurs:
    //      "RuntimeError: unreachable executed"
    // QUESTION: is there a better way to do this?
    {
        let world = &mut WORLD_STATE.lock().unwrap();
        world.debug = if debug == 1 { true } else { false };
        utils::log_fmt(format!("Debug Mode: {}", world.debug));
        if world.debug {
            browser::start_interval_tick(render_interval_ms);
        } else {
            browser::request_next_tick();
        }
    }
    initial_draw();
}

#[no_mangle]
pub extern "C" fn tick(elapsed_time: f64) {
    browser::clear_screen(get_layer("main"));
    update(elapsed_time);
    draw(elapsed_time);
    browser::request_next_tick();
}

#[no_mangle]
pub extern "C" fn key_down(key_code: u32) {
    let engine = &mut ENGINE_STATE.lock().unwrap();
    engine.set_key_down(key_code);
}

#[no_mangle]
pub extern "C" fn key_up(key_code: u32) {
    let engine = &mut ENGINE_STATE.lock().unwrap();
    engine.set_key_up(key_code);
}

fn update(elapsed_time: f64) {
    let world = &mut WORLD_STATE.lock().unwrap();
    let engine = &mut ENGINE_STATE.lock().unwrap();
    engine.update(elapsed_time);
    world.calc_astar();
    unsafe {
        js_update();
    }
}

fn initial_draw() {
    {
        let world = &mut WORLD_STATE.lock().unwrap();
        browser::set_screen_size(world.width, world.height, world.quality);
        browser::set_layer_size(
            get_layer("tile_bg"),
            world.width,
            world.height,
            world.quality,
        );
        browser::set_layer_size(get_layer("main"), world.width, world.height, world.quality);
        browser::set_layer_size(get_layer("fps"), 200, 70, world.quality);
    }
    draw_background();
}

fn draw(elapsed_time: f64) {
    let world = &mut WORLD_STATE.lock().unwrap();
    draw_path(world, &world.tiles[world.end_id as usize]);
    draw_tile("main", &world.tiles[world.start_id as usize]);
    draw_tile("main", &world.tiles[world.end_id as usize]);
    draw_fps(elapsed_time);
}

fn draw_background() {
    let world = WORLD_STATE.lock().unwrap();
    for t in world.tiles.iter() {
        draw_tile("tile_bg", &t);
    }
}

fn draw_path(world: &WorldState, t: &Tile) {
    unsafe {
        js_draw_circle(
            get_layer(&"tile_bg"),
            t.transform.pos_x + (t.transform.scale_x / 2_f64),
            t.transform.pos_y + (t.transform.scale_y / 2_f64),
            (t.transform.scale_x / 5_f64),
            280,
            100,
            73,
            1,
        );
    }
    if t.parent_id >= 0 {
        draw_path(world, &world.tiles[t.parent_id as usize]);
    }
}

fn draw_tile(renderer: &str, t: &Tile) {
    unsafe {
        js_draw_tile(
            get_layer(renderer),
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


fn draw_fps(elapsed_time: f64) {
    let engine = &mut ENGINE_STATE.lock().unwrap();
    let fps = engine.fps;
    engine.render_fps(elapsed_time, 150, || {
        browser::clear_screen(get_layer("fps"));
        unsafe {
            js_draw_fps(get_layer("fps"), fps);
        }
    });
}
