use engine::Color;

mod tile;
pub use self::tile::Tile;

pub struct WorldState {
    pub debug: bool,
    pub width: u32,
    pub height: u32,
    pub tile_size: u32,
    pub start_target: Tile,
    pub end_target: Tile,
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
            start_target: Tile::default(),
            end_target: Tile::default(),
        };
        w.set_target_tiles();
        w
    }

    pub fn get_tile_at(&mut self, x: u32, y: u32) -> &mut Tile {
        let index = self.get_tile_id_at(x, y);
        &mut self.tiles[index]
    }

    fn get_tile_id_at(&self, x: u32, y: u32) -> usize {
        let num_tiles = self.width / self.tile_size;
        let index = x * num_tiles + y;
        index as usize
    }

    fn set_target_tiles(&mut self) {
        self.start_target = self.get_tile_at(0, 0).clone();
        self.start_target.color.h = 220;
        self.end_target = self.get_tile_at(8, 12).clone();
        self.end_target.color.h = 280;
        // Another way I could have done it.
        // self.start_target.transform = Transform::from(&self.get_tile_at(0, 0).transform);
        // self.end_target.transform = Transform::from(&self.get_tile_at(8, 12).transform);
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
