use raylib::math::Vector2;
use crate::{Enemy, Player, SquareCollider};

// TODO: Remove Copy, Clone
#[derive(Copy, Clone)]
pub struct HitBox {
    pub collider: SquareCollider,
    pub damage: u32,
}

impl HitBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32, damage: u32) -> Self {
        Self {
            collider: SquareCollider::new(Vector2::new(x, y), Vector2::new(width, height)),
            damage,
        }
    }

    pub fn get_collision(self, player: &mut Player) {
        if self.collider.get_collision(&player.to_rectangle()) {
            player.damage(self.damage);
        }
    }

    pub fn get_collision_enemy(self, enemies: &mut Vec<Enemy>, aditional_damage: f32) {
        for i in enemies {
            if self.collider.get_collision(&i.to_rectangle()) {
                i.damage(self.damage + aditional_damage as u32);
            }
        }
    }
}
