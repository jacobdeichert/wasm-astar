#[macro_use]
extern crate lazy_static;

use std::os::raw::{c_double, c_int};

mod world;
mod engine;
use world::{WORLD_STATE, Tile, get_tile_at, generate_tiles};

// imported js functions
extern "C" {
    fn js_clear_screen(_: c_int);
    fn js_update();
    fn js_request_tick();
    fn js_start_interval_tick(_: c_int);
    fn js_draw_tile(_: c_double, _: c_double, _: c_double, _: c_int, _: c_int, _: c_int, _: c_int);
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
        js_clear_screen(0);
    }
}

pub fn update() {
    unsafe {
        js_update();
    }
}

pub fn draw() {
    let grid_width: u32 = 900;
    let grid_height: u32 = 600;
    let tile_size: u32 = 50;
    let tiles: Vec<Tile> = generate_tiles(grid_width, grid_height, tile_size);
    let start_target = get_tile_at(&tiles, grid_width, tile_size, 0, 0);
    // TODO: Figure out how to mutate things
    // start_target.color.h = 220;
    let end_target = get_tile_at(&tiles, grid_width, tile_size, 8, 12);
    unsafe {
        for t in tiles.iter() {
            js_draw_tile(
                t.transform.pos_x,
                t.transform.pos_y,
                t.transform.scale_x,
                t.color.h as i32,
                t.color.s as i32,
                t.color.l as i32,
                t.color.a as i32,
            );
        }
        // draw the start tile
        js_draw_tile(
            start_target.transform.pos_x,
            start_target.transform.pos_y,
            start_target.transform.scale_x,
            220,
            start_target.color.s as i32,
            start_target.color.l as i32,
            start_target.color.a as i32,
        );
        // draw the end tile
        js_draw_tile(
            end_target.transform.pos_x,
            end_target.transform.pos_y,
            end_target.transform.scale_x,
            280,
            end_target.color.s as i32,
            end_target.color.l as i32,
            end_target.color.a as i32,
        );
    }
}
