use engine::{Color, Transform};

#[derive(Clone)]
pub struct Tile {
    pub transform: Transform,
    pub color: Color,
    pub x_id: i32,
    pub y_id: i32,
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
}

impl Tile {
    pub fn new(x: f64, y: f64, size: f64) -> Tile {
        Tile {
            transform: Transform::new(x, y, size, size),
            color: Color::default(),
            x_id: 0,
            y_id: 0,
            top: -1,
            bottom: -1,
            left: -1,
            right: -1,
        }
    }

    pub fn default() -> Tile {
        Tile::new(0_f64, 0_f64, 1_f64)
    }

    // pub fn update(&mut self) {
    // unsafe {
    //     self.transform.pos_x += js_random_range(-10, 10) as f64;
    // }
    // }
}
