mod constants;
mod camera;
mod player;
mod collision;
mod world;

use macroquad::prelude::*;
use constants::*;
use camera::GameCamera;
use player::Player;
use world::World;

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
                    state = GameState::Playing;
                }
            }
            GameState::Playing => {
                camera.advance(SCROLL_SPEED_BASE, dt);
                draw_text("Playing... (ESC to pause)", 10.0 + camera.scroll_x, 30.0, 20.0, WHITE);

                world.as_mut().unwrap().update(camera.scroll_x);
                world.as_ref().unwrap().draw();

                if let Some(ref mut p) = player {
                    p.update(dt, SCROLL_SPEED_BASE);

                    let ground_rects = world.as_ref().unwrap().get_ground_rects();
                    let platform_rects = world.as_ref().unwrap().get_platform_rects();
                    p.resolve_terrain(&ground_rects, &platform_rects);

                    p.draw();

                    if p.y > INTERNAL_HEIGHT + 50.0 {
                        state = GameState::GameOver;
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
                }
            }
        }

        camera.end_render();
        next_frame().await;
    }
}
