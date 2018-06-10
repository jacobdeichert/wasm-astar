#[macro_use]
extern crate lazy_static;

use std::os::raw::{c_double, c_float, c_int};
use std::sync::Mutex;

mod browser;
mod engine;
mod utils;
mod world;
use engine::EngineState;
use world::{Tile, WorldState};

// Imported js functions. Note, some are used in other modules (browser, utils).
extern "C" {
    fn js_update();
    fn js_draw_fps(layer_id: c_int, fps: c_double);
    fn js_path_count(layer_id: c_int, count: c_int);
    fn js_draw_circle(
        layer_id: c_int,
        px: c_double,
        py: c_double,
        radius: c_double,
        ch: c_int,
        cs: c_int,
        cl: c_int,
        ca: c_float,
    );
    fn js_draw_tile(
        layer_id: c_int,
        px: c_double,
        py: c_double,
        size: c_double,
        ch: c_int,
        cs: c_int,
        cl: c_int,
        ca: c_float,
    );
}

// Learned about this pattern from rocket_wasm on github
// https://github.com/aochagavia/rocket_wasm/blob/d0ca51beb9c7c351a1f0266206edfd553bf078d3/src/lib.rs
// QUESTION: is there a better way/place to store state???
lazy_static! {
    static ref WORLD_STATE: Mutex<WorldState> = Mutex::new(WorldState::new());
    static ref ENGINE_STATE: Mutex<EngineState> = Mutex::new(EngineState::new());
}

// Maps to WASM_ASTAR.layers on the client side
enum Layer {
    TileBg = 0,
    Main = 1,
    Fps = 2,
}

#[no_mangle]
pub extern "C" fn init(debug: i32, render_interval_ms: i32, window_width: u32, window_height: u32) {
    utils::log("Initializing Rust/WASM");
    // Requires block curlies so lifetime of world ends which causes unlock
    // and allows initial_draw() to gain control of the lock.
    // Otherwise, this generic client error occurs: "RuntimeError: unreachable executed"
    // QUESTION: is there a better way to do this?
    browser::create_layer("TileBg", Layer::TileBg as i32);
    browser::create_layer("Main", Layer::Main as i32);
    browser::create_layer("Fps", Layer::Fps as i32);
    {
        let world = &mut WORLD_STATE.lock().unwrap();
        world.window_width = window_width;
        world.window_height = window_height;
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
    browser::clear_screen(Layer::Main as i32);
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

#[no_mangle]
pub extern "C" fn mouse_move(x: i32, y: i32) {
    let engine = &mut ENGINE_STATE.lock().unwrap();
    let world = &mut WORLD_STATE.lock().unwrap();
    engine.mouse_move(x, y);
    world.set_player_pos(x as f64, y as f64);
}

fn update(elapsed_time: f64) {
    handle_input();
    let engine = &mut ENGINE_STATE.lock().unwrap();
    engine.update(elapsed_time);
    let world = &mut WORLD_STATE.lock().unwrap();
    world.set_start_node();
    world.calc_astar();
    unsafe {
        js_update();
    }
}

fn handle_input() {
    let world = &mut WORLD_STATE.lock().unwrap();
    let engine = &mut ENGINE_STATE.lock().unwrap();
    if !engine.was_key_down(engine::KeyCode::Spacebar)
        && engine.is_key_down(engine::KeyCode::Spacebar)
        && !world.recent_regen
    {
        world.reset();
        browser::clear_screen(Layer::Main as i32);
        // Horrible check until i implement event callbacks for key presses
        world.recent_regen = true;
    } else if !engine.is_key_down(engine::KeyCode::Spacebar) {
        world.recent_regen = false;
    }

    let mut x_dir = 0;
    let mut y_dir = 0;
    if engine.is_key_down(engine::KeyCode::ArrowUp) {
        y_dir = -1;
    } else if engine.is_key_down(engine::KeyCode::ArrowDown) {
        y_dir = 1;
    }
    if engine.is_key_down(engine::KeyCode::ArrowLeft) {
        x_dir = -1;
    } else if engine.is_key_down(engine::KeyCode::ArrowRight) {
        x_dir = 1;
    }
    world.update_player(x_dir, y_dir);
}

fn initial_draw() {
    let world = &mut WORLD_STATE.lock().unwrap();
    if world.window_width < 600 {
        world.width = 350 * world.quality;
        world.height = 450 * world.quality;
        world.reset();
    }
    browser::set_screen_size(world.width, world.height, world.quality);
    browser::set_layer_size(
        Layer::TileBg as i32,
        world.width,
        world.height,
        world.quality,
    );
    browser::set_layer_size(Layer::Main as i32, world.width, world.height, world.quality);
    browser::set_layer_size(Layer::Fps as i32, 200, 70, world.quality);
    draw_background(world);
}

fn draw(elapsed_time: f64) {
    let world = &mut WORLD_STATE.lock().unwrap();
    if world.recent_regen {
        draw_background(world);
    }
    draw_path(world, &world.tiles[world.end_id as usize]);
    draw_tile_with_color(
        Layer::Main,
        &world.tiles[world.start_id as usize],
        &engine::Color::new(32, 100, 60, 0.3),
    );
    draw_tile_with_color(
        Layer::Main,
        &world.tiles[world.end_id as usize],
        &engine::Color::new(112, 89, 61, 1.0),
    );
    let path_count = get_path_count(world, &world.tiles[world.end_id as usize], 0);
    draw_path_count(path_count);
    // draw_player(world);
    draw_fps(elapsed_time);
}

fn draw_background(world: &WorldState) {
    for t in world.tiles.iter() {
        draw_tile(Layer::TileBg, &t);
    }
}

fn draw_path(world: &WorldState, t: &Tile) {
    let half_tile = (world.tile_size / 2) as f64;
    unsafe {
        js_draw_circle(
            Layer::Main as i32,
            t.transform.pos_x + half_tile,
            t.transform.pos_y + half_tile,
            t.transform.scale_x / 5_f64,
            280,
            100,
            73,
            1_f32,
        );
    }
    if t.parent_id >= 0 {
        draw_path(world, &world.tiles[t.parent_id as usize]);
    }
}

fn get_path_count(world: &WorldState, t: &Tile, counter: i32) -> i32 {
    if t.parent_id >= 0 {
        get_path_count(world, &world.tiles[t.parent_id as usize], counter + 1)
    } else {
        counter
    }
}

fn draw_tile(layer: Layer, t: &Tile) {
    draw_tile_with_color(layer, &t, &t.color);
}

fn draw_tile_with_color(layer: Layer, t: &Tile, c: &engine::Color) {
    unsafe {
        js_draw_tile(
            layer as i32,
            t.transform.pos_x,
            t.transform.pos_y,
            t.transform.scale_x,
            c.h as i32,
            c.s as i32,
            c.l as i32,
            c.a,
        );
    }
}

fn draw_player(world: &WorldState) {
    let half_tile = (world.tile_size / 2) as f64;
    unsafe {
        js_draw_circle(
            Layer::Main as i32,
            world.player.pos_x + half_tile,
            world.player.pos_y + half_tile,
            (world.tile_size / 4) as f64,
            32,
            100,
            55,
            1_f32,
        );
    }
}

fn draw_path_count(path_count: i32) {
    unsafe {
        js_path_count(Layer::Main as i32, path_count);
    }
}

fn draw_fps(elapsed_time: f64) {
    let engine = &mut ENGINE_STATE.lock().unwrap();
    let fps = engine.fps;
    engine.render_fps(elapsed_time, 150, || {
        browser::clear_screen(Layer::Fps as i32);
        unsafe {
            js_draw_fps(Layer::Fps as i32, fps);
        }
    });
}
