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
        }
    }

    pub fn update(&mut self, dt: f32, ground_rects: &[Rect]) {
        if !self.alive {
            self.death_timer -= dt;
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

        let color = match self.kind {
            EnemyKind::Goomba => Color::new(0.6, 0.3, 0.1, 1.0),
            EnemyKind::Koopa => GREEN,
            EnemyKind::Shell => DARKGREEN,
            EnemyKind::BuzzyBeetle => DARKBLUE,
            EnemyKind::BulletBill => DARKGRAY,
            EnemyKind::Paratroopa => Color::new(0.8, 0.2, 0.2, 1.0),
        };

        draw_rectangle(self.x, self.y, self.width, self.height, color);
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
        for i in 1..=segments {
            let frac = i as f32 / segments as f32;
            let bx = self.x + self.angle.cos() * self.length * frac;
            let by = self.y + self.angle.sin() * self.length * frac;
            draw_circle(bx, by, 4.0, ORANGE);
        }
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
        draw_rectangle(self.x, self.y, TILE_SIZE * 2.0, TILE_SIZE * 2.0, DARKGRAY);
        draw_rectangle(self.x + 4.0, self.y + TILE_SIZE * 0.5, 6.0, 6.0, WHITE);
        draw_rectangle(self.x + TILE_SIZE * 2.0 - 10.0, self.y + TILE_SIZE * 0.5, 6.0, 6.0, WHITE);
    }
}
