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
use enemy::{Enemy, EnemyKind};
use collision::is_stomp;
use score::ScoreManager;
use zone::ZoneManager;
use audio::{AudioManager, Sfx};
use screens::GameOverAction;

#[cfg(target_arch = "wasm32")]
mod wasm_js {
    extern "C" {
        pub fn cheerio_open_url(ptr: *const u8, len: usize);
        pub fn cheerio_loading_done();
        pub fn cheerio_share_screenshot(score: u32, platform: u32);
    }
}

fn open_url(url: &str) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        wasm_js::cheerio_open_url(url.as_ptr(), url.len());
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = url;
}

fn loading_done() {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        wasm_js::cheerio_loading_done();
    }
}

fn share_screenshot(score: u32, platform: u32) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        wasm_js::cheerio_share_screenshot(score, platform);
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (score, platform);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameState {
    Title,
    Playing,
    Paused,
    DyingAnimation,
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
    audio.load_with_progress().await;
    loading_done();

    let mut dying_timer: f32 = 0.0;
    let mut game_over_timer: f32 = 0.0;
    let mut frozen_zone = zone::ZoneType::Grassland;
    let mut last_share_time: f64 = -10.0;

    loop {
        let dt = get_frame_time();

        if screen_width() < screen_height() {
            set_default_camera();
            clear_background(Color::new(0.07, 0.07, 0.1, 1.0));

            let t = get_time() as f32;
            let cx = screen_width() * 0.5;
            let cy = screen_height() * 0.42;
            let phase = (t % 4.0) / 4.0;

            let phone_w = 40.0;
            let phone_h = 70.0;

            if phase < 0.5 {
                let p = (phase / 0.5).min(1.0);

                draw_rectangle(cx - phone_w * 0.5, cy - phone_h * 0.5, phone_w, phone_h, Color::new(0.3, 0.3, 0.35, 1.0));
                draw_rectangle(cx - phone_w * 0.5 + 3.0, cy - phone_h * 0.5 + 6.0, phone_w - 6.0, phone_h - 14.0, Color::new(0.15, 0.15, 0.2, 1.0));
                draw_circle(cx, cy + phone_h * 0.5 - 5.0, 3.0, Color::new(0.4, 0.4, 0.45, 1.0));

                let lock_y = cy - phone_h * 0.5 - 20.0;
                let lock_color = if p < 0.6 {
                    WHITE
                } else {
                    Color::new(0.3, 0.9, 0.3, 1.0)
                };

                if p < 0.6 {
                    draw_rectangle(cx - 8.0, lock_y + 10.0, 16.0, 14.0, lock_color);
                    draw_rectangle(cx - 5.0, lock_y, 10.0, 12.0, Color::new(0.07, 0.07, 0.1, 1.0));
                    draw_rectangle(cx - 5.0, lock_y + 2.0, 2.0, 10.0, lock_color);
                    draw_rectangle(cx + 3.0, lock_y + 2.0, 2.0, 10.0, lock_color);
                    draw_rectangle(cx - 5.0, lock_y, 10.0, 2.0, lock_color);
                } else {
                    draw_rectangle(cx - 8.0, lock_y + 10.0, 16.0, 14.0, lock_color);
                    let swing = (p - 0.6) / 0.4 * 0.8;
                    draw_rectangle(cx - 5.0 - swing * 8.0, lock_y, 10.0, 12.0, Color::new(0.07, 0.07, 0.1, 1.0));
                    draw_rectangle(cx - 5.0 - swing * 8.0, lock_y + 2.0, 2.0, 10.0, lock_color);
                    draw_rectangle(cx + 3.0 - swing * 8.0, lock_y + 2.0, 2.0, 10.0, lock_color);
                    draw_rectangle(cx - 5.0 - swing * 8.0, lock_y, 10.0, 2.0, lock_color);
                }

                let step1 = "1. Unlock Portrait Lock";
                let ms1 = measure_text(step1, None, 16, 1.0);
                draw_text(step1, (screen_width() - ms1.width) * 0.5, cy + phone_h * 0.5 + 30.0, 16.0, WHITE);
            } else {
                let p = ((phase - 0.5) / 0.5).min(1.0);
                let angle = p * std::f32::consts::FRAC_PI_2;

                let cos_a = angle.cos();
                let sin_a = angle.sin();

                let hw = phone_w * 0.5;
                let hh = phone_h * 0.5;

                let corners = [
                    (-hw, -hh), (hw, -hh), (hw, hh), (-hw, hh)
                ];
                let rotated: Vec<(f32, f32)> = corners.iter().map(|(x, y)| {
                    (cx + x * cos_a - y * sin_a, cy + x * sin_a + y * cos_a)
                }).collect();

                for i in 0..4 {
                    let j = (i + 1) % 4;
                    draw_line(rotated[i].0, rotated[i].1, rotated[j].0, rotated[j].1, 2.0, Color::new(0.3, 0.3, 0.35, 1.0));
                }

                let inner_margin = 4.0;
                let iw = hw - inner_margin;
                let ih = hh - inner_margin * 1.5;
                let ic: Vec<(f32, f32)> = [(-iw, -ih), (iw, -ih), (iw, ih), (-iw, ih)].iter().map(|(x, y)| {
                    (cx + x * cos_a - y * sin_a, cy + x * sin_a + y * cos_a)
                }).collect();

                draw_triangle(
                    vec2(ic[0].0, ic[0].1), vec2(ic[1].0, ic[1].1), vec2(ic[2].0, ic[2].1),
                    Color::new(0.15, 0.15, 0.2, 1.0),
                );
                draw_triangle(
                    vec2(ic[0].0, ic[0].1), vec2(ic[2].0, ic[2].1), vec2(ic[3].0, ic[3].1),
                    Color::new(0.15, 0.15, 0.2, 1.0),
                );

                let step2 = "2. Rotate Your Phone";
                let ms2 = measure_text(step2, None, 16, 1.0);
                draw_text(step2, (screen_width() - ms2.width) * 0.5, cy + phone_h * 0.5 + 50.0, 16.0, WHITE);
            }

            let dots = match ((t * 2.0) as u32) % 4 {
                0 => "",
                1 => ".",
                2 => "..",
                _ => "...",
            };
            let hint = format!("Best played in landscape{}", dots);
            let mh = measure_text(&hint, None, 14, 1.0);
            draw_text(&hint, (screen_width() - mh.width) * 0.5, screen_height() * 0.75, 14.0, Color::new(1.0, 1.0, 1.0, 0.5));

            next_frame().await;
            continue;
        }

        camera.begin_render();

        match state {
            GameState::Title => {
                let high_score = score.as_ref().map(|s| s.high_score).unwrap_or(0);
                if screens::draw_title_screen(high_score, camera.scroll_x) {
                    player = Some(Player::new(camera.scroll_x));
                    world = Some(World::new());
                    score = Some(ScoreManager::new());
                    zone_manager = Some(ZoneManager::new());
                    audio.play_bgm();
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

                    if p.jumped {
                        audio.play_sfx(Sfx::Jump);
                        p.jumped = false;
                    }
                    if p.fired {
                        audio.play_sfx(Sfx::Fireball);
                        p.fired = false;
                    }

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
                                        audio.play_sfx(Sfx::Death);
                                        frozen_zone = current_zone;
                                        dying_timer = 0.0;
                                        state = GameState::DyingAnimation;
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
                        audio.play_sfx(Sfx::Stomp);
                    }

                    let shell_rects: Vec<Rect> = world.as_ref().unwrap()
                        .chunks.iter()
                        .flat_map(|c| c.enemies.iter())
                        .filter(|e| e.alive && e.kind == EnemyKind::Shell && e.spawn_timer <= 0.0)
                        .map(|e| e.rect())
                        .collect();

                    let mut killing_shell_rects: Vec<Rect> = Vec::new();
                    let mut shell_kills = 0u32;
                    for enemy in world.as_mut().unwrap().get_all_enemies_mut() {
                        if !enemy.alive || enemy.kind == EnemyKind::Shell { continue; }
                        for &shell_rect in &shell_rects {
                            if enemy.rect().overlaps(&shell_rect) {
                                enemy.alive = false;
                                enemy.death_timer = 0.5;
                                enemy.death_vy = -150.0;
                                killing_shell_rects.push(shell_rect);
                                shell_kills += 1;
                                break;
                            }
                        }
                    }
                    if !killing_shell_rects.is_empty() {
                        for enemy in world.as_mut().unwrap().get_all_enemies_mut() {
                            if enemy.kind != EnemyKind::Shell || !enemy.alive { continue; }
                            let er = enemy.rect();
                            if killing_shell_rects.iter().any(|kr| (er.x - kr.x).abs() < 2.0 && (er.y - kr.y).abs() < 2.0) {
                                enemy.alive = false;
                                enemy.death_timer = 0.3;
                                enemy.death_vy = -80.0;
                            }
                        }
                    }
                    for _ in 0..shell_kills {
                        p.stomp_chain += 1;
                        score.as_mut().unwrap().add_stomp(p.stomp_chain as usize - 1);
                        audio.play_sfx(Sfx::Stomp);
                    }

                    let ground_rects_for_fb = world.as_ref().unwrap().get_ground_rects();
                    for fb in &mut p.fireballs {
                        fb.update(dt, &ground_rects_for_fb);
                    }
                    for fb in &mut p.fireballs {
                        if !fb.alive { continue; }
                        for enemy in world.as_mut().unwrap().get_all_enemies_mut() {
                            if enemy.alive && fb.rect().overlaps(&enemy.rect()) {
                                if enemy.kind != EnemyKind::BuzzyBeetle {
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
                        c.update(dt, &ground_rects_for_collect, scroll_speed);
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
                        audio.play_sfx(Sfx::Coin);
                    }
                    for _ in 0..powerups_collected {
                        score.as_mut().unwrap().add_powerup();
                        audio.play_sfx(Sfx::PowerUp);
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
                                        audio.play_sfx(Sfx::Bump);
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
                                audio.play_sfx(Sfx::Death);
                                frozen_zone = current_zone;
                                dying_timer = 0.0;
                                state = GameState::DyingAnimation;
                                score.as_mut().unwrap().finalize();
                            }
                        }
                    }

                    for t in world.as_mut().unwrap().get_all_thwomps_mut() {
                        t.update(dt, p.x);
                        if p.rect().overlaps(&t.rect()) {
                            p.take_damage();
                            if p.is_dead {
                                audio.play_sfx(Sfx::Death);
                                frozen_zone = current_zone;
                                dying_timer = 0.0;
                                state = GameState::DyingAnimation;
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
                        audio.play_sfx(Sfx::Death);
                        frozen_zone = current_zone;
                        dying_timer = 0.0;
                        state = GameState::DyingAnimation;
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
            GameState::DyingAnimation => {
                dying_timer += dt;

                clear_background(frozen_zone.bg_color());
                renderer::draw_parallax_background(frozen_zone, camera.scroll_x);
                world.as_ref().unwrap().draw();

                if let Some(ref mut p) = player {
                    p.update(dt, 0.0);
                    p.draw();
                }

                let vignette_alpha = (dying_timer / 1.5).min(0.7);
                draw_rectangle(camera.scroll_x, 0.0, INTERNAL_WIDTH, INTERNAL_HEIGHT, Color::new(0.0, 0.0, 0.0, vignette_alpha));

                let player_fell = player.as_ref().map(|p| p.y > INTERNAL_HEIGHT + 60.0).unwrap_or(true);
                if dying_timer > 2.0 || player_fell {
                    audio.stop_bgm();
                    game_over_timer = 0.0;
                    state = GameState::GameOver;
                }
            }
            GameState::GameOver => {
                game_over_timer += dt;

                clear_background(frozen_zone.bg_color());
                renderer::draw_parallax_background(frozen_zone, camera.scroll_x);
                world.as_ref().unwrap().draw();

                match screens::draw_game_over_screen(score.as_ref().unwrap(), camera.scroll_x, game_over_timer) {
                    GameOverAction::Restart => {
                        state = GameState::Title;
                        camera = GameCamera::new();
                        player = None;
                        world = None;
                        score = None;
                        zone_manager = None;
                    }
                    GameOverAction::ShareTwitter => {
                        if get_time() - last_share_time > 1.0 {
                            let s = score.as_ref().unwrap().score;
                            share_screenshot(s, 1);
                            last_share_time = get_time();
                        }
                    }
                    GameOverAction::ShareWhatsApp => {
                        if get_time() - last_share_time > 1.0 {
                            let s = score.as_ref().unwrap().score;
                            share_screenshot(s, 2);
                            last_share_time = get_time();
                        }
                    }
                    GameOverAction::None => {}
                }
            }
        }

        camera.end_render();
        next_frame().await;
    }
}
