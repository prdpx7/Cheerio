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

    let ver = concat!("v", env!("CARGO_PKG_VERSION"));
    let mv = measure_text(ver, None, 12, 1.0);
    draw_text(ver, camera_x + INTERNAL_WIDTH - mv.width - 4.0, INTERNAL_HEIGHT - 4.0, 12.0, Color::new(1.0, 1.0, 1.0, 0.4));

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameOverAction {
    None,
    Restart,
    ShareTwitter,
    ShareWhatsApp,
}

pub fn draw_game_over_screen(score: &ScoreManager, camera_x: f32, timer: f32) -> GameOverAction {
    let overlay_alpha = (timer / 0.5).min(1.0) * 0.85;
    draw_rectangle(camera_x, 0.0, INTERNAL_WIDTH, INTERNAL_HEIGHT, Color::new(0.05, 0.05, 0.1, overlay_alpha));

    if timer < 0.4 {
        return GameOverAction::None;
    }

    let title_alpha = ((timer - 0.4) / 0.4).min(1.0);
    let title_y_offset = (1.0 - title_alpha) * -20.0;

    let title = "GAME OVER";
    draw_text(
        title,
        center_x(title, 40.0, camera_x),
        55.0 + title_y_offset,
        40.0,
        Color::new(1.0, 0.25, 0.25, title_alpha),
    );

    if timer < 1.0 {
        return GameOverAction::None;
    }

    let stats_alpha = ((timer - 1.0) / 0.5).min(1.0);

    let sc = format!("Score: {}", score.score);
    draw_text(&sc, center_x(&sc, 24.0, camera_x), 95.0, 24.0, Color::new(1.0, 1.0, 1.0, stats_alpha));

    if timer > 1.2 {
        let a2 = ((timer - 1.2) / 0.4).min(1.0);
        let dist = format!("Distance: {:.0}m", score.distance);
        draw_text(&dist, center_x(&dist, 15.0, camera_x), 122.0, 15.0, Color::new(0.8, 0.8, 0.9, a2));
    }
    if timer > 1.4 {
        let a3 = ((timer - 1.4) / 0.4).min(1.0);
        let coins = format!("Coins: {}", score.coins);
        draw_text(&coins, center_x(&coins, 15.0, camera_x), 140.0, 15.0, Color::new(1.0, 0.85, 0.2, a3));
    }
    if timer > 1.6 {
        let a4 = ((timer - 1.6) / 0.4).min(1.0);
        let enemies = format!("Stomped: {}", score.enemies_stomped);
        draw_text(&enemies, center_x(&enemies, 15.0, camera_x), 158.0, 15.0, Color::new(0.8, 0.9, 0.8, a4));
    }

    if timer > 1.8 && score.is_new_high_score() {
        let a5 = ((timer - 1.8) / 0.4).min(1.0);
        let blink_mod = (get_time() * 3.0).sin() > 0.0;
        if blink_mod {
            let hi = "NEW HIGH SCORE!";
            draw_text(hi, center_x(hi, 18.0, camera_x), 180.0, 18.0, Color::new(1.0, 0.85, 0.0, a5));
        }
    }

    if timer < 2.5 {
        return GameOverAction::None;
    }

    let btn_alpha = ((timer - 2.5) / 0.4).min(1.0);
    let ready = btn_alpha >= 1.0;

    let blink = ready && (get_time() * 2.0).sin() > 0.0;
    if blink {
        let prompt = "TAP OR PRESS SPACE TO RESTART";
        draw_text(prompt, center_x(prompt, 13.0, camera_x), 200.0, 13.0, Color::new(1.0, 1.0, 1.0, 0.9));
    }

    let twitter_rect = Rect::new(camera_x + INTERNAL_WIDTH * 0.5 - 110.0, 214.0, 100.0, 22.0);
    let whatsapp_rect = Rect::new(camera_x + INTERNAL_WIDTH * 0.5 + 10.0, 214.0, 100.0, 22.0);

    let tw_col = Color::new(0.11, 0.63, 0.95, btn_alpha);
    let wa_col = Color::new(0.07, 0.73, 0.42, btn_alpha);
    draw_rectangle(twitter_rect.x, twitter_rect.y, twitter_rect.w, twitter_rect.h, tw_col);
    draw_rectangle(whatsapp_rect.x, whatsapp_rect.y, whatsapp_rect.w, whatsapp_rect.h, wa_col);

    let tw_lbl = "Share Twitter";
    let wa_lbl = "Share WhatsApp";
    draw_text(tw_lbl, twitter_rect.x + (twitter_rect.w - measure_text(tw_lbl, None, 11, 1.0).width) * 0.5, twitter_rect.y + 14.0, 11.0, WHITE);
    draw_text(wa_lbl, whatsapp_rect.x + (whatsapp_rect.w - measure_text(wa_lbl, None, 11, 1.0).width) * 0.5, whatsapp_rect.y + 14.0, 11.0, WHITE);

    if !ready {
        return GameOverAction::None;
    }

    let tapped = touches().iter().any(|t| t.phase == TouchPhase::Started);
    let mouse_down = is_mouse_button_pressed(MouseButton::Left);

    let (mx, my) = mouse_position();
    let mouse_in_canvas = Rect::new(camera_x, 0.0, INTERNAL_WIDTH, INTERNAL_HEIGHT);

    if mouse_down && mouse_in_canvas.contains(vec2(mx, my)) {
        let mp = vec2(mx, my);
        if twitter_rect.contains(mp) {
            return GameOverAction::ShareTwitter;
        }
        if whatsapp_rect.contains(mp) {
            return GameOverAction::ShareWhatsApp;
        }
    }

    for touch in touches() {
        if touch.phase == TouchPhase::Started {
            let tp = touch.position;
            let scale_x = INTERNAL_WIDTH / screen_width();
            let scale_y = INTERNAL_HEIGHT / screen_height();
            let tp_game = vec2(camera_x + tp.x * scale_x, tp.y * scale_y);

            if twitter_rect.contains(tp_game) {
                return GameOverAction::ShareTwitter;
            }
            if whatsapp_rect.contains(tp_game) {
                return GameOverAction::ShareWhatsApp;
            }
        }
    }

    if is_key_pressed(KeyCode::Space) {
        return GameOverAction::Restart;
    }
    if mouse_down {
        return GameOverAction::Restart;
    }
    if tapped {
        return GameOverAction::Restart;
    }

    GameOverAction::None
}
