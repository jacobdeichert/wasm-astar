use engine::{Color, Transform};

#[derive(Clone)]
pub struct Tile {
    pub transform: Transform,
    pub color: Color,
    pub x_id: i32,
    pub y_id: i32,
    pub node_id: usize,
    pub parent_id: i32,
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
    pub is_closed: bool,
    // A* values
    pub h: i32,
    pub g: i32,
    pub f: i32,
}

impl Tile {
    pub fn new(x: f64, y: f64, size: f64) -> Tile {
        Tile {
            transform: Transform::new(x, y, size, size),
            color: Color::default(),
            x_id: 0,
            y_id: 0,
            node_id: 0,
            parent_id: -1,
            top: -1,
            bottom: -1,
            left: -1,
            right: -1,
            is_closed: false,
            h: 0,
            g: 0,
            f: 0,
        }
    }

    pub fn default() -> Tile {
        Tile::new(0_f64, 0_f64, 1_f64)
    }

    pub fn calc_h(&mut self, end_node: &Tile) {
        // H: difference between this position and the end target
        // only needs to be calculated once
        let x_diff = (self.transform.pos_x - end_node.transform.pos_x).abs() as i32;
        let y_diff = (self.transform.pos_y - end_node.transform.pos_y).abs() as i32;
        self.h = (x_diff + y_diff) * 10;
    }

    pub fn calc_f_g(&mut self, parent_g: i32) {
        self.g = parent_g + 10;
        self.f = self.g + self.h;
    }
}
