use engine::{Transform, Color};

pub struct Tile {
    pub transform: Transform,
    pub color: Color,
}

impl Tile {
    pub fn new(x: f64, y: f64, size: f64) -> Tile {
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

    pub fn default() -> Tile {
        Tile {
            transform: Transform {
                pos_x: 0_f64,
                pos_y: 0_f64,
                scale_x: 0_f64,
                scale_y: 0_f64,
            },
            color: Color::default(),
        }
    }
}
