use raylib::prelude::*;
use crate::{Item, Player, SquareCollider};

#[derive(Clone, Copy, PartialEq)]
pub struct Chest {
    pub position: Vector2,
    pub scale: Vector2,
    item: Item,
    pub open: bool,
    pub openable: bool,
}

impl Chest {
    pub fn new(position: Vector2, scale: Vector2, item: Item, openable: bool) -> Self {
        Self {
            position,
            scale,
            item,
            open: false,
            openable,
        }
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle, player: &mut Player) {
        let collider = SquareCollider::new(self.position, self.scale);

        if collider.get_collision(&player.to_rectangle()) && !self.open && self.openable {
            if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                self.open = true;
                player.add_item_to_inventory(self.item);
            }
        }
    }

    pub fn draw(self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_rectangle_v(self.position, self.scale, Color::DARKPURPLE);
    }
}
