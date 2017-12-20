pub struct Transform {
    pub pos_x: f64,
    pub pos_y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
}

pub struct Color {
    pub h: u16,
    pub s: u16,
    pub l: u16,
    pub a: u16,
}

impl Color {
    pub fn new(h: u16, s: u16, l: u16, a: u16) -> Color {
        Color { h, s, l, a }
    }

    pub fn default() -> Color {
        Color { h: 0, s: 100, l: 80, a: 1 }
    }
}
