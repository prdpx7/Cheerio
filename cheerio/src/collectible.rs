use macroquad::prelude::*;
use crate::constants::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollectibleKind {
    Coin,
    Mushroom,
    FireFlower,
    Star,
}

#[derive(Debug, Clone)]
pub struct Collectible {
    pub x: f32,
    pub y: f32,
    pub kind: CollectibleKind,
    pub collected: bool,
    pub vy: f32,
    pub active: bool,
}

impl Collectible {
    pub fn new(kind: CollectibleKind, x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            kind,
            collected: false,
            vy: 0.0,
            active: kind == CollectibleKind::Coin,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.vy = -200.0;
    }

    pub fn update(&mut self, dt: f32, ground_rects: &[Rect]) {
        if self.collected || !self.active {
            return;
        }

        match self.kind {
            CollectibleKind::Coin => {
                self.y += (get_time() as f32 * 4.0).sin() * 0.3;
            }
            CollectibleKind::Mushroom | CollectibleKind::Star => {
                self.vy += GRAVITY * 0.5 * dt;
                self.y += self.vy * dt;
                self.x += 30.0 * dt;

                let r = self.rect();
                for g in ground_rects {
                    if r.overlaps(g) && self.vy >= 0.0 {
                        self.y = g.y - TILE_SIZE;
                        self.vy = 0.0;
                    }
                }
            }
            CollectibleKind::FireFlower => {}
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, TILE_SIZE, TILE_SIZE)
    }

    pub fn draw(&self) {
        if self.collected || !self.active {
            return;
        }

        let color = match self.kind {
            CollectibleKind::Coin => GOLD,
            CollectibleKind::Mushroom => Color::new(0.9, 0.2, 0.1, 1.0),
            CollectibleKind::FireFlower => ORANGE,
            CollectibleKind::Star => YELLOW,
        };

        match self.kind {
            CollectibleKind::Coin => {
                let size = TILE_SIZE * 0.6;
                draw_rectangle(self.x + 2.0, self.y + 2.0, size, size, color);
            }
            _ => {
                draw_rectangle(self.x, self.y, TILE_SIZE, TILE_SIZE, color);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct QuestionBlock {
    pub x: f32,
    pub y: f32,
    pub contents: CollectibleKind,
    pub hit: bool,
    pub bounce_timer: f32,
}

impl QuestionBlock {
    pub fn new(x: f32, y: f32, contents: CollectibleKind) -> Self {
        Self {
            x,
            y,
            contents,
            hit: false,
            bounce_timer: 0.0,
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, TILE_SIZE, TILE_SIZE)
    }

    pub fn hit_block(&mut self) -> Option<Collectible> {
        if self.hit {
            return None;
        }
        self.hit = true;
        self.bounce_timer = 0.2;
        let mut c = Collectible::new(self.contents, self.x, self.y - TILE_SIZE);
        c.active = true;
        if self.contents != CollectibleKind::Coin {
            c.activate();
        }
        Some(c)
    }

    pub fn update(&mut self, dt: f32) {
        if self.bounce_timer > 0.0 {
            self.bounce_timer -= dt;
        }
    }

    pub fn draw(&self) {
        let offset_y = if self.bounce_timer > 0.0 { -4.0 } else { 0.0 };
        let color = if self.hit {
            Color::new(0.4, 0.3, 0.2, 1.0)
        } else {
            GOLD
        };
        draw_rectangle(self.x, self.y + offset_y, TILE_SIZE, TILE_SIZE, color);
        if !self.hit {
            draw_text("?", self.x + 3.0, self.y + offset_y + 13.0, 16.0, WHITE);
        }
    }
}
