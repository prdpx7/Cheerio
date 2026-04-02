mod constants;
mod camera;
mod player;
mod collision;
mod world;
mod enemy;
mod collectible;
mod score;
mod zone;
mod screens;
mod renderer;
mod audio;

use macroquad::prelude::*;
use constants::*;
use camera::GameCamera;
use player::{Player, PowerState};
use world::World;
use enemy::Enemy;
use collision::is_stomp;
use score::ScoreManager;
use zone::ZoneManager;
use audio::AudioManager;

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
    let mut zone_manager: Option<ZoneManager> = None;
    let mut audio = AudioManager::new();

    loop {
        let dt = get_frame_time();

        camera.begin_render();
        clear_background(SKYBLUE);

        match state {
            GameState::Title => {
                let high_score = score.as_ref().map(|s| s.high_score).unwrap_or(0);
                if screens::draw_title_screen(high_score) {
                    player = Some(Player::new(camera.scroll_x));
                    world = Some(World::new());
                    score = Some(ScoreManager::new());
                    zone_manager = Some(ZoneManager::new());
                    audio.play_zone_bgm(zone::ZoneType::Grassland);
                    state = GameState::Playing;
                }
            }
            GameState::Playing => {
                zone_manager.as_mut().unwrap().update(dt);

                let scroll_speed = zone_manager.as_ref().unwrap().scroll_speed();
                let current_zone = zone_manager.as_ref().unwrap().current;
                let zone_cycle = zone_manager.as_ref().unwrap().cycle;

                clear_background(current_zone.bg_color());
                renderer::draw_parallax_background(current_zone, camera.scroll_x);

                camera.advance(scroll_speed, dt);
                score.as_mut().unwrap().add_distance(scroll_speed * dt);
                score.as_mut().unwrap().cycle = zone_cycle + 1;

                world.as_mut().unwrap().update(camera.scroll_x, current_zone, zone_cycle);
                world.as_ref().unwrap().draw();

                if let Some(ref mut p) = player {
                    p.update(dt, scroll_speed);

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

                    let ground_rects_for_fb = world.as_ref().unwrap().get_ground_rects();
                    for fb in &mut p.fireballs {
                        fb.update(dt, &ground_rects_for_fb);
                    }
                    for fb in &mut p.fireballs {
                        if !fb.alive { continue; }
                        for enemy in world.as_mut().unwrap().get_all_enemies_mut() {
                            if enemy.alive && fb.rect().overlaps(&enemy.rect()) {
                                if enemy.kind != crate::enemy::EnemyKind::BuzzyBeetle {
                                    enemy.alive = false;
                                    enemy.death_timer = 0.3;
                                }
                                fb.alive = false;
                                break;
                            }
                        }
                    }
                    p.fireballs.retain(|fb| fb.alive && fb.x < camera.scroll_x + INTERNAL_WIDTH + 50.0);

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

                    for fb in world.as_mut().unwrap().get_all_fire_bars_mut() {
                        fb.update(dt);
                        if fb.hits_player(&p.rect()) {
                            p.take_damage();
                            if p.is_dead {
                                state = GameState::GameOver;
                                score.as_mut().unwrap().finalize();
                            }
                        }
                    }

                    for t in world.as_mut().unwrap().get_all_thwomps_mut() {
                        t.update(dt, p.x);
                        if p.rect().overlaps(&t.rect()) {
                            p.take_damage();
                            if p.is_dead {
                                state = GameState::GameOver;
                                score.as_mut().unwrap().finalize();
                            }
                        }
                    }

                    p.draw();
                    for fb in &p.fireballs {
                        fb.draw();
                    }
                    score.as_ref().unwrap().draw_hud(camera.scroll_x, current_zone.name());
                    zone_manager.as_ref().unwrap().draw_transition(camera.scroll_x);

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
                let current_zone = zone_manager.as_ref().unwrap().current;
                clear_background(current_zone.bg_color());
                renderer::draw_parallax_background(current_zone, camera.scroll_x);
                world.as_ref().unwrap().draw();

                if let Some(ref p) = player {
                    p.draw();
                }

                score.as_ref().unwrap().draw_hud(camera.scroll_x, current_zone.name());

                if screens::draw_pause_screen(camera.scroll_x) {
                    state = GameState::Playing;
                }
            }
            GameState::GameOver => {
                if screens::draw_game_over_screen(score.as_ref().unwrap()) {
                    state = GameState::Title;
                    camera = GameCamera::new();
                    player = None;
                    world = None;
                    score = None;
                    zone_manager = None;
                    audio.stop_bgm();
                }
            }
        }

        camera.end_render();
        next_frame().await;
    }
}
