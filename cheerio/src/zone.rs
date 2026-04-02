use macroquad::prelude::*;
use crate::constants::*;
use crate::enemy::EnemyKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZoneType {
    Grassland,
    Underground,
    Sky,
    Castle,
}

impl ZoneType {
    pub fn next(self) -> Self {
        match self {
            ZoneType::Grassland => ZoneType::Underground,
            ZoneType::Underground => ZoneType::Sky,
            ZoneType::Sky => ZoneType::Castle,
            ZoneType::Castle => ZoneType::Grassland,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ZoneType::Grassland => "GRASSLAND",
            ZoneType::Underground => "UNDERGROUND",
            ZoneType::Sky => "SKY",
            ZoneType::Castle => "CASTLE",
        }
    }

    pub fn bg_color(&self) -> Color {
        match self {
            ZoneType::Grassland => Color::new(0.4, 0.7, 1.0, 1.0),
            ZoneType::Underground => Color::new(0.1, 0.1, 0.18, 1.0),
            ZoneType::Sky => Color::new(0.6, 0.85, 1.0, 1.0),
            ZoneType::Castle => Color::new(0.17, 0.1, 0.0, 1.0),
        }
    }

    pub fn ground_color(&self) -> Color {
        match self {
            ZoneType::Grassland => Color::new(0.6, 0.4, 0.2, 1.0),
            ZoneType::Underground => Color::new(0.4, 0.35, 0.3, 1.0),
            ZoneType::Sky => Color::new(0.9, 0.9, 0.95, 1.0),
            ZoneType::Castle => Color::new(0.3, 0.25, 0.2, 1.0),
        }
    }

    pub fn ground_top_color(&self) -> Color {
        match self {
            ZoneType::Grassland => Color::new(0.3, 0.7, 0.3, 1.0),
            ZoneType::Underground => Color::new(0.5, 0.4, 0.35, 1.0),
            ZoneType::Sky => Color::new(1.0, 1.0, 1.0, 1.0),
            ZoneType::Castle => Color::new(0.5, 0.3, 0.2, 1.0),
        }
    }

    pub fn enemy_pool(&self) -> Vec<EnemyKind> {
        match self {
            ZoneType::Grassland => vec![EnemyKind::Goomba, EnemyKind::Koopa],
            ZoneType::Underground => vec![EnemyKind::BuzzyBeetle, EnemyKind::BulletBill],
            ZoneType::Sky => vec![EnemyKind::Paratroopa],
            ZoneType::Castle => vec![EnemyKind::Goomba],
        }
    }

    pub fn gap_is_lethal(&self) -> bool {
        matches!(self, ZoneType::Underground | ZoneType::Castle)
    }
}

pub struct ZoneManager {
    pub current: ZoneType,
    pub timer: f32,
    pub cycle: u32,
    pub transition_timer: f32,
}

impl ZoneManager {
    pub fn new() -> Self {
        Self {
            current: ZoneType::Grassland,
            timer: 0.0,
            cycle: 0,
            transition_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        self.timer += dt;

        if self.transition_timer > 0.0 {
            self.transition_timer -= dt;
            return self.transition_timer <= 0.0;
        }

        if self.timer >= ZONE_DURATION {
            self.timer = 0.0;
            self.current = self.current.next();
            if self.current == ZoneType::Grassland {
                self.cycle += 1;
            }
            self.transition_timer = 1.0;
        }

        false
    }

    pub fn scroll_speed(&self) -> f32 {
        let idx = (self.cycle as usize).min(SCROLL_SPEED_MULTIPLIERS.len() - 1);
        SCROLL_SPEED_BASE * SCROLL_SPEED_MULTIPLIERS[idx]
    }

    pub fn draw_transition(&self, camera_x: f32) {
        if self.transition_timer > 0.0 {
            let alpha = self.transition_timer.min(1.0);
            draw_rectangle(
                camera_x,
                0.0,
                INTERNAL_WIDTH,
                INTERNAL_HEIGHT,
                Color::new(0.0, 0.0, 0.0, alpha * 0.8),
            );
            draw_text(
                self.current.name(),
                camera_x + INTERNAL_WIDTH * 0.3,
                INTERNAL_HEIGHT * 0.5,
                32.0,
                Color::new(1.0, 1.0, 1.0, alpha),
            );
        }
    }
}
