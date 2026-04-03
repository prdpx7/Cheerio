use macroquad::audio::{self, Sound, PlaySoundParams};
use macroquad::prelude::*;

pub struct AudioManager {
    pub jump: Option<Sound>,
    pub coin: Option<Sound>,
    pub stomp: Option<Sound>,
    pub powerup: Option<Sound>,
    pub fireball: Option<Sound>,
    pub death: Option<Sound>,
    pub oneup: Option<Sound>,
    pub bump: Option<Sound>,
    pub bgm: Option<Sound>,
}

const ASSET_PATHS: [&str; 9] = [
    "assets/audio/smb_jump-small.wav",
    "assets/audio/smb_coin.wav",
    "assets/audio/smb_stomp.wav",
    "assets/audio/smb_powerup.wav",
    "assets/audio/smb_fireball.wav",
    "assets/audio/smb_mariodie.wav",
    "assets/audio/smb_1-up.wav",
    "assets/audio/smb_bump.wav",
    "assets/audio/bgm_main.ogg",
];

fn draw_loading_screen(progress: f32) {
    clear_background(Color::new(0.29, 0.72, 1.0, 1.0));

    let title = "CHEERIO";
    let m = measure_text(title, None, 48, 1.0);
    draw_text(title, (screen_width() - m.width) * 0.5, screen_height() * 0.35, 48.0, WHITE);

    let sub = "An Endless Adventure";
    let ms = measure_text(sub, None, 16, 1.0);
    draw_text(sub, (screen_width() - ms.width) * 0.5, screen_height() * 0.35 + 30.0, 16.0, Color::new(1.0, 1.0, 1.0, 0.8));

    let bar_w = 200.0;
    let bar_h = 10.0;
    let bar_x = (screen_width() - bar_w) * 0.5;
    let bar_y = screen_height() * 0.55;

    draw_rectangle(bar_x, bar_y, bar_w, bar_h, Color::new(0.0, 0.0, 0.0, 0.3));
    draw_rectangle(bar_x, bar_y, bar_w * progress, bar_h, WHITE);

    let label = "Loading...";
    let ml = measure_text(label, None, 14, 1.0);
    draw_text(label, (screen_width() - ml.width) * 0.5, bar_y + 28.0, 14.0, Color::new(1.0, 1.0, 1.0, 0.7));
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            jump: None, coin: None, stomp: None, powerup: None,
            fireball: None, death: None, oneup: None, bump: None,
            bgm: None,
        }
    }

    pub async fn load_with_progress(&mut self) {
        let paths = ASSET_PATHS;
        let total = paths.len();
        let mut sounds: Vec<Option<Sound>> = Vec::new();

        for (i, path) in paths.iter().enumerate() {
            draw_loading_screen(i as f32 / total as f32);
            next_frame().await;
            let sound = audio::load_sound(path).await.ok();
            sounds.push(sound);
        }

        draw_loading_screen(1.0);
        next_frame().await;

        self.jump = sounds[0].take();
        self.coin = sounds[1].take();
        self.stomp = sounds[2].take();
        self.powerup = sounds[3].take();
        self.fireball = sounds[4].take();
        self.death = sounds[5].take();
        self.oneup = sounds[6].take();
        self.bump = sounds[7].take();
        self.bgm = sounds[8].take();
    }

    pub fn play_sfx(&self, sfx: Sfx) {
        let sound = match sfx {
            Sfx::Jump => &self.jump,
            Sfx::Coin => &self.coin,
            Sfx::Stomp => &self.stomp,
            Sfx::PowerUp => &self.powerup,
            Sfx::Fireball => &self.fireball,
            Sfx::Death => &self.death,
            Sfx::OneUp => &self.oneup,
            Sfx::Bump => &self.bump,
        };
        if let Some(s) = sound {
            audio::play_sound(s, PlaySoundParams {
                looped: false,
                volume: 0.5,
            });
        }
    }

    pub fn play_bgm(&self) {
        if let Some(s) = &self.bgm {
            audio::play_sound(s, PlaySoundParams {
                looped: true,
                volume: 0.3,
            });
        }
    }

    pub fn stop_bgm(&self) {
        if let Some(s) = &self.bgm {
            audio::stop_sound(s);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Sfx {
    Jump,
    Coin,
    Stomp,
    PowerUp,
    Fireball,
    Death,
    OneUp,
    Bump,
}
