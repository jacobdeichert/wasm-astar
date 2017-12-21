pub struct EngineState {
    pub last_timestamp: f64,
    pub last_fps_render_timestamp: f64,
    pub fps: f64,
}

impl EngineState {
    pub fn new() -> EngineState {
        EngineState {
            last_timestamp: 0_f64,
            last_fps_render_timestamp: 0_f64,
            fps: 0_f64,
        }
    }

    pub fn update(&mut self, elapsed_time: f64) {
        if self.last_timestamp != 0_f64 {
            let delta: f64 = (elapsed_time - self.last_timestamp) / 1000_f64;
            self.fps = 1_f64 / delta;
        }
        self.last_timestamp = elapsed_time;
    }

    pub fn render_fps<F>(&mut self, elapsed_time: f64, render_delay_ms: i32, render_cb: F)
    where
        F: FnOnce(),
    {
        if self.last_fps_render_timestamp + (render_delay_ms as f64) < elapsed_time {
            self.last_fps_render_timestamp = elapsed_time;
            render_cb();
        }
    }
}

#[derive(Clone)]
pub struct Transform {
    pub pos_x: f64,
    pub pos_y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
}

impl Transform {
    pub fn new(px: f64, py: f64, sx: f64, sy: f64) -> Transform {
        Transform {
            pos_x: px,
            pos_y: py,
            scale_x: sx,
            scale_y: sy,
        }
    }

    // pub fn from(t: &Transform) -> Transform {
    //     Transform::new(t.pos_x, t.pos_y, t.scale_x, t.scale_x)
    // }

    pub fn default() -> Transform {
        Transform::new(0_f64, 0_f64, 0_f64, 0_f64)
    }
}

#[derive(Clone)]
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
        Color {
            h: 0,
            s: 100,
            l: 80,
            a: 1,
        }
    }
}
