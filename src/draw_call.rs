use std::rc::Rc;

use raylib::prelude::*;

#[derive(Clone)]
pub struct DrawCall {
    pub texture: Rc<Texture2D>,
    pub position: Vector2,
    pub rotation: f32,
    pub zoom: f32,
    pub tint: Color,
}

impl DrawCall {
    pub fn new(texture: Rc<Texture2D>, position: Vector2, rotation: f32, zoom: f32, tint: Color) -> Self {
        Self {
            texture,
            position,
            rotation,
            zoom,
            tint,
        }
    }

    pub fn draw(self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_texture_ex(self.texture.as_ref(), self.position, self.rotation, self.zoom, self.tint);
    }
}
