use macroquad::prelude::*;
use crate::constants::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyKind {
    Goomba,
    Koopa,
    Shell,
    BuzzyBeetle,
    BulletBill,
    Paratroopa,
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub kind: EnemyKind,
    pub vx: f32,
    pub vy: f32,
    pub alive: bool,
    pub on_ground: bool,
    pub death_timer: f32,
    pub death_vy: f32,
}

impl Enemy {
    pub fn new(kind: EnemyKind, x: f32, y: f32) -> Self {
        let (w, h, speed) = match kind {
            EnemyKind::Goomba => (TILE_SIZE, TILE_SIZE, -ENEMY_GOOMBA_SPEED),
            EnemyKind::Koopa => (TILE_SIZE, TILE_SIZE * 1.5, -ENEMY_KOOPA_SPEED),
            EnemyKind::Shell => (TILE_SIZE, TILE_SIZE, SHELL_SPEED),
            EnemyKind::BuzzyBeetle => (TILE_SIZE, TILE_SIZE, -ENEMY_GOOMBA_SPEED),
            EnemyKind::BulletBill => (TILE_SIZE * 1.5, TILE_SIZE, -SCROLL_SPEED_BASE * 1.5),
            EnemyKind::Paratroopa => (TILE_SIZE, TILE_SIZE * 1.5, -ENEMY_KOOPA_SPEED),
        };
        Self {
            x,
            y: y - h,
            width: w,
            height: h,
            kind,
            vx: speed,
            vy: 0.0,
            alive: true,
            on_ground: false,
            death_timer: 0.0,
            death_vy: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, ground_rects: &[Rect]) {
        if !self.alive {
            if self.death_timer > 0.0 {
                self.death_timer -= dt;
                self.death_vy += GRAVITY * dt;
                self.y += self.death_vy * dt;
            }
            return;
        }

        self.x += self.vx * dt;

        match self.kind {
            EnemyKind::Paratroopa => {
                self.vy = (get_time() as f32 * 3.0).sin() * 100.0;
                self.y += self.vy * dt;
            }
            EnemyKind::BulletBill => {}
            _ => {
                self.vy += GRAVITY * dt;
                self.y += self.vy * dt;

                self.on_ground = false;
                let r = self.rect();
                for g in ground_rects {
                    if r.overlaps(g) && self.vy >= 0.0 {
                        self.y = g.y - self.height;
                        self.vy = 0.0;
                        self.on_ground = true;
                    }
                }
            }
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    pub fn stomp(&mut self) -> Option<Enemy> {
        match self.kind {
            EnemyKind::Goomba | EnemyKind::BuzzyBeetle => {
                self.alive = false;
                self.death_timer = 0.3;
                self.death_vy = 0.0;
                None
            }
            EnemyKind::Koopa | EnemyKind::Paratroopa => {
                self.alive = false;
                self.death_timer = 0.0;
                let mut shell = Enemy::new(EnemyKind::Shell, self.x, self.y + self.height);
                shell.vx = SHELL_SPEED;
                Some(shell)
            }
            EnemyKind::BulletBill => {
                self.alive = false;
                self.death_timer = 0.3;
                self.death_vy = -100.0;
                None
            }
            EnemyKind::Shell => {
                self.alive = false;
                None
            }
        }
    }

    pub fn draw(&self) {
        if !self.alive && self.death_timer <= 0.0 {
            return;
        }

        let x = self.x;
        let y = self.y;
        let w = self.width;
        let h = self.height;

        if !self.alive {
            if self.kind == EnemyKind::Goomba {
                draw_rectangle(x + 1.0, y + h - 6.0, w - 2.0, 6.0, Color::new(0.55, 0.27, 0.07, 1.0));
                draw_rectangle(x + 3.0, y + h - 4.0, w - 6.0, 2.0, Color::new(0.9, 0.8, 0.6, 1.0));
                return;
            }
            if self.kind == EnemyKind::BuzzyBeetle {
                draw_circle(x + w * 0.5, y + h * 0.5, w * 0.4, Color::new(0.1, 0.1, 0.4, 1.0));
                return;
            }
        }

        match self.kind {
            EnemyKind::Goomba => {
                let cap = Color::new(0.55, 0.27, 0.07, 1.0);
                let face = Color::new(0.9, 0.8, 0.6, 1.0);
                let foot = Color::new(0.3, 0.15, 0.05, 1.0);
                draw_circle(x + w * 0.5, y + 5.0, w * 0.5, cap);
                draw_rectangle(x + 2.0, y + 5.0, w - 4.0, h * 0.4, face);
                draw_rectangle(x + 3.0, y + 7.0, 3.0, 3.0, WHITE);
                draw_rectangle(x + w - 6.0, y + 7.0, 3.0, 3.0, WHITE);
                draw_rectangle(x + 4.0, y + 8.0, 2.0, 2.0, BLACK);
                draw_rectangle(x + w - 5.0, y + 8.0, 2.0, 2.0, BLACK);
                draw_line(x + 3.0, y + 6.0, x + 6.0, y + 7.0, 1.0, BLACK);
                draw_line(x + w - 3.0, y + 6.0, x + w - 6.0, y + 7.0, 1.0, BLACK);
                draw_rectangle(x, y + h - 4.0, 5.0, 4.0, foot);
                draw_rectangle(x + w - 5.0, y + h - 4.0, 5.0, 4.0, foot);
            }
            EnemyKind::Koopa => {
                let shell_color = Color::new(0.1, 0.7, 0.2, 1.0);
                let head_color = Color::new(0.9, 0.8, 0.5, 1.0);
                draw_circle(x + w * 0.5, y + h * 0.55, w * 0.45, shell_color);
                draw_rectangle(x + 2.0, y + h * 0.4, w - 4.0, h * 0.45, shell_color);
                draw_circle(x + w * 0.7, y + 4.0, 5.0, head_color);
                draw_rectangle(x + w * 0.6 + 3.0, y + 2.0, 2.0, 2.0, BLACK);
                draw_rectangle(x + 1.0, y + h - 4.0, 4.0, 4.0, head_color);
                draw_rectangle(x + w - 5.0, y + h - 4.0, 4.0, 4.0, head_color);
                draw_line(x + 3.0, y + h * 0.5, x + w - 3.0, y + h * 0.5, 1.0, Color::new(0.05, 0.5, 0.1, 1.0));
            }
            EnemyKind::Shell => {
                let shell_color = Color::new(0.0, 0.5, 0.1, 1.0);
                draw_circle(x + w * 0.5, y + h * 0.5, w * 0.45, shell_color);
                draw_rectangle(x + 2.0, y + 3.0, w - 4.0, h - 6.0, shell_color);
                draw_line(x + 3.0, y + h * 0.5, x + w - 3.0, y + h * 0.5, 1.0, Color::new(0.0, 0.35, 0.05, 1.0));
                draw_circle(x + w * 0.5, y + h * 0.5, w * 0.2, Color::new(0.9, 0.85, 0.6, 1.0));
            }
            EnemyKind::BuzzyBeetle => {
                let shell_color = Color::new(0.1, 0.1, 0.4, 1.0);
                let highlight = Color::new(0.2, 0.2, 0.6, 1.0);
                draw_circle(x + w * 0.5, y + h * 0.5, w * 0.48, shell_color);
                draw_rectangle(x + 1.0, y + 3.0, w - 2.0, h - 6.0, shell_color);
                draw_rectangle(x + 3.0, y + 2.0, w - 6.0, 3.0, highlight);
                draw_rectangle(x + 3.0, y + h * 0.45, 2.0, 2.0, WHITE);
                draw_rectangle(x + w - 5.0, y + h * 0.45, 2.0, 2.0, WHITE);
                draw_rectangle(x, y + h - 3.0, 4.0, 3.0, Color::new(0.05, 0.05, 0.25, 1.0));
                draw_rectangle(x + w - 4.0, y + h - 3.0, 4.0, 3.0, Color::new(0.05, 0.05, 0.25, 1.0));
            }
            EnemyKind::BulletBill => {
                let body = Color::new(0.15, 0.15, 0.15, 1.0);
                draw_rectangle(x, y + 2.0, w, h - 4.0, body);
                draw_circle(x + w, y + h * 0.5, h * 0.4, body);
                draw_rectangle(x, y, 6.0, h, Color::new(0.3, 0.15, 0.05, 1.0));
                draw_rectangle(x + w * 0.5, y + 3.0, 4.0, 4.0, WHITE);
                draw_rectangle(x + w * 0.5 + 1.0, y + 4.0, 2.0, 2.0, BLACK);
                draw_triangle(
                    vec2(x, y),
                    vec2(x, y + 4.0),
                    vec2(x - 4.0, y + 2.0),
                    body,
                );
                draw_triangle(
                    vec2(x, y + h - 4.0),
                    vec2(x, y + h),
                    vec2(x - 4.0, y + h - 2.0),
                    body,
                );
            }
            EnemyKind::Paratroopa => {
                let shell_color = Color::new(0.8, 0.15, 0.15, 1.0);
                let head_color = Color::new(0.9, 0.8, 0.5, 1.0);
                let wing = Color::new(1.0, 1.0, 1.0, 0.8);
                draw_circle(x + w * 0.5, y + h * 0.55, w * 0.45, shell_color);
                draw_rectangle(x + 2.0, y + h * 0.4, w - 4.0, h * 0.45, shell_color);
                draw_circle(x + w * 0.7, y + 4.0, 5.0, head_color);
                draw_rectangle(x + w * 0.6 + 3.0, y + 2.0, 2.0, 2.0, BLACK);
                draw_rectangle(x + 1.0, y + h - 4.0, 4.0, 4.0, head_color);
                draw_rectangle(x + w - 5.0, y + h - 4.0, 4.0, 4.0, head_color);
                let wing_y = y + h * 0.3;
                draw_triangle(vec2(x - 2.0, wing_y), vec2(x + 3.0, wing_y + 4.0), vec2(x - 6.0, wing_y - 6.0), wing);
                draw_triangle(vec2(x + w + 2.0, wing_y), vec2(x + w - 3.0, wing_y + 4.0), vec2(x + w + 6.0, wing_y - 6.0), wing);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FireBar {
    pub x: f32,
    pub y: f32,
    pub length: f32,
    pub speed: f32,
    pub angle: f32,
}

impl FireBar {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            length: TILE_SIZE * 3.0,
            speed: 2.0,
            angle: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.angle += self.speed * dt;
    }

    pub fn hits_player(&self, player_rect: &Rect) -> bool {
        let segments = 6;
        for i in 1..=segments {
            let frac = i as f32 / segments as f32;
            let bx = self.x + self.angle.cos() * self.length * frac;
            let by = self.y + self.angle.sin() * self.length * frac;
            let ball_rect = Rect::new(bx - 4.0, by - 4.0, 8.0, 8.0);
            if player_rect.overlaps(&ball_rect) {
                return true;
            }
        }
        false
    }

    pub fn draw(&self) {
        let segments = 6;
        let t = get_time() as f32;
        for i in 1..=segments {
            let frac = i as f32 / segments as f32;
            let bx = self.x + self.angle.cos() * self.length * frac;
            let by = self.y + self.angle.sin() * self.length * frac;
            let flicker = (t * 12.0 + i as f32 * 2.0).sin() * 0.15;
            draw_circle(bx, by, 6.0, Color::new(1.0, 0.3, 0.0, 0.25 + flicker));
            draw_circle(bx, by, 4.0, Color::new(1.0, 0.5 + flicker, 0.0, 0.8));
            draw_circle(bx, by, 2.0, Color::new(1.0, 0.9, 0.3, 1.0));
        }
        draw_circle(self.x, self.y, 5.0, Color::new(0.3, 0.3, 0.3, 1.0));
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThwompState {
    Waiting,
    Slamming,
    Rising,
}

#[derive(Debug, Clone)]
pub struct Thwomp {
    pub x: f32,
    pub y: f32,
    pub home_y: f32,
    pub state: ThwompState,
    pub vy: f32,
}

impl Thwomp {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            home_y: y,
            state: ThwompState::Waiting,
            vy: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, player_x: f32) {
        match self.state {
            ThwompState::Waiting => {
                if (player_x - self.x).abs() < TILE_SIZE * 4.0 {
                    self.state = ThwompState::Slamming;
                }
            }
            ThwompState::Slamming => {
                self.vy += GRAVITY * 2.0 * dt;
                self.y += self.vy * dt;
                if self.y >= GROUND_Y - TILE_SIZE * 2.0 {
                    self.y = GROUND_Y - TILE_SIZE * 2.0;
                    self.vy = 0.0;
                    self.state = ThwompState::Rising;
                }
            }
            ThwompState::Rising => {
                self.y -= 30.0 * dt;
                if self.y <= self.home_y {
                    self.y = self.home_y;
                    self.state = ThwompState::Waiting;
                }
            }
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, TILE_SIZE * 2.0, TILE_SIZE * 2.0)
    }

    pub fn draw(&self) {
        let x = self.x;
        let y = self.y;
        let s = TILE_SIZE * 2.0;
        let stone = Color::new(0.55, 0.55, 0.6, 1.0);
        let dark_stone = Color::new(0.4, 0.4, 0.45, 1.0);
        draw_rectangle(x, y, s, s, stone);
        draw_rectangle(x, y, s, 3.0, dark_stone);
        draw_rectangle(x, y + s - 3.0, s, 3.0, dark_stone);
        draw_rectangle(x, y, 3.0, s, dark_stone);
        draw_rectangle(x + s - 3.0, y, 3.0, s, dark_stone);
        draw_rectangle(x + 5.0, y + 8.0, 7.0, 7.0, WHITE);
        draw_rectangle(x + s - 12.0, y + 8.0, 7.0, 7.0, WHITE);
        draw_rectangle(x + 7.0, y + 10.0, 3.0, 3.0, BLACK);
        draw_rectangle(x + s - 10.0, y + 10.0, 3.0, 3.0, BLACK);
        let teeth_y = y + s * 0.65;
        for i in 0..4 {
            let tx = x + 4.0 + i as f32 * 7.0;
            draw_triangle(
                vec2(tx, teeth_y),
                vec2(tx + 6.0, teeth_y),
                vec2(tx + 3.0, teeth_y + 5.0),
                WHITE,
            );
        }
        draw_line(x + 5.0, y + 8.0, x + 9.0, y + 10.0, 1.5, BLACK);
        draw_line(x + s - 5.0, y + 8.0, x + s - 9.0, y + 10.0, 1.5, BLACK);
    }
}
