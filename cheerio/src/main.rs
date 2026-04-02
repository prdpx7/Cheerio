mod constants;

use macroquad::prelude::*;
use constants::*;

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

    loop {
        clear_background(SKYBLUE);

        match state {
            GameState::Title => {
                draw_text("CHEERIO", INTERNAL_WIDTH * 0.5 - 60.0, 100.0, 40.0, WHITE);
                draw_text("Press SPACE to Start", INTERNAL_WIDTH * 0.5 - 90.0, 160.0, 20.0, WHITE);

                if is_key_pressed(KeyCode::Space) {
                    state = GameState::Playing;
                }
            }
            GameState::Playing => {
                draw_text("Playing... (ESC to pause)", 10.0, 30.0, 20.0, WHITE);

                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::Paused;
                }
            }
            GameState::Paused => {
                draw_text("PAUSED (ESC to resume)", INTERNAL_WIDTH * 0.5 - 100.0, 130.0, 24.0, WHITE);

                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::Playing;
                }
            }
            GameState::GameOver => {
                draw_text("GAME OVER", INTERNAL_WIDTH * 0.5 - 80.0, 100.0, 40.0, WHITE);
                draw_text("Press SPACE to Restart", INTERNAL_WIDTH * 0.5 - 90.0, 160.0, 20.0, WHITE);

                if is_key_pressed(KeyCode::Space) {
                    state = GameState::Title;
                }
            }
        }

        next_frame().await;
    }
}
