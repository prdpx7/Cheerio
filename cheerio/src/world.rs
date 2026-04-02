use macroquad::prelude::*;
use crate::constants::*;

#[derive(Debug, Clone)]
pub struct Platform {
    pub rect: Rect,
    pub breakable: bool,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub x: f32,
    pub ground_segments: Vec<Rect>,
    pub platforms: Vec<Platform>,
    pub has_gap: bool,
    pub gap_start: f32,
    pub gap_width: f32,
}

impl Chunk {
    pub fn generate(x: f32, cycle: u32) -> Self {
        let mut ground_segments = Vec::new();
        let mut platforms = Vec::new();
        let has_gap;
        let mut gap_start = 0.0;
        let mut gap_width = 0.0;

        let roll: f32 = rand::gen_range(0.0, 1.0);

        if roll < 0.3 {
            has_gap = true;
            let min_tiles = 1 + cycle.min(3) as i32;
            let max_tiles = 2 + cycle.min(3) as i32;
            let gap_tiles = rand::gen_range(min_tiles, max_tiles + 1);
            gap_width = gap_tiles as f32 * TILE_SIZE;
            gap_start = x + rand::gen_range(TILE_SIZE * 4.0, CHUNK_WIDTH - gap_width - TILE_SIZE * 2.0);

            ground_segments.push(Rect::new(x, GROUND_Y, gap_start - x, TILE_SIZE * 2.0));
            ground_segments.push(Rect::new(
                gap_start + gap_width,
                GROUND_Y,
                (x + CHUNK_WIDTH) - (gap_start + gap_width),
                TILE_SIZE * 2.0,
            ));
        } else {
            has_gap = false;
            ground_segments.push(Rect::new(x, GROUND_Y, CHUNK_WIDTH, TILE_SIZE * 2.0));
        }

        let platform_count = rand::gen_range(0, 3);
        for _ in 0..platform_count {
            let pw = rand::gen_range(2, 5) as f32 * TILE_SIZE;
            let px = x + rand::gen_range(TILE_SIZE, CHUNK_WIDTH - pw);
            let py = GROUND_Y - rand::gen_range(3, 6) as f32 * TILE_SIZE;
            platforms.push(Platform {
                rect: Rect::new(px, py, pw, TILE_SIZE),
                breakable: rand::gen_range(0.0, 1.0) < 0.4,
            });
        }

        Self {
            x,
            ground_segments,
            platforms,
            has_gap,
            gap_start,
            gap_width,
        }
    }

    pub fn draw(&self) {
        let ground_color = Color::new(0.6, 0.4, 0.2, 1.0);
        let ground_top_color = Color::new(0.3, 0.7, 0.3, 1.0);

        for seg in &self.ground_segments {
            draw_rectangle(seg.x, seg.y + 4.0, seg.w, seg.h - 4.0, ground_color);
            draw_rectangle(seg.x, seg.y, seg.w, 4.0, ground_top_color);
        }

        for plat in &self.platforms {
            let c = if plat.breakable {
                Color::new(0.7, 0.5, 0.3, 1.0)
            } else {
                Color::new(0.8, 0.7, 0.2, 1.0)
            };
            draw_rectangle(plat.rect.x, plat.rect.y, plat.rect.w, plat.rect.h, c);
        }
    }
}

pub struct World {
    pub chunks: Vec<Chunk>,
    pub cycle: u32,
}

impl World {
    pub fn new() -> Self {
        let mut chunks = Vec::new();
        chunks.push(Chunk {
            x: 0.0,
            ground_segments: vec![Rect::new(0.0, GROUND_Y, CHUNK_WIDTH, TILE_SIZE * 2.0)],
            platforms: vec![],
            has_gap: false,
            gap_start: 0.0,
            gap_width: 0.0,
        });
        for i in 1..CHUNK_BUFFER {
            chunks.push(Chunk::generate(i as f32 * CHUNK_WIDTH, 0));
        }
        Self { chunks, cycle: 0 }
    }

    pub fn update(&mut self, camera_x: f32) {
        let rightmost = self.chunks.iter().map(|c| c.x).fold(0.0_f32, f32::max);

        while rightmost + CHUNK_WIDTH < camera_x + INTERNAL_WIDTH * 2.0 {
            let next_x = self.chunks.iter().map(|c| c.x + CHUNK_WIDTH).fold(0.0_f32, f32::max);
            self.chunks.push(Chunk::generate(next_x, self.cycle));
            break;
        }

        self.chunks.retain(|c| c.x + CHUNK_WIDTH > camera_x - CHUNK_WIDTH);
    }

    pub fn draw(&self) {
        for chunk in &self.chunks {
            chunk.draw();
        }
    }

    pub fn get_ground_rects(&self) -> Vec<Rect> {
        let mut rects = Vec::new();
        for chunk in &self.chunks {
            rects.extend(chunk.ground_segments.iter().cloned());
        }
        rects
    }

    pub fn get_platform_rects(&self) -> Vec<Rect> {
        let mut plats = Vec::new();
        for chunk in &self.chunks {
            for p in &chunk.platforms {
                plats.push(p.rect);
            }
        }
        plats
    }
}
