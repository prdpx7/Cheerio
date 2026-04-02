use macroquad::audio::{self, Sound, PlaySoundParams};

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

impl AudioManager {
    pub fn new() -> Self {
        Self {
            jump: None, coin: None, stomp: None, powerup: None,
            fireball: None, death: None, oneup: None, bump: None,
            bgm: None,
        }
    }

    pub async fn load(&mut self) {
        self.jump = audio::load_sound("assets/audio/smb_jump-small.wav").await.ok();
        self.coin = audio::load_sound("assets/audio/smb_coin.wav").await.ok();
        self.stomp = audio::load_sound("assets/audio/smb_stomp.wav").await.ok();
        self.powerup = audio::load_sound("assets/audio/smb_powerup.wav").await.ok();
        self.fireball = audio::load_sound("assets/audio/smb_fireball.wav").await.ok();
        self.death = audio::load_sound("assets/audio/smb_mariodie.wav").await.ok();
        self.oneup = audio::load_sound("assets/audio/smb_1-up.wav").await.ok();
        self.bump = audio::load_sound("assets/audio/smb_bump.wav").await.ok();
        self.bgm = audio::load_sound("assets/audio/bgm_main.ogg").await.ok();
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
