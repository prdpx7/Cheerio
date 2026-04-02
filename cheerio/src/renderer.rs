use macroquad::prelude::*;
use crate::constants::*;
use crate::zone::ZoneType;

pub struct ParallaxLayer {
    pub speed_ratio: f32,
    pub y: f32,
    pub height: f32,
    pub color: Color,
}

pub fn draw_parallax_background(zone: ZoneType, camera_x: f32) {
    let layers = get_parallax_layers(zone);

    for layer in &layers {
        let offset = camera_x * layer.speed_ratio;
        let tile_w = INTERNAL_WIDTH;
        let start_x = camera_x - (offset % tile_w);

        for i in 0..3 {
            let x = start_x + i as f32 * tile_w;
            draw_rectangle(x, layer.y, tile_w, layer.height, layer.color);

            match zone {
                ZoneType::Grassland => draw_grassland_details(x, layer, i),
                ZoneType::Underground => draw_underground_details(x, layer, i),
                ZoneType::Sky => draw_sky_details(x, layer, i),
                ZoneType::Castle => draw_castle_details(x, layer, i),
            }
        }
    }
}

fn get_parallax_layers(zone: ZoneType) -> Vec<ParallaxLayer> {
    match zone {
        ZoneType::Grassland => vec![
            ParallaxLayer { speed_ratio: 0.1, y: 80.0, height: 60.0, color: Color::new(0.5, 0.8, 0.5, 0.3) },
            ParallaxLayer { speed_ratio: 0.3, y: 120.0, height: 50.0, color: Color::new(0.4, 0.7, 0.4, 0.4) },
            ParallaxLayer { speed_ratio: 0.5, y: 160.0, height: 40.0, color: Color::new(0.3, 0.6, 0.3, 0.5) },
        ],
        ZoneType::Underground => vec![
            ParallaxLayer { speed_ratio: 0.1, y: 0.0, height: 30.0, color: Color::new(0.15, 0.12, 0.2, 0.8) },
            ParallaxLayer { speed_ratio: 0.3, y: 0.0, height: 20.0, color: Color::new(0.2, 0.15, 0.25, 0.6) },
        ],
        ZoneType::Sky => vec![
            ParallaxLayer { speed_ratio: 0.1, y: 40.0, height: 30.0, color: Color::new(1.0, 1.0, 1.0, 0.2) },
            ParallaxLayer { speed_ratio: 0.2, y: 80.0, height: 20.0, color: Color::new(1.0, 1.0, 1.0, 0.15) },
        ],
        ZoneType::Castle => vec![
            ParallaxLayer { speed_ratio: 0.15, y: GROUND_Y + TILE_SIZE * 2.0, height: 30.0, color: Color::new(0.8, 0.2, 0.0, 0.3) },
            ParallaxLayer { speed_ratio: 0.3, y: 60.0, height: 40.0, color: Color::new(0.25, 0.15, 0.05, 0.5) },
        ],
    }
}

fn draw_grassland_details(x: f32, layer: &ParallaxLayer, seed: usize) {
    if layer.speed_ratio < 0.2 {
        let cloud = Color::new(1.0, 1.0, 1.0, 0.5);
        for i in 0..3 {
            let cx = x + 60.0 + i as f32 * 170.0 + seed as f32 * 50.0;
            let cy = layer.y + 15.0;
            draw_circle(cx, cy, 10.0, cloud);
            draw_circle(cx + 10.0, cy - 3.0, 12.0, cloud);
            draw_circle(cx + 22.0, cy, 10.0, cloud);
            draw_circle(cx + 8.0, cy - 8.0, 8.0, cloud);
            draw_circle(cx + 16.0, cy - 6.0, 7.0, cloud);
            draw_rectangle(cx - 8.0, cy, 38.0, 8.0, cloud);
        }
    }
    if layer.speed_ratio > 0.4 {
        let hill_dark = Color::new(0.25, 0.55, 0.25, 0.6);
        let hill_light = Color::new(0.35, 0.65, 0.3, 0.5);
        for i in 0..3 {
            let hx = x + 40.0 + i as f32 * 160.0;
            let radius = 30.0 + (seed as f32 * 7.0 + i as f32 * 11.0) % 20.0;
            let base = layer.y + layer.height;
            let c = if i % 2 == 0 { hill_dark } else { hill_light };
            draw_circle(hx, base, radius, c);
            draw_circle(hx + radius * 0.6, base, radius * 0.7, c);
        }
    }
}

fn draw_underground_details(x: f32, layer: &ParallaxLayer, seed: usize) {
    if layer.speed_ratio < 0.2 {
        let rock = Color::new(0.3, 0.25, 0.2, 0.5);
        for i in 0..5 {
            let sx = x + 20.0 + i as f32 * 100.0 + seed as f32 * 30.0;
            let sw = 8.0 + (seed as f32 * 3.0 + i as f32 * 7.0) % 12.0;
            let sh = 15.0 + (seed as f32 * 5.0 + i as f32 * 11.0) % 25.0;
            draw_triangle(
                vec2(sx, layer.y),
                vec2(sx - sw * 0.5, layer.y + sh),
                vec2(sx + sw * 0.5, layer.y + sh),
                rock,
            );
            draw_triangle(
                vec2(sx + 3.0, layer.y),
                vec2(sx - sw * 0.4, layer.y + sh * 0.6),
                vec2(sx + sw * 0.3, layer.y + sh * 0.6),
                Color::new(0.35, 0.3, 0.25, 0.3),
            );
        }
    }
    if layer.speed_ratio > 0.15 {
        for i in 0..3 {
            let tx = x + 60.0 + i as f32 * 160.0 + seed as f32 * 40.0;
            let ty = layer.y + 10.0;
            draw_circle(tx, ty, 8.0, Color::new(1.0, 0.6, 0.1, 0.12));
            draw_circle(tx, ty, 4.0, Color::new(1.0, 0.7, 0.2, 0.2));
            draw_circle(tx, ty, 2.0, Color::new(1.0, 0.85, 0.4, 0.35));
        }
    }
}

fn draw_sky_details(x: f32, layer: &ParallaxLayer, seed: usize) {
    let cloud = Color::new(1.0, 1.0, 1.0, 0.35);
    for i in 0..2 {
        let cx = x + 80.0 + i as f32 * 220.0 + seed as f32 * 70.0;
        let cy = layer.y + layer.height * 0.5;
        draw_circle(cx, cy, 8.0, cloud);
        draw_circle(cx + 8.0, cy - 2.0, 10.0, cloud);
        draw_circle(cx + 18.0, cy, 8.0, cloud);
        draw_rectangle(cx - 6.0, cy, 30.0, 6.0, cloud);
    }
    let bird = Color::new(0.2, 0.2, 0.3, 0.3);
    for i in 0..4 {
        let bx = x + 50.0 + i as f32 * 130.0 + seed as f32 * 35.0;
        let by = layer.y + 5.0 + (i as f32 * 7.0) % 12.0;
        draw_line(bx, by, bx + 3.0, by - 2.0, 1.0, bird);
        draw_line(bx, by, bx - 3.0, by - 2.0, 1.0, bird);
    }
}

fn draw_castle_details(x: f32, layer: &ParallaxLayer, seed: usize) {
    if layer.speed_ratio < 0.2 {
        for i in 0..5 {
            let lx = x + 20.0 + i as f32 * 100.0 + seed as f32 * 20.0;
            let lava_a = 0.2 + ((macroquad::miniquad::date::now() as f32 * 3.0 + i as f32).sin() * 0.1);
            draw_rectangle(lx, layer.y, 40.0, layer.height, Color::new(1.0, 0.3, 0.0, lava_a));
            draw_rectangle(lx + 5.0, layer.y + 2.0, 30.0, layer.height - 4.0, Color::new(1.0, 0.5, 0.0, lava_a * 0.7));
        }
    }
    if layer.speed_ratio > 0.2 {
        let pillar = Color::new(0.25, 0.18, 0.1, 0.45);
        let cap = Color::new(0.3, 0.22, 0.12, 0.5);
        for i in 0..3 {
            let bx = x + 40.0 + i as f32 * 160.0 + seed as f32 * 40.0;
            draw_rectangle(bx + 5.0, layer.y, 20.0, layer.height, pillar);
            draw_rectangle(bx, layer.y, 30.0, 5.0, cap);
            draw_rectangle(bx, layer.y + layer.height - 5.0, 30.0, 5.0, cap);
        }
    }
}
