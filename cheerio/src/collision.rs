use macroquad::prelude::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollisionSide {
    Top,
    Bottom,
    Left,
    Right,
}

pub fn aabb_collision(a: &Rect, b: &Rect) -> Option<CollisionSide> {
    if !a.overlaps(b) {
        return None;
    }

    let overlap_x = if a.x < b.x {
        (a.x + a.w) - b.x
    } else {
        (b.x + b.w) - a.x
    };

    let overlap_y = if a.y < b.y {
        (a.y + a.h) - b.y
    } else {
        (b.y + b.h) - a.y
    };

    if overlap_x < overlap_y {
        if a.x < b.x {
            Some(CollisionSide::Right)
        } else {
            Some(CollisionSide::Left)
        }
    } else if a.y < b.y {
        Some(CollisionSide::Top)
    } else {
        Some(CollisionSide::Bottom)
    }
}

pub fn is_stomp(player_rect: &Rect, enemy_rect: &Rect, player_vy: f32) -> bool {
    if player_vy <= 0.0 {
        return false;
    }
    let player_feet = player_rect.y + player_rect.h;
    let enemy_top = enemy_rect.y;
    let overlap = player_feet - enemy_top;
    overlap > 0.0 && overlap < player_rect.h * 0.5
}
