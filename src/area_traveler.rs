use raylib::color::Color;
use raylib::prelude::*;
use crate::SquareCollider;

#[derive(Clone)]
pub struct AreaTraveler {
    pub position: Vector2,
    pub scale: Vector2,
    pub player_point: Vector2,
    pub travel_point: usize,
}

impl AreaTraveler {
    pub fn new(x: f32, y: f32, width: f32, height: f32, player_point: Vector2, travel_point: usize) -> Self {
        Self {
            position: Vector2::new(x, y),
            scale: Vector2::new(width, height),
            player_point,
            travel_point,
        }
    }

    pub fn test_draw(self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_rectangle_v(self.position, self.scale, Color::MAROON);
    }

    pub fn to_square_collider(self) -> SquareCollider {
        SquareCollider::new(self.position, self.scale)
    }
}
