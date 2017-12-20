use std::os::raw::c_double;
use std::os::raw::c_int;

// imported js functions
extern "C" {
    fn js_clear_screen();
    fn js_update();
    fn js_draw_tile(_: c_double, _: c_double, _: c_double, _: c_int, _: c_int, _: c_int, _: c_int);
}

#[no_mangle]
pub extern "C" fn tick() {
    unsafe {
        js_clear_screen();
    }
    update();
    draw();
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

struct Transform {
    pos_x: f64,
    pos_y: f64,
    scale_x: f64,
    scale_y: f64,
}

struct Color {
    h: u16,
    s: u16,
    l: u16,
    a: u16,
}

impl Color {
    fn new(h: u16, s: u16, l: u16, a: u16) -> Color {
        Color { h, s, l, a }
    }

    fn default() -> Color {
        Color { h: 0, s: 100, l: 80, a: 1 }
    }
}

struct Tile {
    transform: Transform,
    color: Color,
}

impl Tile {
    fn new(x: f64, y: f64, size: f64) -> Tile {
        Tile {
            transform: Transform {
                pos_x: x,
                pos_y: y,
                scale_x: size,
                scale_y: size,
            },
            color: Color::default(),
        }
    }
}

fn get_tile_at(tiles: &Vec<Tile>, grid_width: u32, tile_size: u32, x: u32, y: u32) -> &Tile {
    let num_tiles = grid_width / tile_size;
    let index = x * num_tiles + y;
    &tiles[index as usize]
}

fn generate_tiles(grid_width: u32, grid_height: u32, tile_size: u32) -> Vec<Tile> {
    let mut vec = Vec::new();

    for y in 0..(grid_height / tile_size) {
        for x in 0..(grid_width / tile_size) {
            let px = x as f64 * tile_size as f64;
            let py = y as f64 * tile_size as f64;
            let size = tile_size as f64;
            let mut t: Tile = Tile::new(px, py, size);
            // Every other tile is true and rows are offset by one. This creates a checkerboard
            let checkerboard_tile_test = (x + y) % 2 == 0;
            let hue = if checkerboard_tile_test { 0 } else { 120 };
            t.color = Color::new(hue, 100, 20, 1);
            vec.push(t);
        }
    }
    vec
}

