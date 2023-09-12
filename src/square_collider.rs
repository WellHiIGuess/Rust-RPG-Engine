use raylib::prelude::*;

// A class for colliders that are square
#[derive(Copy, Clone, PartialEq)]
pub struct SquareCollider {
    pub rectangle: Rectangle,
}

impl SquareCollider {
    pub fn new(position: Vector2, scale: Vector2) -> Self {
        Self {
            rectangle: Rectangle::new(position.x, position.y, scale.x, scale.y),
        }
    }

    pub fn get_collision(self, rect: &Rectangle) -> bool {
        let val = self.rectangle.get_collision_rec(rect);
        if val != None {
            return true
        }

        return false
    }
}
