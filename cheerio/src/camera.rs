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
        set_camera(&Camera2D {
            target: vec2(
                self.scroll_x + INTERNAL_WIDTH * 0.5,
                INTERNAL_HEIGHT * 0.5,
            ),
            zoom: vec2(2.0 / INTERNAL_WIDTH, 2.0 / INTERNAL_HEIGHT),
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
