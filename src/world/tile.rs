use engine::{Color, Transform};

pub struct Tile {
    pub transform: Transform,
    pub color: Color,
}

impl Tile {
    pub fn new(x: f64, y: f64, size: f64) -> Tile {
        Tile {
            transform: Transform::new(x, y, size, size),
            color: Color::default(),
        }
    }

    pub fn default() -> Tile {
        Tile {
            transform: Transform::default(),
            color: Color::default(),
        }
    }
}
