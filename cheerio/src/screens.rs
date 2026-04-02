use macroquad::prelude::*;
use crate::constants::*;
use crate::score::ScoreManager;

fn center_x(text: &str, font_size: f32, camera_x: f32) -> f32 {
    let m = measure_text(text, None, font_size as u16, 1.0);
    camera_x + (INTERNAL_WIDTH - m.width) * 0.5
}

pub fn draw_title_screen(high_score: u32, camera_x: f32) -> bool {
    clear_background(Color::new(0.4, 0.7, 1.0, 1.0));

    let title = "CHEERIO";
    draw_text(title, center_x(title, 48.0, camera_x), 80.0, 48.0, WHITE);

    let sub = "An Endless Adventure";
    draw_text(sub, center_x(sub, 16.0, camera_x), 110.0, 16.0, Color::new(1.0, 1.0, 1.0, 0.8));

    let blink = (get_time() * 2.0).sin() > 0.0;
    if blink {
        let prompt = "TAP OR PRESS SPACE";
        draw_text(prompt, center_x(prompt, 18.0, camera_x), 180.0, 18.0, WHITE);
    }

    let hi = format!("HI-SCORE: {}", high_score);
    draw_text(&hi, center_x(&hi, 16.0, camera_x), 30.0, 16.0, GOLD);

    let tapped = touches().iter().any(|t| t.phase == TouchPhase::Started);
    is_key_pressed(KeyCode::Space) || tapped || is_mouse_button_pressed(MouseButton::Left)
}

pub fn draw_pause_screen(camera_x: f32) -> bool {
    draw_rectangle(
        camera_x, 0.0, INTERNAL_WIDTH, INTERNAL_HEIGHT,
        Color::new(0.0, 0.0, 0.0, 0.6),
    );

    let title = "PAUSED";
    draw_text(title, center_x(title, 36.0, camera_x), INTERNAL_HEIGHT * 0.5, 36.0, WHITE);

    let sub = "TAP or ESC to Resume";
    draw_text(sub, center_x(sub, 16.0, camera_x), INTERNAL_HEIGHT * 0.5 + 30.0, 16.0, Color::new(1.0, 1.0, 1.0, 0.7));

    let tapped = touches().iter().any(|t| t.phase == TouchPhase::Started);
    is_key_pressed(KeyCode::Escape) || tapped || is_mouse_button_pressed(MouseButton::Left)
}

pub fn draw_game_over_screen(score: &ScoreManager, camera_x: f32) -> bool {
    clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

    let title = "GAME OVER";
    draw_text(title, center_x(title, 40.0, camera_x), 60.0, 40.0, WHITE);

    let sc = format!("Score: {}", score.score);
    draw_text(&sc, center_x(&sc, 24.0, camera_x), 100.0, 24.0, WHITE);

    let dist = format!("Distance: {:.0}", score.distance);
    let coins = format!("Coins: {}", score.coins);
    let enemies = format!("Enemies: {}", score.enemies_stomped);
    let stats_x = camera_x + INTERNAL_WIDTH * 0.3;
    draw_text(&dist, stats_x, 140.0, 16.0, LIGHTGRAY);
    draw_text(&coins, stats_x, 160.0, 16.0, GOLD);
    draw_text(&enemies, stats_x, 180.0, 16.0, LIGHTGRAY);

    if score.is_new_high_score() {
        let blink = (get_time() * 3.0).sin() > 0.0;
        if blink {
            let hi = "NEW HIGH SCORE!";
            draw_text(hi, center_x(hi, 20.0, camera_x), 210.0, 20.0, GOLD);
        }
    }

    let blink = (get_time() * 2.0).sin() > 0.0;
    if blink {
        let prompt = "TAP OR PRESS SPACE";
        draw_text(prompt, center_x(prompt, 16.0, camera_x), 245.0, 16.0, WHITE);
    }

    let tapped = touches().iter().any(|t| t.phase == TouchPhase::Started);
    is_key_pressed(KeyCode::Space) || tapped || is_mouse_button_pressed(MouseButton::Left)
}
