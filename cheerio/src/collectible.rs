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

        let x = self.x;
        let y = self.y;
        let s = TILE_SIZE;

        match self.kind {
            CollectibleKind::Coin => {
                let cx = x + s * 0.5;
                let cy = y + s * 0.5;
                draw_circle(cx, cy, s * 0.35, Color::new(0.85, 0.65, 0.1, 1.0));
                draw_circle(cx, cy, s * 0.28, Color::new(1.0, 0.85, 0.2, 1.0));
                draw_circle(cx - 1.0, cy - 1.0, s * 0.12, Color::new(1.0, 0.95, 0.6, 0.7));
            }
            CollectibleKind::Mushroom => {
                let cap = Color::new(0.9, 0.15, 0.1, 1.0);
                let stem = Color::new(0.9, 0.85, 0.7, 1.0);
                draw_circle(x + s * 0.5, y + 5.0, s * 0.48, cap);
                draw_rectangle(x + 3.0, y + 5.0, s - 6.0, s * 0.45, stem);
                draw_rectangle(x + 4.0, y + s - 4.0, s - 8.0, 4.0, stem);
                draw_circle(x + s * 0.3, y + 3.0, 2.5, WHITE);
                draw_circle(x + s * 0.7, y + 3.0, 2.5, WHITE);
                draw_circle(x + s * 0.5, y + 1.0, 2.0, WHITE);
                draw_rectangle(x + 4.0, y + 7.0, 2.0, 2.0, BLACK);
                draw_rectangle(x + s - 6.0, y + 7.0, 2.0, 2.0, BLACK);
            }
            CollectibleKind::FireFlower => {
                let petal = Color::new(1.0, 0.35, 0.1, 1.0);
                let center = Color::new(1.0, 0.85, 0.1, 1.0);
                let stem_c = Color::new(0.2, 0.6, 0.2, 1.0);
                let cx = x + s * 0.5;
                let cy = y + 5.0;
                draw_circle(cx, cy - 3.0, 3.0, petal);
                draw_circle(cx + 3.0, cy, 3.0, petal);
                draw_circle(cx - 3.0, cy, 3.0, petal);
                draw_circle(cx, cy + 3.0, 3.0, petal);
                draw_circle(cx, cy, 2.5, center);
                draw_rectangle(cx - 1.0, cy + 3.0, 2.0, s - 8.0, stem_c);
                draw_rectangle(cx - 3.0, cy + 5.0, 3.0, 2.0, Color::new(0.15, 0.5, 0.15, 1.0));
            }
            CollectibleKind::Star => {
                let cx = x + s * 0.5;
                let cy = y + s * 0.5;
                let outer_r = s * 0.45;
                let inner_r = s * 0.2;
                let star_color = Color::new(1.0, 0.9, 0.1, 1.0);
                let highlight = Color::new(1.0, 1.0, 0.7, 1.0);
                for i in 0..5 {
                    let angle1 = std::f32::consts::PI * 2.0 * i as f32 / 5.0 - std::f32::consts::PI / 2.0;
                    let angle2 = std::f32::consts::PI * 2.0 * (i as f32 + 0.5) / 5.0 - std::f32::consts::PI / 2.0;
                    let angle3 = std::f32::consts::PI * 2.0 * ((i + 1) % 5) as f32 / 5.0 - std::f32::consts::PI / 2.0;
                    let p1 = vec2(cx + angle1.cos() * outer_r, cy + angle1.sin() * outer_r);
                    let p2 = vec2(cx + angle2.cos() * inner_r, cy + angle2.sin() * inner_r);
                    let p3 = vec2(cx + angle3.cos() * outer_r, cy + angle3.sin() * outer_r);
                    draw_triangle(p1, p2, vec2(cx, cy), star_color);
                    draw_triangle(p2, p3, vec2(cx, cy), star_color);
                }
                draw_circle(cx, cy, 2.5, highlight);
                draw_rectangle(cx - 1.0, cy - 2.0, 2.0, 2.0, BLACK);
                draw_rectangle(cx + 1.0, cy - 2.0, 2.0, 2.0, BLACK);
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
        let x = self.x;
        let y = self.y + offset_y;
        let s = TILE_SIZE;

        if self.hit {
            let used = Color::new(0.45, 0.35, 0.25, 1.0);
            let border = Color::new(0.35, 0.25, 0.15, 1.0);
            draw_rectangle(x, y, s, s, used);
            draw_rectangle(x, y, s, 2.0, border);
            draw_rectangle(x, y + s - 2.0, s, 2.0, border);
            draw_rectangle(x, y, 2.0, s, border);
            draw_rectangle(x + s - 2.0, y, 2.0, s, border);
        } else {
            let face = Color::new(0.9, 0.7, 0.15, 1.0);
            let border = Color::new(0.65, 0.45, 0.1, 1.0);
            let highlight = Color::new(1.0, 0.85, 0.3, 1.0);
            draw_rectangle(x, y, s, s, face);
            draw_rectangle(x, y, s, 2.0, border);
            draw_rectangle(x, y + s - 2.0, s, 2.0, border);
            draw_rectangle(x, y, 2.0, s, border);
            draw_rectangle(x + s - 2.0, y, 2.0, s, border);
            draw_rectangle(x + 2.0, y + 2.0, 3.0, 2.0, highlight);
            draw_rectangle(x + 2.0, y + 2.0, 2.0, 3.0, highlight);
            draw_text("?", x + 3.0, y + 13.0, 14.0, WHITE);
        }
    }
}
