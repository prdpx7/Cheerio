use macroquad::prelude::*;
use crate::constants::*;

pub struct GameCamera {
    pub scroll_x: f32,
    render_target: RenderTarget,
}

impl GameCamera {
    pub fn new() -> Self {
        let render_target = render_target(INTERNAL_WIDTH as u32, INTERNAL_HEIGHT as u32);
        render_target.texture.set_filter(FilterMode::Nearest);
        Self {
            scroll_x: 0.0,
            render_target,
        }
    }

    pub fn begin_render(&self) {
        set_camera(&Camera2D {
            target: vec2(self.scroll_x + INTERNAL_WIDTH * 0.5, INTERNAL_HEIGHT * 0.5),
            zoom: vec2(2.0 / INTERNAL_WIDTH, 2.0 / INTERNAL_HEIGHT),
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });
    }

    pub fn end_render(&self) {
        set_default_camera();

        let scale = (screen_width() / INTERNAL_WIDTH)
            .min(screen_height() / INTERNAL_HEIGHT);
        let dest_w = INTERNAL_WIDTH * scale;
        let dest_h = INTERNAL_HEIGHT * scale;
        let offset_x = (screen_width() - dest_w) * 0.5;
        let offset_y = (screen_height() - dest_h) * 0.5;

        clear_background(BLACK);
        draw_texture_ex(
            &self.render_target.texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dest_w, dest_h)),
                flip_y: true,
                ..Default::default()
            },
        );
    }

    pub fn advance(&mut self, speed: f32, dt: f32) {
        self.scroll_x += speed * dt;
    }
}
