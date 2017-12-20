use engine::{Color};

mod tile;
pub use self::tile::Tile;

pub struct WorldState {
    pub debug: bool,
    pub width: u32,
    pub height: u32,
    pub tile_size: u32,
    pub start_target_id: usize,
    pub end_target_id: usize,
    pub tiles: Vec<Tile>,
}

impl WorldState {
    pub fn new() -> WorldState {
        let width: u32 = 900;
        let height: u32 = 600;
        let tile_size: u32 = 50;
        let tiles = generate_tiles(width, height, tile_size);

        let mut w = WorldState {
            debug: false,
            width,
            height,
            tile_size,
            tiles,
            start_target_id: 0,
            end_target_id: 0,
        };
        w.set_target_tiles();
        w
    }

    pub fn get_tile_at(&mut self, x: u32, y: u32) -> &mut Tile {
        let num_tiles = self.width / self.tile_size;
        let index = x * num_tiles + y;
        &mut self.tiles[index as usize]
    }

    fn get_tile_id_at(&self, x: u32, y: u32) -> usize {
        let num_tiles = self.width / self.tile_size;
        let index = x * num_tiles + y;
        index as usize
    }

    fn get_start_target(&mut self) -> &mut Tile {
        &mut self.tiles[self.start_target_id]
    }

    fn get_end_target(&mut self) -> &mut Tile {
        &mut self.tiles[self.end_target_id]
    }

    fn set_target_tiles(&mut self) {
        self.start_target_id = self.get_tile_id_at(0, 0);
        self.end_target_id = self.get_tile_id_at(8, 12);
        // Requires block curlies so lifetimes of mutable borrows can die instead of conflict
        // QUESTION: is there a better way to do it?
        {
            let start_target = self.get_start_target();
            start_target.color.h = 220;
        }
        let end_target = self.get_end_target();
        end_target.color.h = 280;
    }
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
