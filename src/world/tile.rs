use engine::{Color, Transform};

pub const MOVE_COST: i32 = 10;

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
    pub is_wall: bool,
    // A* values
    // TODO: move to a new struct type just for A*
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
            is_wall: false,
            h: 0,
            g: 0,
            f: 0,
        }
    }

    pub fn default() -> Tile {
        Tile::new(0_f64, 0_f64, 1_f64)
    }

    pub fn reset(&mut self, end_node: &Tile) {
        self.parent_id = -1;
        self.g = 0;
        self.f = 0;
        self.calc_h(end_node);
    }

    fn calc_h(&mut self, end_node: &Tile) {
        if self.is_wall {
            return;
        }
        // H: difference between this position and the end target
        // REMINDER TO SELF: the MOVE_COST is very dependant on the x/y diff scale.
        // I was using px,py before by accident which caused diffs to be very large
        // and my MOVE_COST of 10 became useless. Using x/y ids keeps the diffs small
        // enough for MOVE_COST of 10 to work.
        let x_diff = (self.x_id - end_node.x_id).abs() as i32;
        let y_diff = (self.y_id - end_node.y_id).abs() as i32;
        self.h = (x_diff + y_diff) * MOVE_COST;
    }

    pub fn calc_f_g(&mut self, parent_g: i32) {
        self.g = parent_g + MOVE_COST;
        self.f = self.g + self.h;
    }
}
