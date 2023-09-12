extern crate core;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1000, 700)
        .title("")
        .resizable()
        .build();

   rl.hide_cursor();

    let mut camera = Camera2D {
        offset: Vector2::new(0.0, 0.0),
        target: Vector2::new(0.0, 0.0),
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut active_area = 0;
    let mut areas: [Box<RefCell<dyn Area>>; 0] = [
    ];
    //
    // A vector that draws things in front of everything else
    let mut draw_after: Vec<DrawCall> = vec![];

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::WHITE);

        // Everything that is seen through the camera is put in this
        {
            let mut d2 = d.begin_mode2D(camera);
        }
    }
}
