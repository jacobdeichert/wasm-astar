use std::sync::Mutex;

use engine::{Color};

mod tile;
pub use self::tile::Tile;

lazy_static! {
    pub static ref WORLD_STATE: Mutex<WorldState> = Mutex::new(new_world_state());
}

pub struct WorldState {
    pub debug: bool,
}

fn new_world_state() -> WorldState {
    WorldState {
        debug: false,
        // state: GameState::new(Size::new(width, height)),
        // actions: Actions::default(),
        // time_controller: TimeController::new(Pcg32Basic::from_seed([42, 42]))
    }
}

pub fn get_tile_at(tiles: &Vec<Tile>, grid_width: u32, tile_size: u32, x: u32, y: u32) -> &Tile {
    let num_tiles = grid_width / tile_size;
    let index = x * num_tiles + y;
    &tiles[index as usize]
}

pub fn generate_tiles(grid_width: u32, grid_height: u32, tile_size: u32) -> Vec<Tile> {
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
