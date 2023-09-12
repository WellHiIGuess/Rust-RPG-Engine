use raylib::prelude::*;
use std::collections::HashMap;
use crate::{DialogEvent, Player, SquareCollider, enemy::Enemy, draw_call::DrawCall};

pub trait Area {
    fn update(&mut self, d: &mut RaylibDrawHandle, player: &mut Player, active_area: &mut usize, active_dialog_event: &mut Option<String>, dialog_events: &mut HashMap<&str, DialogEvent>);

    fn draw(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>, draw_after: &mut Vec<DrawCall>);

    fn get_colliders(&self) -> Option<Vec<SquareCollider>>;


    fn get_dialog_events(&self) -> Vec<DialogEvent>;

    fn get_dialog_enemies(&mut self) -> Option<&mut Vec<Enemy>>;
}
