use raylib::prelude::*;

// #[derive(Clone)]
pub struct Animation {
    frames: Vec<Result<Texture2D, String>>,
    time_between_frames: f32,
    time: f32,
    step: usize,
}

impl Animation {
    pub fn new(frames: Vec<Result<Texture2D, String>>, time_between_frames: f32) -> Self {
        Self {
            frames,
            time_between_frames,
            time: 0.0,
            step: 0,
        }
    }

    pub fn next(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        self.time += d.get_frame_time();

        if self.time >= self.time_between_frames {
            self.time = 0.0;
            self.step += 1;

            if self.step > self.frames.len() - 1 {
                 self.step = 0;
             }
        }
    }

    pub fn draw(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>, position: Vector2, rotation: f32, zoom: f32, tint: Color) {
        d.draw_texture_ex(self.frames[self.step].as_ref().unwrap(), position, rotation, zoom, tint);

        self.next(d);
    }

    pub fn reset(&mut self) {
        self.step = 0;
    }
} 

