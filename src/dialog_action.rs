use raylib::math::Vector2;

#[derive(Clone, PartialEq)]
pub enum DialogAction {
    None,
    PlayerTurn { direction: u8 },
    CameraPan { position: Vector2, time: f32 },
    CameraShake { scale: f32 },
}
