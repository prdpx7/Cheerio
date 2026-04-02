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
        for i in 0..3 {
            let cx = x + 60.0 + i as f32 * 150.0 + seed as f32 * 50.0;
            draw_circle(cx, layer.y + 10.0, 20.0, Color::new(1.0, 1.0, 1.0, 0.4));
            draw_circle(cx + 15.0, layer.y + 5.0, 15.0, Color::new(1.0, 1.0, 1.0, 0.4));
        }
    }
    if layer.speed_ratio > 0.4 {
        for i in 0..4 {
            let hx = x + 30.0 + i as f32 * 120.0;
            let hh = 15.0 + (seed as f32 * 7.0 + i as f32 * 13.0) % 20.0;
            draw_circle(hx, layer.y + layer.height - hh * 0.5, hh, layer.color);
        }
    }
}

fn draw_underground_details(x: f32, layer: &ParallaxLayer, seed: usize) {
    if layer.speed_ratio < 0.2 {
        for i in 0..5 {
            let sx = x + 20.0 + i as f32 * 100.0 + seed as f32 * 30.0;
            let sw = 8.0 + (seed as f32 * 3.0 + i as f32 * 7.0) % 12.0;
            let sh = 15.0 + (seed as f32 * 5.0 + i as f32 * 11.0) % 25.0;
            draw_triangle(
                vec2(sx, layer.y),
                vec2(sx - sw * 0.5, layer.y + sh),
                vec2(sx + sw * 0.5, layer.y + sh),
                Color::new(0.3, 0.25, 0.2, 0.4),
            );
        }
    }
}

fn draw_sky_details(x: f32, layer: &ParallaxLayer, seed: usize) {
    for i in 0..2 {
        let cx = x + 80.0 + i as f32 * 200.0 + seed as f32 * 70.0;
        draw_circle(cx, layer.y + layer.height * 0.5, 25.0, Color::new(1.0, 1.0, 1.0, 0.3));
        draw_circle(cx + 20.0, layer.y + layer.height * 0.3, 18.0, Color::new(1.0, 1.0, 1.0, 0.3));
    }
}

fn draw_castle_details(x: f32, layer: &ParallaxLayer, seed: usize) {
    if layer.speed_ratio > 0.2 {
        for i in 0..3 {
            let bx = x + 40.0 + i as f32 * 160.0 + seed as f32 * 40.0;
            draw_rectangle(bx, layer.y, 30.0, layer.height, Color::new(0.3, 0.2, 0.1, 0.3));
        }
    }
}
