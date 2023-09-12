use raylib::prelude::*;
use crate::square_collider::SquareCollider;
use crate::enemy::Enemy;
use crate::player::Player;

#[derive(Clone, PartialEq)]
pub struct CombatEvent {
    pub position: Vector2,
    pub scale: Vector2,
    pub colliders_list: *mut Vec<SquareCollider>,
    pub enemys_list: *mut Vec<Enemy>,
    pub enemies: Vec<Enemy>,
    pub active: bool,
}

impl CombatEvent {
    pub fn new(position: Vector2, scale: Vector2, colliders_list: *mut Vec<SquareCollider>, enemys_list: *mut Vec<Enemy>, enemies: Vec<Enemy>) -> Self {
        Self {
            position,
            scale,
            colliders_list,
            enemys_list,
            enemies,
            active: false,
        }
    }

    pub fn to_square_collider(self) -> SquareCollider {
        SquareCollider::new(self.position, self.scale)
    }

    pub fn activate(&mut self) {
        self.active = true;

        unsafe {
            // left collider
            self.colliders_list.as_mut().unwrap().push(SquareCollider::new(
                Vector2::new(self.position.x - 100.0, self.position.y - 50.0),
                Vector2::new(50.0, self.scale.y + 100.0)
            ));
            
            // right collider
            self.colliders_list.as_mut().unwrap().push(SquareCollider::new(
                Vector2::new(self.position.x + self.scale.x + 50.0, self.position.y - 50.0),
                Vector2::new(50.0, self.scale.y + 100.0)
            ));
            
            // top collider
            self.colliders_list.as_mut().unwrap().push(SquareCollider::new(
                Vector2::new(self.position.x - 50.0, self.position.y - 100.0),
                Vector2::new(self.scale.x + 100.0, 50.0)
            ));
            
            // bottom collider
            self.colliders_list.as_mut().unwrap().push(SquareCollider::new(
                Vector2::new(self.position.x - 50.0, self.position.y + self.scale.y + 50.0),
                Vector2::new(self.scale.x + 100.0, 50.0)
            ));

            self.enemys_list.as_mut().unwrap().append(&mut self.enemies);
        }
    }

    // Is used for an even that happens when the player enters the combat area
    pub fn update_as_collider(&mut self, player: Player) {
        if self.clone().to_square_collider().get_collision(&player.to_rectangle()) && !self.active {
            self.activate();
        }
    }
}

