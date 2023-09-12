use raylib::prelude::*;
use crate::game::Game;
use crate::square_collider::SquareCollider;
use crate::draw_call::DrawCall;

pub trait Area {
    fn update(&mut self, d: &mut RaylibDrawHandle, game: Game);

    fn draw(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>, draw_after: &mut Vec<DrawCall>);

    fn get_colliders(&self) -> Option<Vec<SquareCollider>>;
}
