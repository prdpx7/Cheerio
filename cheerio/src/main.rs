mod constants;
mod camera;
mod player;
mod collision;
mod world;
mod enemy;
mod collectible;
mod score;

use macroquad::prelude::*;
use constants::*;
use camera::GameCamera;
use player::{Player, PowerState};
use world::World;
use enemy::Enemy;
use collision::is_stomp;
use score::ScoreManager;

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameState {
    Title,
    Playing,
    Paused,
    GameOver,
}

fn conf() -> Conf {
    Conf {
        window_title: "Cheerio".to_string(),
        window_width: 960,
        window_height: 540,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut state = GameState::Title;
    let mut camera = GameCamera::new();
    let mut player: Option<Player> = None;
    let mut world: Option<World> = None;
    let mut score: Option<ScoreManager> = None;

    loop {
        let dt = get_frame_time();

        camera.begin_render();
        clear_background(SKYBLUE);

        match state {
            GameState::Title => {
                draw_text("CHEERIO", INTERNAL_WIDTH * 0.5 - 60.0, 100.0, 40.0, WHITE);
                draw_text("Press SPACE to Start", INTERNAL_WIDTH * 0.5 - 90.0, 160.0, 20.0, WHITE);

                if is_key_pressed(KeyCode::Space) {
                    player = Some(Player::new(camera.scroll_x));
                    world = Some(World::new());
                    score = Some(ScoreManager::new());
                    state = GameState::Playing;
                }
            }
            GameState::Playing => {
                camera.advance(SCROLL_SPEED_BASE, dt);
                score.as_mut().unwrap().add_distance(SCROLL_SPEED_BASE * dt);

                world.as_mut().unwrap().update(camera.scroll_x);
                world.as_ref().unwrap().draw();

                if let Some(ref mut p) = player {
                    p.update(dt, SCROLL_SPEED_BASE);

                    let ground_rects = world.as_ref().unwrap().get_ground_rects();
                    let platform_rects = world.as_ref().unwrap().get_platform_rects();
                    p.resolve_terrain(&ground_rects, &platform_rects);

                    let ground_rects_for_enemies = world.as_ref().unwrap().get_ground_rects();
                    let mut new_shells: Vec<Enemy> = Vec::new();
                    let mut stomps_this_frame: Vec<usize> = Vec::new();
                    for enemy in world.as_mut().unwrap().get_all_enemies_mut() {
                        enemy.update(dt, &ground_rects_for_enemies);
                        if enemy.alive {
                            let player_rect = p.rect();
                            let enemy_rect = enemy.rect();
                            if player_rect.overlaps(&enemy_rect) {
                                if is_stomp(&player_rect, &enemy_rect, p.vy) {
                                    if let Some(shell) = enemy.stomp() {
                                        new_shells.push(shell);
                                    }
                                    p.vy = STOMP_BOUNCE_VELOCITY;
                                    p.on_ground = false;
                                    p.stomp_chain += 1;
                                    stomps_this_frame.push(p.stomp_chain as usize - 1);
                                } else {
                                    p.take_damage();
                                    if p.is_dead {
                                        state = GameState::GameOver;
                                        score.as_mut().unwrap().finalize();
                                    }
                                }
                            }
                        }
                    }
                    for shell in new_shells {
                        if let Some(chunk) = world.as_mut().unwrap().chunks.last_mut() {
                            chunk.enemies.push(shell);
                        }
                    }
                    for chain_idx in stomps_this_frame {
                        score.as_mut().unwrap().add_stomp(chain_idx);
                    }

                    let ground_rects_for_collect = world.as_ref().unwrap().get_ground_rects();
                    let mut coins_collected = 0u32;
                    let mut powerups_collected = 0u32;
                    for c in world.as_mut().unwrap().get_all_collectibles_mut() {
                        c.update(dt, &ground_rects_for_collect);
                        if !c.collected && c.active && p.rect().overlaps(&c.rect()) {
                            c.collected = true;
                            match c.kind {
                                collectible::CollectibleKind::Coin => {
                                    coins_collected += 1;
                                }
                                collectible::CollectibleKind::Mushroom => {
                                    if p.power_state == PowerState::Small {
                                        p.power_state = PowerState::Super;
                                    }
                                    powerups_collected += 1;
                                }
                                collectible::CollectibleKind::FireFlower => {
                                    if p.power_state == PowerState::Super {
                                        p.power_state = PowerState::Fire;
                                    } else if p.power_state == PowerState::Small {
                                        p.power_state = PowerState::Super;
                                    }
                                    powerups_collected += 1;
                                }
                                collectible::CollectibleKind::Star => {
                                    p.star_timer = STAR_DURATION;
                                    powerups_collected += 1;
                                }
                            }
                        }
                    }
                    for _ in 0..coins_collected {
                        score.as_mut().unwrap().add_coin();
                    }
                    for _ in 0..powerups_collected {
                        score.as_mut().unwrap().add_powerup();
                    }

                    let mut spawned_collectibles = Vec::new();
                    for qb in world.as_mut().unwrap().get_all_question_blocks_mut() {
                        qb.update(dt);
                        if !qb.hit {
                            let player_rect = p.rect();
                            let qb_rect = qb.rect();
                            if player_rect.overlaps(&qb_rect) && p.vy < 0.0 {
                                let player_top = player_rect.y;
                                let qb_bottom = qb_rect.y + qb_rect.h;
                                if (player_top - qb_bottom).abs() < 8.0 {
                                    if let Some(item) = qb.hit_block() {
                                        spawned_collectibles.push(item);
                                    }
                                }
                            }
                        }
                    }
                    for item in spawned_collectibles {
                        world.as_mut().unwrap().add_collectible_to_nearest_chunk(item);
                    }

                    p.draw();
                    score.as_ref().unwrap().draw_hud(camera.scroll_x, "GRASSLAND");

                    if p.y > INTERNAL_HEIGHT + 50.0 {
                        state = GameState::GameOver;
                        score.as_mut().unwrap().finalize();
                    }
                }

                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::Paused;
                }
            }
            GameState::Paused => {
                draw_text("PAUSED (ESC to resume)", INTERNAL_WIDTH * 0.5 - 100.0 + camera.scroll_x, 130.0, 24.0, WHITE);

                world.as_ref().unwrap().draw();

                if let Some(ref p) = player {
                    p.draw();
                }

                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::Playing;
                }
            }
            GameState::GameOver => {
                draw_text("GAME OVER", INTERNAL_WIDTH * 0.5 - 80.0 + camera.scroll_x, 100.0, 40.0, WHITE);
                draw_text("Press SPACE to Restart", INTERNAL_WIDTH * 0.5 - 90.0 + camera.scroll_x, 160.0, 20.0, WHITE);

                if is_key_pressed(KeyCode::Space) {
                    state = GameState::Title;
                    camera = GameCamera::new();
                    player = None;
                    world = None;
                    score = None;
                }
            }
        }

        camera.end_render();
        next_frame().await;
    }
}
