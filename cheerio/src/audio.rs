use crate::zone::ZoneType;

pub struct AudioManager {
    pub sfx_enabled: bool,
    pub bgm_enabled: bool,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            sfx_enabled: true,
            bgm_enabled: true,
        }
    }

    pub fn play_sfx(&self, _sfx: Sfx) {
    }

    pub fn play_zone_bgm(&mut self, _zone: ZoneType) {
    }

    pub fn play_star_bgm(&mut self) {
    }

    pub fn stop_bgm(&mut self) {
    }
}

pub enum Sfx {
    Jump,
    Coin,
    Stomp,
    PowerUp,
    Fireball,
    Death,
    OneUp,
}
