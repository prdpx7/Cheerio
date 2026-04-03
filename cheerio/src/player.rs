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
    pub jumped: bool,
    pub fired: bool,
    pub ducking: bool,
    jump_buffer: f32,
    coyote_timer: f32,
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
            jumped: false,
            fired: false,
            ducking: false,
            jump_buffer: 0.0,
            coyote_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, scroll_speed: f32) {
        if self.is_dead {
            self.vy += GRAVITY * dt;
            self.y += self.vy * dt;
            return;
        }

        let mut want_jump = false;
        let mut want_duck = false;
        let mut want_fire = false;

        want_jump |= is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Up);
        want_duck |= is_key_down(KeyCode::Down);
        want_fire |= is_key_pressed(KeyCode::X)
            || is_key_pressed(KeyCode::LeftShift)
            || is_key_pressed(KeyCode::RightShift);

        for touch in touches() {
            if touch.phase == TouchPhase::Started {
                if touch.position.x > screen_width() * 0.5
                    && self.power_state == PowerState::Fire
                    && self.fireballs.len() < 2
                {
                    want_fire = true;
                } else {
                    want_jump = true;
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, _) = mouse_position();
            if mx > screen_width() * 0.5
                && self.power_state == PowerState::Fire
                && self.fireballs.len() < 2
            {
                want_fire = true;
            } else {
                want_jump = true;
            }
        }

        self.x += scroll_speed * dt;

        if self.star_timer > 0.0 {
            self.star_timer -= dt;
        }

        if self.on_ground {
            self.coyote_timer = 0.1;
        } else {
            self.coyote_timer -= dt;
        }

        if want_jump {
            self.jump_buffer = 0.15;
        }
        self.jump_buffer -= dt;

        if self.jump_buffer > 0.0 && (self.on_ground || self.coyote_timer > 0.0) {
            self.vy = JUMP_VELOCITY;
            self.on_ground = false;
            self.jumped = true;
            self.jump_buffer = 0.0;
            self.coyote_timer = 0.0;
            self.ducking = false;
        }

        if !self.on_ground
            && (is_key_released(KeyCode::Space) || is_key_released(KeyCode::Up))
            && self.vy < JUMP_CUT_VELOCITY
        {
            self.vy = JUMP_CUT_VELOCITY;
        }

        if want_fire && self.power_state == PowerState::Fire && self.fireballs.len() < 2 {
            self.fireballs.push(Fireball::new(self.x + self.width, self.y + self.height * 0.5));
            self.fired = true;
        }

        let prev_height = self.height;
        if want_duck && self.on_ground {
            self.ducking = true;
        } else {
            self.ducking = false;
        }

        if !self.on_ground && want_duck && self.vy > 0.0 {
            self.vy += GRAVITY * 1.5 * dt;
        }

        if !self.on_ground {
            self.vy += GRAVITY * dt;
        }

        self.y += self.vy * dt;

        let new_height = if self.ducking {
            PLAYER_HEIGHT_SMALL * 0.6
        } else {
            match self.power_state {
                PowerState::Small => PLAYER_HEIGHT_SMALL,
                PowerState::Super | PowerState::Fire => PLAYER_HEIGHT_SUPER,
            }
        };

        if self.on_ground && new_height != prev_height {
            self.y += prev_height - new_height;
        }
        self.height = new_height;
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
        let star = self.star_timer > 0.0;
        let rainbow = || {
            let t = (get_time() * 10.0) as u32 % 6;
            match t {
                0 => RED,
                1 => ORANGE,
                2 => YELLOW,
                3 => GREEN,
                4 => BLUE,
                _ => PURPLE,
            }
        };

        let hat_color = if star {
            rainbow()
        } else {
            match self.power_state {
                PowerState::Small | PowerState::Super => Color::new(0.9, 0.1, 0.1, 1.0),
                PowerState::Fire => WHITE,
            }
        };

        let overalls_color = if star {
            rainbow()
        } else {
            match self.power_state {
                PowerState::Small | PowerState::Super => Color::new(0.1, 0.2, 0.8, 1.0),
                PowerState::Fire => Color::new(0.9, 0.1, 0.1, 1.0),
            }
        };

        let skin = Color::new(0.96, 0.76, 0.53, 1.0);
        let shoe = Color::new(0.45, 0.22, 0.1, 1.0);
        let x = self.x;
        let y = self.y;

        if self.ducking {
            let h = self.height;
            draw_rectangle(x + 2.0, y, 10.0, 3.0, hat_color);
            draw_rectangle(x + 2.0, y + 3.0, 10.0, 3.0, skin);
            draw_rectangle(x + 4.0, y + 4.0, 2.0, 1.0, BLACK);
            draw_rectangle(x + 8.0, y + 4.0, 2.0, 1.0, BLACK);
            draw_rectangle(x + 1.0, y + 6.0, 12.0, h - 8.0, overalls_color);
            draw_rectangle(x + 1.0, y + h - 2.0, 12.0, 2.0, shoe);
            return;
        }

        match self.power_state {
            PowerState::Small => {
                draw_rectangle(x + 2.0, y, 10.0, 4.0, hat_color);
                draw_rectangle(x + 0.0, y + 2.0, 3.0, 2.0, hat_color);
                draw_rectangle(x + 2.0, y + 4.0, 10.0, 4.0, skin);
                draw_rectangle(x + 4.0, y + 5.0, 2.0, 2.0, BLACK);
                draw_rectangle(x + 8.0, y + 5.0, 2.0, 2.0, BLACK);
                draw_rectangle(x + 10.0, y + 6.0, 3.0, 2.0, skin);
                draw_rectangle(x + 1.0, y + 8.0, 12.0, 5.0, overalls_color);
                draw_rectangle(x + 5.0, y + 9.0, 4.0, 2.0, Color::new(0.9, 0.8, 0.2, 1.0));
                draw_rectangle(x + 1.0, y + 13.0, 5.0, 3.0, shoe);
                draw_rectangle(x + 8.0, y + 13.0, 5.0, 3.0, shoe);
            }
            PowerState::Super | PowerState::Fire => {
                draw_rectangle(x + 2.0, y, 10.0, 5.0, hat_color);
                draw_rectangle(x + 0.0, y + 3.0, 3.0, 2.0, hat_color);
                draw_rectangle(x + 2.0, y + 5.0, 10.0, 5.0, skin);
                draw_rectangle(x + 4.0, y + 6.0, 2.0, 2.0, BLACK);
                draw_rectangle(x + 8.0, y + 6.0, 2.0, 2.0, BLACK);
                draw_rectangle(x + 10.0, y + 8.0, 3.0, 2.0, skin);
                draw_rectangle(x + 6.0, y + 9.0, 3.0, 1.0, Color::new(0.7, 0.5, 0.3, 1.0));
                draw_rectangle(x + 0.0, y + 10.0, 14.0, 3.0, hat_color);
                draw_rectangle(x + 1.0, y + 13.0, 12.0, 10.0, overalls_color);
                draw_rectangle(x + 5.0, y + 14.0, 4.0, 3.0, Color::new(0.9, 0.8, 0.2, 1.0));
                draw_rectangle(x + 0.0, y + 13.0, 3.0, 5.0, skin);
                draw_rectangle(x + 11.0, y + 13.0, 3.0, 5.0, skin);
                draw_rectangle(x + 1.0, y + 23.0, 12.0, 4.0, overalls_color);
                draw_rectangle(x + 0.0, y + 27.0, 6.0, 5.0, shoe);
                draw_rectangle(x + 8.0, y + 27.0, 6.0, 5.0, shoe);
            }
        }
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
            let t = (get_time() * 8.0).sin() as f32;
            let outer = Color::new(1.0, 0.3 + t * 0.1, 0.0, 0.4);
            let mid = Color::new(1.0, 0.5 + t * 0.15, 0.0, 0.8);
            let core = Color::new(1.0, 0.9, 0.2, 1.0);
            draw_circle(self.x + 3.0, self.y + 3.0, 5.0, outer);
            draw_circle(self.x + 3.0, self.y + 3.0, 3.5, mid);
            draw_circle(self.x + 3.0, self.y + 3.0, 2.0, core);
            draw_circle(self.x - 1.0, self.y + 3.0, 2.0, Color::new(1.0, 0.4, 0.0, 0.3));
            draw_circle(self.x - 4.0, self.y + 3.0, 1.5, Color::new(1.0, 0.3, 0.0, 0.15));
        }
    }
}
