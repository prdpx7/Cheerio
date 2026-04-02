use macroquad::prelude::*;
use crate::constants::*;

pub struct GameCamera {
    pub scroll_x: f32,
}

impl GameCamera {
    pub fn new() -> Self {
        Self { scroll_x: 0.0 }
    }

    pub fn begin_render(&self) {
        let scale = (screen_width() / INTERNAL_WIDTH).min(screen_height() / INTERNAL_HEIGHT);
        let offset_x = (screen_width() - INTERNAL_WIDTH * scale) * 0.5 / scale;
        let offset_y = (screen_height() - INTERNAL_HEIGHT * scale) * 0.5 / scale;

        set_camera(&Camera2D {
            target: vec2(
                self.scroll_x + INTERNAL_WIDTH * 0.5 - offset_x,
                INTERNAL_HEIGHT * 0.5 - offset_y,
            ),
            zoom: vec2(
                2.0 * scale / screen_width(),
                2.0 * scale / screen_height(),
            ),
            ..Default::default()
        });
    }

    pub fn end_render(&self) {
        set_default_camera();
    }

    pub fn advance(&mut self, speed: f32, dt: f32) {
        self.scroll_x += speed * dt;
    }
}
