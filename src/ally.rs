use raylib::prelude::*;
use crate::{player::Player, direction::Direction, square_collider::SquareCollider};

#[derive(Clone, Copy, PartialEq)]
pub enum AllyId {
    Lukas = 0, // For using Lukas's ability don't make an ally this ID
    Chris = 1,
}

#[derive(Clone)]
pub struct Ally {
    pub id: AllyId,
    pub position: Vector2,
    pub scale: Vector2,
    pub velocity: Vector2,
    correction_velocity: Vector2,
    speed: f32,
    facing: u8,
    pub placement: i8,
}

impl Ally {
    pub fn new(id: AllyId, position: Vector2, scale: Vector2, speed: f32, placement: i8) -> Self {
        Self {
            id,
            position,
            scale,
            velocity: Vector2::zero(),
            correction_velocity: Vector2::zero(),
            speed,
            facing: Direction::Forward as u8,
            placement,
        }
    }

    pub fn draw(self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_rectangle_v(self.position, self.scale, Color::BLACK);
    }

    pub fn update(&mut self, d:&mut RaylibDrawHandle, player: &mut Player) {
        let normalized = Vector2::new(player.position.x - self.position.x, player.position.y - self.position.y).normalized();
        let distance_to_player = self.position.distance_to(player.position);

        if normalized.y.abs() > normalized.x.abs() {
            if normalized.y > 0.0 {
                self.facing = Direction::Back as u8;
            } else if normalized.y < 0.0 {
                self.facing = Direction::Forward as u8;
            }
        } else {
            if normalized.x > 0.0 {
                self.facing = Direction::Right as u8;
            } else if normalized.x < 0.0 {
                self.facing = Direction::Left as u8;
            }
        }

        self.velocity = normalized * self.speed;

        if distance_to_player > (self.placement as f32) * 100.0 {
            self.position += self.velocity * d.get_frame_time();
            self.position += self.correction_velocity * d.get_frame_time();
        }
    }

    // Converts the player to a raylib Rectangle
    pub fn to_rectangle(self) -> Rectangle {
        Rectangle::new(self.position.x, self.position.y, self.scale.x, self.scale.y)
    }

    // Very primative path finding
    fn path_find(&mut self, i: SquareCollider, player: &mut Player) {
        self.correction_velocity = Vector2::zero();

        let down_raycast = SquareCollider::new(self.position, Vector2::new(self.scale.x, self.scale.y + 1.0));
        let up_raycast = SquareCollider::new(Vector2::new(self.position.x, self.position.y - 1.0), Vector2::new(self.scale.x, self.scale.y + 1.0));
        let right_raycast = SquareCollider::new(self.position, Vector2::new(self.scale.x + 1.0, self.scale.y));
        let left_raycast = SquareCollider::new(Vector2::new(self.position.x - 1.0, self.position.y), Vector2::new(self.scale.x + 1.0, self.scale.y));

        let normalized = Vector2::new(player.position.x - self.position.x, player.position.y - self.position.y).normalized();
        let mut cor_x = 1.0;
        let mut cor_y = 1.0;

        if normalized.y <= 0.0 {
            cor_y = -1.0;
        } 

        if normalized.x <= 0.0 {
            cor_x = -1.0;
        } 

        if i.get_collision(&down_raycast.rectangle) {
            self.correction_velocity.x = self.speed * cor_x;
        } else if i.get_collision(&up_raycast.rectangle) {
            self.correction_velocity.x = self.speed * cor_x;
        }

        if i.get_collision(&right_raycast.rectangle) {
            self.correction_velocity.y = self.speed * cor_y;
        } else if i.get_collision(&left_raycast.rectangle) {
            self.correction_velocity.y = self.speed * cor_y;
        }
    }

    pub fn rigid_body(&mut self, player: &mut Player, colliders: Option<Vec<SquareCollider>>) {

        let ally_rect = self.clone().to_rectangle();

        if colliders == None {
            return;
        }

        for i in colliders.unwrap() {
            self.path_find(i, player);

            if i.get_collision(&ally_rect) {
                let overlap_x = calculate_overlap(self.position.x, self.position.x + self.scale.x, i.rectangle.x, i.rectangle.x + i.rectangle.width);
                let overlap_y = calculate_overlap(self.position.y, self.position.y + self.scale.y, i.rectangle.y, i.rectangle.y + i.rectangle.height);

                if overlap_x < overlap_y {
                    if self.position.x > i.rectangle.x {
                        self.position.x += overlap_x;
                    } else {
                        self.position.x -= overlap_x;
                    }
                } else {
                    if self.position.y > i.rectangle.y {
                        self.position.y += overlap_y;
                    } else {
                        self.position.y -= overlap_y;
                    }
                }
            }
        }
    }
}

pub fn calculate_overlap(min1: f32, max1: f32, min2: f32, max2: f32) -> f32 {
    let overlap = f32::min(max1, max2) - f32::max(min1, min2);
    overlap
}

