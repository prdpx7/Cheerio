use macroquad::prelude::*;
use crate::constants::*;
use crate::score::ScoreManager;

pub fn draw_title_screen(high_score: u32, camera_x: f32) -> bool {
    clear_background(Color::new(0.4, 0.7, 1.0, 1.0));

    let cx = camera_x;

    draw_text("CHEERIO", cx + INTERNAL_WIDTH * 0.5 - 80.0, 80.0, 48.0, WHITE);
    draw_text("An Endless Adventure", cx + INTERNAL_WIDTH * 0.5 - 90.0, 110.0, 16.0, Color::new(1.0, 1.0, 1.0, 0.8));

    let blink = (get_time() * 2.0).sin() > 0.0;
    if blink {
        draw_text("PRESS SPACE TO START", cx + INTERNAL_WIDTH * 0.5 - 90.0, 180.0, 18.0, WHITE);
    }

    draw_text(&format!("HI-SCORE: {}", high_score), cx + INTERNAL_WIDTH * 0.5 - 60.0, 30.0, 16.0, GOLD);

    is_key_pressed(KeyCode::Space)
}

pub fn draw_pause_screen(camera_x: f32) -> bool {
    draw_rectangle(
        camera_x,
        0.0,
        INTERNAL_WIDTH,
        INTERNAL_HEIGHT,
        Color::new(0.0, 0.0, 0.0, 0.6),
    );
    draw_text(
        "PAUSED",
        camera_x + INTERNAL_WIDTH * 0.5 - 50.0,
        INTERNAL_HEIGHT * 0.5,
        36.0,
        WHITE,
    );
    draw_text(
        "Press ESC to Resume",
        camera_x + INTERNAL_WIDTH * 0.5 - 80.0,
        INTERNAL_HEIGHT * 0.5 + 30.0,
        16.0,
        Color::new(1.0, 1.0, 1.0, 0.7),
    );

    is_key_pressed(KeyCode::Escape)
}

pub fn draw_game_over_screen(score: &ScoreManager, camera_x: f32) -> bool {
    clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

    let cx = camera_x;

    draw_text("GAME OVER", cx + INTERNAL_WIDTH * 0.5 - 80.0, 60.0, 40.0, WHITE);
    draw_text(&format!("Score: {}", score.score), cx + INTERNAL_WIDTH * 0.5 - 60.0, 100.0, 24.0, WHITE);

    draw_text(&format!("Distance: {:.0}", score.distance), cx + 100.0, 140.0, 16.0, LIGHTGRAY);
    draw_text(&format!("Coins: {}", score.coins), cx + 100.0, 160.0, 16.0, GOLD);
    draw_text(&format!("Enemies: {}", score.enemies_stomped), cx + 100.0, 180.0, 16.0, LIGHTGRAY);

    if score.is_new_high_score() {
        let blink = (get_time() * 3.0).sin() > 0.0;
        if blink {
            draw_text("NEW HIGH SCORE!", cx + INTERNAL_WIDTH * 0.5 - 70.0, 210.0, 20.0, GOLD);
        }
    }

    let blink = (get_time() * 2.0).sin() > 0.0;
    if blink {
        draw_text("PRESS SPACE TO PLAY AGAIN", cx + INTERNAL_WIDTH * 0.5 - 110.0, 245.0, 16.0, WHITE);
    }

    is_key_pressed(KeyCode::Space)
}
