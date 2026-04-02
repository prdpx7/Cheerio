use macroquad::prelude::*;
use crate::constants::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerState {
    Small,
    Super,
    Fire,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub width: f32,
    pub height: f32,
    pub on_ground: bool,
    pub power_state: PowerState,
    pub is_dead: bool,
    pub stomp_chain: usize,
    pub star_timer: f32,
    pub fireballs: Vec<Fireball>,
}

impl Player {
    pub fn new(scroll_x: f32) -> Self {
        Self {
            x: scroll_x + INTERNAL_WIDTH * PLAYER_START_X_RATIO,
            y: GROUND_Y - PLAYER_HEIGHT_SMALL,
            vy: 0.0,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT_SMALL,
            on_ground: true,
            power_state: PowerState::Small,
            is_dead: false,
            stomp_chain: 0,
            star_timer: 0.0,
            fireballs: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32, scroll_speed: f32) {
        if self.is_dead {
            self.vy += GRAVITY * dt;
            self.y += self.vy * dt;
            return;
        }

        self.x += scroll_speed * dt;

        if self.star_timer > 0.0 {
            self.star_timer -= dt;
        }

        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Up) {
            if self.on_ground {
                self.vy = JUMP_VELOCITY;
                self.on_ground = false;
            }
        }

        if !self.on_ground
            && (is_key_released(KeyCode::Space) || is_key_released(KeyCode::Up))
            && self.vy < JUMP_CUT_VELOCITY
        {
            self.vy = JUMP_CUT_VELOCITY;
        }

        if self.power_state == PowerState::Fire
            && (is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::LeftShift) || is_key_pressed(KeyCode::RightShift))
            && self.fireballs.len() < 2
        {
            self.fireballs.push(Fireball::new(self.x + self.width, self.y + self.height * 0.5));
        }

        if !self.on_ground {
            self.vy += GRAVITY * dt;
        }

        self.y += self.vy * dt;

        self.height = match self.power_state {
            PowerState::Small => PLAYER_HEIGHT_SMALL,
            PowerState::Super | PowerState::Fire => PLAYER_HEIGHT_SUPER,
        };
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    pub fn die(&mut self) {
        self.is_dead = true;
        self.vy = JUMP_VELOCITY * 0.8;
    }

    pub fn take_damage(&mut self) {
        if self.star_timer > 0.0 {
            return;
        }
        match self.power_state {
            PowerState::Small => self.die(),
            PowerState::Super | PowerState::Fire => {
                self.power_state = PowerState::Small;
            }
        }
    }

    pub fn draw(&self) {
        let color = if self.star_timer > 0.0 {
            let t = (get_time() * 10.0) as u32 % 6;
            match t {
                0 => RED,
                1 => ORANGE,
                2 => YELLOW,
                3 => GREEN,
                4 => BLUE,
                _ => PURPLE,
            }
        } else {
            match self.power_state {
                PowerState::Small => RED,
                PowerState::Super => RED,
                PowerState::Fire => WHITE,
            }
        };

        draw_rectangle(self.x, self.y, self.width, self.height, color);
    }

    pub fn resolve_terrain(&mut self, ground_rects: &[Rect], platform_rects: &[Rect]) {
        self.on_ground = false;
        let player_rect = self.rect();

        for ground in ground_rects {
            if player_rect.overlaps(ground) {
                let feet = player_rect.y + player_rect.h;
                if self.vy >= 0.0 && feet > ground.y && feet < ground.y + ground.h + 4.0 {
                    self.y = ground.y - self.height;
                    self.vy = 0.0;
                    self.on_ground = true;
                }
            }
        }

        for plat in platform_rects {
            if player_rect.overlaps(plat) {
                let feet = player_rect.y + player_rect.h;
                if self.vy >= 0.0 && feet > plat.y && feet < plat.y + plat.h + 4.0 {
                    self.y = plat.y - self.height;
                    self.vy = 0.0;
                    self.on_ground = true;
                }
            }
        }
    }
}

pub struct Fireball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub alive: bool,
}

impl Fireball {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vx: 250.0,
            vy: 0.0,
            alive: true,
        }
    }

    pub fn update(&mut self, dt: f32, ground_rects: &[Rect]) {
        self.x += self.vx * dt;
        self.vy += GRAVITY * dt;
        self.y += self.vy * dt;

        let r = Rect::new(self.x, self.y, 6.0, 6.0);
        for g in ground_rects {
            if r.overlaps(g) && self.vy > 0.0 {
                self.y = g.y - 6.0;
                self.vy = -150.0;
            }
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, 6.0, 6.0)
    }

    pub fn draw(&self) {
        if self.alive {
            draw_circle(self.x + 3.0, self.y + 3.0, 3.0, ORANGE);
        }
    }
}
