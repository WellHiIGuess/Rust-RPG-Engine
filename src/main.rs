extern crate core;

mod player;
mod square_collider;
mod hit_box;
mod enemy;
mod area;
mod dialog;
mod dialog_event;
mod npc;
mod dialog_action;
mod area_traveler;
mod item;
mod chest;
mod areas;
mod trigger_event;
mod direction;
mod draw_call;
mod ally;
mod combat_event;
mod animation;

use std::cell::RefCell;
use draw_call::DrawCall;
use raylib::prelude::*;
use std::collections::HashMap;
use crate::area::Area;
use crate::area_traveler::AreaTraveler;
use areas::beginning_area::BeginningArea;
use areas::beginning_area_crash::BeginningAreaCrash;
use areas::captains_room::CaptainsRoom;
use areas::ally_test::AllyTest;
use crate::chest::Chest;
use crate::dialog_action::DialogAction;
use crate::dialog_action::DialogAction::PlayerTurn;
use crate::dialog_event::DialogEvent;
use crate::enemy::Enemy;
use crate::item::{Item, ItemId};
use crate::item::ItemId::TempHeart;
use crate::npc::NPC;
use crate::player::Player;
use crate::square_collider::SquareCollider;
use areas::combat_event_test::CombatEventTest;
use areas::crash_island::CrashIsland;
use crate::areas::enemy_test::EnemyTest;
use crate::animation::Animation;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1000, 700)
        .title("")
        .resizable()
        // .msaa_4x()
        .build();

   rl.hide_cursor();
    /*
        TODO: Set exit key none
        rl.set_exit_key(None);
     */

    let mut camera = Camera2D {
        offset: Vector2::new(0.0, 0.0),
        target: Vector2::new(0.0, 0.0),
        rotation: 0.0,
        zoom: 2.65,
        // zoom: 3.0,
    };

    let mut camera_shake = Vector2::new(0.0, 0.0);
    let mut shaking = false;
    let mut done_shaking = false;

    let mut player: Player = Player::new(0.0, 0.0, 80.0, 80.0, 200.0, 500.0);
    let player_texture_right = rl.load_texture(&thread, "src/sprites/lukas-sprite-happy-right.png");
    let player_texture_left = rl.load_texture(&thread, "src/sprites/lukas-sprite-happy-left.png");
    let player_texture_front = rl.load_texture(&thread, "src/sprites/lukas-sprite-happy-front.png");
    let player_texture_back = rl.load_texture(&thread, "src/sprites/lukas-sprite-back.png");
    // front back
    let mut front_walking_animation = Animation::new(vec![rl.load_texture(&thread, "src/sprites/lukas-sprite-happy-front.png"), rl.load_texture(&thread, "src/sprites/lukas-sprite-happy-front-walking-1.png"), rl.load_texture(&thread, "src/sprites/lukas-sprite-happy-front.png"), rl.load_texture(&thread, "src/sprites/lukas-sprite-happy-front-walking-2.png")], 0.1);
    let mut back_walking_animation = Animation::new(vec![rl.load_texture(&thread, "src/sprites/lukas-sprite-back.png"), rl.load_texture(&thread, "src/sprites/lukas-sprite-back-walking-1.png"), rl.load_texture(&thread, "src/sprites/lukas-sprite-back.png"), rl.load_texture(&thread, "src/sprites/lukas-sprite-back-walking-2.png")], 0.1);
    // left right
    let mut right_walking_animation = Animation::new(vec![rl.load_texture(&thread, "src/sprites/lukas-sprite-happy-right-walking-animation1.png"), rl.load_texture(&thread, "src/sprites/lukas-sprite-happy-right-walking-animation2.png"), rl.load_texture(&thread, "src/sprites/lukas-sprite-happy-right-walking-animation3.png"), rl.load_texture(&thread, "src/sprites/lukas-sprite-happy-right-walking-animation4.png")], 0.12);
    let mut left_walking_animation = Animation::new(vec![rl.load_texture(&thread, "src/sprites/lukas-sprite-happy-left-walking-animation1.png"), rl.load_texture(&thread, "src/sprites/lukas-sprite-happy-left-walking-animation2.png"), rl.load_texture(&thread, "src/sprites/lukas-sprite-happy-left-walking-animation3.png"), rl.load_texture(&thread, "src/sprites/lukas-sprite-happy-left-walking-animation4.png")], 0.12);

    let full_heart = rl.load_texture(&thread, "src/sprites/full-heart.png");
    let three_quarter_heart = rl.load_texture(&thread, "src/sprites/3:4-heart.png");
    let half_heart = rl.load_texture(&thread, "src/sprites/1:2-heart.png");
    let one_quarter_heart= rl.load_texture(&thread, "src/sprites/1:4-heart.png");
    let empty_heart = rl.load_texture(&thread, "src/sprites/empty-heart.png");

    let full_temp_heart = rl.load_texture(&thread, "src/sprites/full-temp-heart.png");
    let three_quarter_temp_heart = rl.load_texture(&thread, "src/sprites/3:4-temp-heart.png");
    let half_temp_heart = rl.load_texture(&thread, "src/sprites/1:2-temp-heart.png");
    let one_quarter_temp_heart= rl.load_texture(&thread, "src/sprites/1:4-temp-heart.png");

    let beginning_dialog_event = DialogEvent::new(vec![
        ("Captain. Captain Thatchback!", DialogAction::None),
        ("You scrubbing the deck!", DialogAction::None),
        ("Is he talking to me?", DialogAction::None),
        ("Oh it's just you Lukas.", PlayerTurn { direction: 2 }),
        ("I swear you look some much like the captain.", DialogAction::None),
        ("Oh by the way the captain actually wants to talk to you.", DialogAction::None),
        ("I think he's probably in his room.", DialogAction::None),
    ]);

    let chest_texture = rl.load_texture(&thread, "src/sprites/chest.png");
    let beginning_area_texture = rl.load_texture(&thread, "src/sprites/beginning-boat.png");
    let beginning_area_texture_2 = rl.load_texture(&thread, "src/sprites/beginning-boat.png");
    let beginning_pole = rl.load_texture(&thread, "src/sprites/pole.png");
    let beginning_pole_2 = rl.load_texture(&thread, "src/sprites/pole.png");
    let captains_texture = rl.load_texture(&thread, "src/sprites/captains-room.png");

    let crash_island_texture = rl.load_texture(&thread, "src/sprites/crash-island.png");

    let mut active_area = 0;
    // TODO: Remove this
    active_area = 3;
    let mut areas: [Box<RefCell<dyn Area>>; 7] = [
        Box::new(RefCell::new(BeginningArea::new(beginning_area_texture, beginning_pole))),
        Box::new(RefCell::new(CaptainsRoom::new(captains_texture, chest_texture))),
        Box::new(RefCell::new(BeginningAreaCrash::new(beginning_area_texture_2, beginning_pole_2))),
        Box::new(RefCell::new(CrashIsland::new(crash_island_texture))),
        Box::new(RefCell::new(EnemyTest::new())),
        Box::new(RefCell::new(AllyTest::new())),
        Box::new(RefCell::new(CombatEventTest::new())),
    ];

    let mut active_dialog_event: Option<String> = None;
    let mut dialog_events = HashMap::from([
        ("beginning-event", beginning_dialog_event.clone()),
        ("first-mate", areas[0].get_mut().get_dialog_events()[0].clone()),
        ("thatchback-room", areas[1].get_mut().get_dialog_events()[0].clone()),
        ("crash-event", areas[2].get_mut().get_dialog_events()[0].clone()),
        ("after-crash", DialogEvent::new(vec![
            ("(ugh, what happened)", DialogAction::None),
            ("(last thing I remember I was on Thatchback's ship)", DialogAction::None),
            ("(now that I think about it I think there was a crash)", DialogAction::None),
            ("(anyway where is everyone)", DialogAction::None),
            ("(... and ... where am I)", DialogAction::None),
        ])),
    ]);

    // Makes it only play the beginnign dialog event if you are beginning the game
    if active_area == 0 {
        active_dialog_event = Some("beginning-event".to_owned());
    }

    if active_area == 0 || active_area == 1 {
        player.position.y = 225.0;
    }

    // A vector that draws things in front of everything else
    let mut draw_after: Vec<DrawCall> = vec![];

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        
        camera.target = player.position;
        camera.offset = Vector2::new(d.get_screen_width() as f32 / 2.0 - 45.0 + camera_shake.x, d.get_screen_height() as f32 / 2.0 - 53.0 + camera_shake.x);

        d.clear_background(Color::WHITE);

        // Everything that is seen through the camera is put in this
        {
            let mut d2 = d.begin_mode2D(camera);

            areas[active_area].get_mut().draw(&mut d2, &mut draw_after);

            player.draw(&mut d2, player_texture_right.as_ref(), player_texture_left.as_ref(), player_texture_front.as_ref(), player_texture_back.as_ref(), &mut front_walking_animation, &mut back_walking_animation, &mut right_walking_animation, &mut left_walking_animation);
            player.combat(&mut d2, areas[active_area].borrow_mut().get_dialog_enemies());

            // ! This code goes after all other draw statements in the camera
            for i in draw_after {
                i.draw(&mut d2);
            }
        }

        /*
            This draws hearts
            ! STAY AWAY FROM THIS CODE IF YOU CAN !
            it is complicated
         */
        let mut hearts = player.health as f32 / 4.0;
        let mut j = 0;
        let mut jj = 0;

        for i in 0..(hearts as i32) {
            d.draw_texture(full_heart.as_ref().unwrap(), i * 50, 0, Color::WHITE);
            hearts -= 1.0;
            j += 1;
        }

        if hearts == 0.75 {
            d.draw_texture(three_quarter_heart.as_ref().unwrap(), j * 50, 0, Color::WHITE);
            jj = 1;
        } else if hearts == 0.5 {
            d.draw_texture(half_heart.as_ref().unwrap(), j * 50, 0, Color::WHITE);
            jj = 1;
        } else if hearts == 0.25 {
            d.draw_texture(one_quarter_heart.as_ref().unwrap(), j * 50, 0, Color::WHITE);
            jj = 1;
        }

        let blank_hearts = (player.max_health - player.health) / 4;

        for i in 0..blank_hearts {
            d.draw_texture(empty_heart.as_ref().unwrap(), (i as i32) * 50 + j * 50 + jj * 50, 0, Color::WHITE);
        }

        let mut temp_hearts = (player.temp_health as f32) / 4.0;
        for i in 0..temp_hearts as i32 {
            d.draw_texture(full_temp_heart.as_ref().unwrap(), (i as i32) * 50 + j * 50 + jj * 50, 0, Color::WHITE);
            temp_hearts -= 1.0;
        }

        if temp_hearts == 0.75 {
            d.draw_texture(three_quarter_temp_heart.as_ref().unwrap(), j * 50, 0, Color::WHITE);
        } else if temp_hearts == 0.5 {
            d.draw_texture(half_temp_heart.as_ref().unwrap(), j * 50, 0, Color::WHITE);
        } else if temp_hearts == 0.25 {
            d.draw_texture(one_quarter_temp_heart.as_ref().unwrap(), j * 50, 0, Color::WHITE);
        }

        // Draws bar that shows Lukas's stored force
        d.draw_rectangle(30, d.get_screen_height() - 80, 200, 50, Color::new(255, 255 - (player.stored_force * 10.0) as u8, 0, 255));

        // This makes sure that NPCs without a dialog event ID don't crash the game
        if active_dialog_event == Some("".to_owned()) {
            active_dialog_event = None;
        }

        // Shows the active dialog event
        if active_dialog_event.clone() != None {
            let result = dialog_events.get_mut(active_dialog_event.clone().unwrap().as_str()).unwrap().active(&mut d, &mut player, (&mut camera_shake, &mut shaking, &mut done_shaking));

            if result == false {
                active_dialog_event = None;
            }
        }

        // * Update code
        let colliders = areas[active_area].get_mut().get_colliders();
        player.update(&mut d);
        player.rigid_body(colliders);
        areas[active_area].get_mut().update(&mut d, &mut player, &mut active_area, &mut active_dialog_event, &mut dialog_events);
        draw_after = vec![];

        // Camera Shake code
        if camera_shake.x <= 0.0 && camera_shake.y <= 0.0 && shaking {
            done_shaking = true;
            shaking = false;
        }

        if shaking {
            camera_shake -= 200.0 * d.get_frame_time();
        }

        if !shaking {
            camera_shake = Vector2::zero();
        }
    }
}
