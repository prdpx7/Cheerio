use macroquad::prelude::*;
use crate::constants::*;

pub struct ScoreManager {
    pub score: u32,
    pub coins: u32,
    pub high_score: u32,
    pub distance: f32,
    pub enemies_stomped: u32,
    pub cycle: u32,
}

impl ScoreManager {
    pub fn new() -> Self {
        Self {
            score: 0,
            coins: 0,
            high_score: 0,
            distance: 0.0,
            enemies_stomped: 0,
            cycle: 1,
        }
    }

    pub fn add_distance(&mut self, dx: f32) {
        self.distance += dx;
        self.score += (dx * self.cycle as f32) as u32;
    }

    pub fn add_coin(&mut self) {
        self.coins += 1;
        self.score += COIN_SCORE;
    }

    pub fn add_stomp(&mut self, chain: usize) {
        self.enemies_stomped += 1;
        let idx = chain.min(STOMP_CHAIN.len() - 1);
        self.score += STOMP_CHAIN[idx];
    }

    pub fn add_powerup(&mut self) {
        self.score += POWERUP_SCORE;
    }

    pub fn finalize(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }

    pub fn is_new_high_score(&self) -> bool {
        self.score > self.high_score
    }

    pub fn draw_hud(&self, camera_x: f32, zone_name: &str) {
        let left = camera_x + 8.0;
        let hud_y = 16.0;

        draw_text(&format!("SCORE {}", self.score), left, hud_y, 16.0, WHITE);
        draw_text(&format!("x{}", self.coins), left + INTERNAL_WIDTH * 0.3, hud_y, 16.0, GOLD);
        draw_text(zone_name, left + INTERNAL_WIDTH * 0.45, hud_y, 16.0, WHITE);
        draw_text(
            &format!("HI-{}", self.high_score),
            camera_x + INTERNAL_WIDTH - 108.0,
            hud_y,
            16.0,
            WHITE,
        );
    }
}
