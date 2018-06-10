use std::collections::HashSet;

use engine::{Color, Transform};
use utils::{log_fmt, random, random_range};

mod tile;
pub use self::tile::Tile;

pub struct WorldState {
    pub debug: bool,
    pub window_width: u32,
    pub window_height: u32,
    pub width: u32,
    pub height: u32,
    pub quality: u32,
    pub tile_size: u32,
    pub start_id: i32,
    pub end_id: i32,
    pub player: Transform,
    pub tiles: Vec<Tile>,
    pub recent_regen: bool,
}

impl WorldState {
    pub fn new() -> WorldState {
        let quality = 2; // Make the canvas quality better
        let width: u32 = 900 * quality;
        let height: u32 = 600 * quality;
        let tile_size: u32 = 50;

        let mut w = WorldState {
            debug: false,
            window_width: 0,
            window_height: 0,
            width,
            height,
            quality,
            tile_size,
            tiles: Vec::new(),
            player: Transform::default(),
            start_id: -1,
            end_id: -1,
            recent_regen: false,
        };
        w.reset();
        w
    }

    pub fn reset(&mut self) {
        self.load_random_map();
        // self.load_test_map();
    }

    pub fn update_player(&mut self, x_dir: i32, y_dir: i32) {
        let new_x = self.player.pos_x + (7_f64 * x_dir as f64);
        let new_y = self.player.pos_y + (7_f64 * y_dir as f64);
        if new_x + (self.tile_size as f64) < self.width as f64 && new_x > 0_f64 {
            self.player.pos_x = new_x;
        }
        if new_y + (self.tile_size as f64) < self.height as f64 && new_y > 0_f64 {
            self.player.pos_y = new_y;
        }
    }

    pub fn set_start_node(&mut self) {
        let half_tile = (self.tile_size / 2) as f64;
        self.start_id = self
            .get_tile_id_closest_to(self.player.pos_x - half_tile, self.player.pos_y - half_tile)
            as i32;
    }

    pub fn calc_astar(&mut self) {
        let mut open_nodes: Vec<usize> = Vec::new();
        let mut closed_nodes = HashSet::new();

        open_nodes.push(self.start_id as usize);
        let mut current_node;
        let end = self.tiles[self.end_id as usize].clone();

        for t in self.tiles.iter_mut() {
            t.reset(&end);
        }

        // Stop searching when either:
        // 1) target is closed, in which case the path has been found
        // 2) failed to find the target and the open list is empty (no path)
        while !closed_nodes.contains(&(self.end_id as usize)) && open_nodes.len() > 0 {
            // Find lowest F score
            open_nodes.sort_by(|a, b| {
                let a_f = &self.tiles[*a].f;
                let b_f = &self.tiles[*b].f;
                a_f.cmp(b_f)
            });

            current_node = open_nodes.swap_remove(0);
            closed_nodes.insert(current_node);

            let side_ids = vec![
                self.tiles[current_node].top,
                self.tiles[current_node].bottom,
                self.tiles[current_node].right,
                self.tiles[current_node].left,
            ];

            // Check each side node.
            // If the side exists (id >= 0)
            // If it's a wall, it's not set as a side so we don't need to worry about it.
            for s in side_ids.iter() {
                let id = *s as usize;
                if *s >= 0 && !closed_nodes.contains(&id) {
                    self.check_node(&mut open_nodes, current_node, id);
                }
            }
        }
    }

    pub fn set_player_pos(&mut self, x: f64, y: f64) {
        let half_tile = (self.tile_size / 2) as f64;
        let new_x = (x * self.quality as f64) - half_tile;
        let new_y = (y * self.quality as f64) - half_tile;
        if new_x + (self.tile_size as f64) < self.width as f64 && new_x > 0_f64 {
            self.player.pos_x = new_x;
        }
        if new_y + (self.tile_size as f64) < self.height as f64 && new_y > 0_f64 {
            self.player.pos_y = new_y;
        }
    }

    fn check_node(
        &mut self,
        open_nodes: &mut Vec<usize>,
        curr_node_id: usize,
        side_node_id: usize,
    ) {
        let id = side_node_id as usize;
        let mut parent_id = 0;
        let mut parent_g = -1;
        // if it's not already on the open list
        if !open_nodes.contains(&id) {
            open_nodes.push(id);
            parent_id = curr_node_id;
            parent_g = self.tiles[parent_id].g;
        }
        // if it's already on the open list and the path is better (lower G value)
        else if self.tiles[id].g > self.tiles[curr_node_id].g + tile::MOVE_COST {
            parent_id = curr_node_id;
            parent_g = self.tiles[parent_id].g;
        }
        if parent_g != -1 {
            self.tiles[id].parent_id = parent_id as i32;
            self.tiles[id].calc_f_g(parent_g);
        }
    }

    fn get_tile_at(&mut self, x: u32, y: u32) -> &mut Tile {
        let index = self.get_tile_id_at(x, y);
        &mut self.tiles[index]
    }

    fn get_random_tile(&mut self) -> Tile {
        let num_x_tiles = (self.width / self.tile_size) as i32;
        let num_y_tiles = (self.height / self.tile_size) as i32;
        let index = self.get_tile_id_at(
            random_range(0, num_x_tiles - 1) as u32,
            random_range(0, num_y_tiles - 1) as u32,
        );
        self.tiles[index].clone()
    }

    fn get_tile_id_at(&self, x: u32, y: u32) -> usize {
        let num_tiles = self.width / self.tile_size;
        let index = y * num_tiles + x;
        index as usize
    }

    fn get_tile_id_closest_to(&self, x: f64, y: f64) -> usize {
        let size = self.tile_size as f64;
        let x_id = (x / size).ceil() as u32;
        let y_id = (y / size).ceil() as u32;
        self.get_tile_id_at(x_id, y_id)
    }

    fn get_random_tile_id(&self) -> usize {
        let num_x_tiles = (self.width / self.tile_size) as i32;
        let num_y_tiles = (self.height / self.tile_size) as i32;
        self.get_tile_id_at(
            random_range(0, num_x_tiles - 1) as u32,
            random_range(0, num_y_tiles - 1) as u32,
        )
    }

    fn set_target_tiles(&mut self) {
        self.start_id = self.get_random_tile_id() as i32;
        self.end_id = self.get_random_tile_id() as i32;
        self.player.pos_x = self.tiles[self.start_id as usize].transform.pos_x;
        self.player.pos_y = self.tiles[self.start_id as usize].transform.pos_y;
    }

    fn set_all_tile_sides(&mut self) {
        let num_x_tiles = (self.width / self.tile_size) as i32;
        let num_y_tiles = (self.height / self.tile_size) as i32;
        for t_id in 0..self.tiles.len() {
            let x_id = self.tiles[t_id].x_id;
            let y_id = self.tiles[t_id].y_id;
            if x_id + 1 < num_x_tiles {
                let right = y_id * num_x_tiles + x_id + 1;
                if !self.tiles[right as usize].is_wall {
                    self.tiles[t_id].right = right;
                }
            }
            if x_id - 1 >= 0 {
                let left = y_id * num_x_tiles + x_id - 1;
                if !self.tiles[left as usize].is_wall {
                    self.tiles[t_id].left = left;
                }
            }

            if y_id - 1 >= 0 {
                let top = ((y_id - 1) * num_x_tiles) + x_id;
                if !self.tiles[top as usize].is_wall {
                    self.tiles[t_id].top = top;
                }
            }
            if y_id + 1 < num_y_tiles {
                let bottom = ((y_id + 1) * num_x_tiles) + x_id;
                if !self.tiles[bottom as usize].is_wall {
                    self.tiles[t_id].bottom = bottom;
                }
            }
        }
    }

    fn print_map(&self) {
        let num_y_tiles = self.height / self.tile_size;
        let num_x_tiles = self.width / self.tile_size;
        let mut map = String::from("");

        for y in 0..num_y_tiles {
            for x in 0..num_x_tiles {
                let id = self.get_tile_id_at(x, y);
                if self.tiles[id].is_wall {
                    map = format!("{}{},", map, "1");
                } else {
                    map = format!("{}{},", map, "0");
                }
            }
            map = format!("{}\n", map);
        }
        log_fmt(map);
    }

    fn load_random_map(&mut self) {
        let tile_sizes = vec![10, 20, 50];
        self.tile_size = tile_sizes[random_range(0, (tile_sizes.len() - 1) as i32) as usize];
        self.tiles = generate_tiles(self.width, self.height, self.tile_size);
        self.set_all_tile_sides();
        self.set_target_tiles();
        self.set_start_node();
        self.calc_astar();

        // Force a new map if no path found.
        if self.tiles[self.end_id as usize].parent_id == -1 {
            self.reset();
        }
    }

    fn load_test_map(&mut self) {
        let test_map = "0,0,1,1,1,0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,1,0,1,0,0,1,0,0,0,1,1,0,1,0,0,0,
            0,0,1,1,0,0,0,0,0,0,0,0,0,1,0,0,0,1,1,0,0,0,0,0,0,1,0,0,0,1,0,1,0,1,1,1,
            0,1,0,0,1,0,1,0,1,1,0,0,0,0,1,0,0,0,0,1,0,1,0,1,1,0,0,0,1,0,0,1,0,0,1,1,
            0,0,0,0,1,0,1,1,0,0,1,0,1,0,0,0,0,0,1,0,0,0,0,1,1,0,0,0,0,0,0,1,0,0,0,1,
            1,0,1,0,1,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,0,0,1,0,0,0,
            0,0,0,0,1,1,0,0,0,1,0,0,1,1,1,0,1,0,0,0,0,0,1,0,1,0,0,0,0,1,0,0,1,0,0,0,
            1,0,0,0,1,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,1,0,0,0,1,0,0,0,0,
            0,0,1,1,1,1,0,1,0,0,1,0,0,0,1,0,0,1,1,0,0,0,0,1,0,0,0,0,1,1,1,0,1,1,1,0,
            0,0,0,0,0,1,0,0,0,0,1,1,1,1,1,0,0,1,0,0,0,0,0,1,1,0,0,0,0,0,1,0,1,1,0,1,
            0,1,0,1,0,0,0,0,0,0,0,0,1,0,1,0,1,0,0,0,0,0,0,1,0,1,0,0,0,1,0,0,0,0,0,0,
            0,1,0,1,0,1,1,0,1,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,1,0,0,0,1,1,0,0,0,1,1,0,
            1,0,1,0,1,0,0,0,1,0,1,0,1,1,0,0,1,1,0,1,0,1,0,0,0,1,0,1,0,0,0,0,0,1,0,0,
            0,0,0,0,1,0,0,0,1,0,0,1,0,0,1,1,0,0,0,1,1,1,0,0,0,1,0,0,0,0,1,0,1,0,1,0,
            0,0,0,0,0,0,1,1,0,0,1,1,0,0,1,0,0,0,1,0,1,1,0,0,0,0,1,1,1,0,0,1,0,0,0,0,
            1,0,0,1,0,1,0,0,1,0,0,0,0,0,0,1,1,1,1,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,
            0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,1,1,1,0,0,0,0,0,1,0,0,1,0,1,0,0,0,1,0,
            0,1,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,
            0,0,0,1,0,0,0,0,0,0,0,1,0,0,1,0,1,1,0,0,0,0,0,0,1,0,0,0,0,0,1,0,0,1,0,0,
            0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,0,0,1,0,0,0,0,0,0,0,1,0,0,0,1,0,0,1,1,0,
            1,0,0,0,0,1,0,0,1,1,0,1,0,0,1,0,1,0,0,0,0,0,0,1,1,1,0,1,0,0,0,0,1,1,0,1,
            0,1,1,1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,1,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,
            1,0,0,0,1,0,1,0,0,1,0,0,1,1,1,0,1,0,0,0,1,1,1,0,0,0,1,1,0,0,0,1,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,0,1,0,0,1,1,0,1,0,0,0,1,0,0,1,0,0,0,0,0,0,0,0,1,1,0,
            0,0,0,0,0,0,1,1,0,0,0,1,1,1,0,0,0,0,0,0,0,0,1,1,1,0,1,0,1,0,1,0,1,0,0,0,";

        self.tile_size = 50;
        self.tiles = load_map(self.tile_size, test_map);
        self.set_all_tile_sides();
        self.start_id = 418;
        self.end_id = 316;
        self.player.pos_x = self.tiles[self.start_id as usize].transform.pos_x;
        self.player.pos_y = self.tiles[self.start_id as usize].transform.pos_y;
    }
}

fn load_map(tile_size: u32, map: &str) -> Vec<Tile> {
    let mut vec = Vec::new();
    let rows: Vec<&str> = map.split_terminator("\n").collect();

    for (y, row) in rows.iter().enumerate() {
        let cols: Vec<&str> = row.trim().split_terminator(",").collect();
        let num_cols = cols.len();
        for (x, col) in cols.iter().enumerate() {
            let px = x as f64 * tile_size as f64;
            let py = y as f64 * tile_size as f64;
            let size = tile_size as f64;
            let mut t: Tile = Tile::new(px, py, size);
            t.x_id = x as i32;
            t.y_id = y as i32;
            t.node_id = (y * num_cols + x) as usize;
            t.is_wall = {
                if String::from(*col).eq("1") {
                    true
                } else {
                    false
                }
            };
            let lightness = if t.is_wall { 20 } else { 30 };
            t.color = Color::new(0, 0, lightness, 1_f32);
            vec.push(t);
        }
    }
    vec
}

fn generate_tiles(grid_width: u32, grid_height: u32, tile_size: u32) -> Vec<Tile> {
    let mut vec = Vec::new();
    let num_y_tiles = grid_height / tile_size;
    let num_x_tiles = grid_width / tile_size;

    for y in 0..num_y_tiles {
        for x in 0..num_x_tiles {
            let px = x as f64 * tile_size as f64;
            let py = y as f64 * tile_size as f64;
            let size = tile_size as f64;
            let mut t: Tile = Tile::new(px, py, size);
            t.x_id = x as i32;
            t.y_id = y as i32;
            t.node_id = (y * num_x_tiles + x) as usize;
            t.is_wall = {
                if random() >= 0.7 {
                    true
                } else {
                    false
                }
            };
            let lightness = if t.is_wall { 20 } else { 30 };
            t.color = Color::new(0, 0, lightness, 1_f32);
            vec.push(t);
        }
    }
    vec
}
