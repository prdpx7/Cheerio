pub const INTERNAL_WIDTH: f32 = 480.0;
pub const INTERNAL_HEIGHT: f32 = 270.0;

pub const GRAVITY: f32 = 1200.0;
pub const JUMP_VELOCITY: f32 = -460.0;
pub const JUMP_CUT_VELOCITY: f32 = -220.0;
pub const PLAYER_SPEED: f32 = 120.0;

pub const TILE_SIZE: f32 = 16.0;
pub const GROUND_Y: f32 = INTERNAL_HEIGHT - TILE_SIZE * 2.0;

pub const PLAYER_START_X_RATIO: f32 = 0.2;
pub const PLAYER_WIDTH: f32 = 14.0;
pub const PLAYER_HEIGHT_SMALL: f32 = 16.0;
pub const PLAYER_HEIGHT_SUPER: f32 = 32.0;

pub const CHUNK_WIDTH: f32 = INTERNAL_WIDTH;
pub const CHUNK_BUFFER: usize = 4;

pub const SCROLL_SPEED_BASE: f32 = 120.0;
pub const SCROLL_SPEED_MULTIPLIERS: [f32; 4] = [1.0, 1.15, 1.3, 1.4];

pub const ZONE_DURATION: f32 = 30.0;

pub const COIN_SCORE: u32 = 100;
pub const STOMP_BASE_SCORE: u32 = 200;
pub const POWERUP_SCORE: u32 = 500;
pub const STOMP_CHAIN: [u32; 5] = [200, 400, 800, 1600, 8000];

pub const STAR_DURATION: f32 = 10.0;

pub const ENEMY_GOOMBA_SPEED: f32 = 40.0;
pub const ENEMY_KOOPA_SPEED: f32 = 35.0;
pub const SHELL_SPEED: f32 = 200.0;

pub const STOMP_BOUNCE_VELOCITY: f32 = -200.0;
