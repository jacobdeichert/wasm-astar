use engine::{Color, Transform};

mod tile;
pub use self::tile::Tile;

pub struct WorldState {
    pub debug: bool,
    pub width: u32,
    pub height: u32,
    pub tile_size: u32,
    pub tiles: Vec<Tile>,
}

impl WorldState {
    pub fn new() -> WorldState {
        let width: u32 = 900;
        let height: u32 = 600;
        let tile_size: u32 = 50;

        WorldState {
            debug: false,
            width,
            height,
            tile_size,
            tiles: generate_tiles(width, height, tile_size),
        }
    }

    pub fn get_tile_at(&self, x: u32, y: u32) -> Tile {
        let num_tiles = self.width / self.tile_size;
        let index = x * num_tiles + y;
        Tile {
            transform: Transform {
                pos_x: 10_f64,
                pos_y: 10_f64,
                scale_x: 10_f64,
                scale_y: 10_f64,
            },
            color: Color::default(),
        }
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
