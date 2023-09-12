use raylib::prelude::*;
use crate::{SquareCollider, DialogEvent, Player};

#[derive(Clone)]
pub struct TriggerEvent {
    square_collider: SquareCollider,
    pub dialog_event: DialogEvent,
    dialog_event_id: String,
    pub enabled: bool,
}

impl TriggerEvent {
    pub fn new(x: f32, y: f32, width: f32, height: f32, dialog_event: DialogEvent, dialog_event_id: &str) -> Self {
        Self {
            square_collider: SquareCollider::new(Vector2::new(x, y), Vector2::new(width, height)),
            dialog_event,
            dialog_event_id: dialog_event_id.to_owned(),
            enabled: true,
        }
    }

    pub fn update(&mut self, player: Player, current_dialog_event: &mut Option<String>) {
        if self.square_collider.get_collision(&player.to_rectangle()) && self.enabled {
            *current_dialog_event = Some(self.dialog_event_id.clone());
            self.enabled = false;
        }
    }
}
