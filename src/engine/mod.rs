use std::collections::HashMap;

pub enum KeyCode {
    ArrowUp = 38,
    ArrowDown = 40,
    ArrowLeft = 37,
    ArrowRight = 39,
    Spacebar = 32,
}

pub struct EngineState {
    pub last_timestamp: f64,
    pub last_fps_render_timestamp: f64,
    pub fps: f64,
    pub mouse_x: i32,
    pub mouse_y: i32,
    key_state: HashMap<u32, bool>,
    was_down: HashMap<u32, bool>,
}

impl EngineState {
    pub fn new() -> EngineState {
        EngineState {
            last_timestamp: 0_f64,
            last_fps_render_timestamp: 0_f64,
            fps: 0_f64,
            mouse_x: 0,
            mouse_y: 0,
            key_state: HashMap::new(),
            was_down: HashMap::new(),
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

    pub fn set_key_down(&mut self, raw_key_code: u32) {
        let was_down = self.is_raw_key_down(raw_key_code);
        self.was_down.insert(raw_key_code, was_down);
        if !was_down {
            self.key_state.insert(raw_key_code, true);
        } else {
            self.was_down.insert(raw_key_code, true);
        }
    }

    pub fn set_key_up(&mut self, raw_key_code: u32) {
        self.was_down.insert(raw_key_code, false);
        self.key_state.insert(raw_key_code, false);
    }

    pub fn is_key_down(&self, key_code: KeyCode) -> bool {
        self.is_raw_key_down(key_code as u32)
    }

    fn is_raw_key_down(&self, raw_key_code: u32) -> bool {
        match self.key_state.get(&raw_key_code) {
            Some(is_down) => *is_down,
            None => false,
        }
    }

    pub fn was_key_down(&self, key_code: KeyCode) -> bool {
        match self.was_down.get(&(key_code as u32)) {
            Some(was_down) => *was_down,
            None => false,
        }
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) {
        self.mouse_x = x;
        self.mouse_y = y;
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

    pub fn default() -> Transform {
        Transform::new(0_f64, 0_f64, 0_f64, 0_f64)
    }
}

#[derive(Clone)]
pub struct Color {
    pub h: u16,
    pub s: u16,
    pub l: u16,
    pub a: f32,
}

impl Color {
    pub fn new(h: u16, s: u16, l: u16, a: f32) -> Color {
        Color { h, s, l, a }
    }

    pub fn default() -> Color {
        Color {
            h: 0,
            s: 100,
            l: 80,
            a: 1_f32,
        }
    }
}
