use crate::dialog::Dialog;
use crate::dialog_action::DialogAction;
use crate::dialog_action::DialogAction::CameraShake;
use crate::Player;
use raylib::prelude::*;

#[derive(Clone)]
pub struct DialogEvent {
    pub dialog_vec: Vec<Dialog>,
    pub step: u16,
    pub done: bool,
}

impl DialogEvent {
    pub fn new_from_strings(strings: Vec<&str>) -> Self {
        let mut dialog_vec = vec![];

        for i in strings {
            dialog_vec.push(Dialog::new(i, DialogAction::None));
        }

        Self {
            dialog_vec,
            step: 0,
            done: false,
        }
    }

    pub fn new(events: Vec<(&str, DialogAction)>) -> Self {
        let mut dialog_vec = vec![];

        for i in events {
            dialog_vec.push(Dialog::new(i.0, i.1));
        }

        Self {
            dialog_vec,
            step: 0,
            done: false,
        }
    }

    pub fn add_dialog(&mut self, dialog: Dialog) {
        self.dialog_vec.push(dialog);
    }

    // If the dialog is still active it wll show dialog otherwise it will end the event
    pub fn active(&mut self, d: &mut RaylibDrawHandle, player: &mut Player, camera_shake: (&mut Vector2, &mut bool, &mut bool)) -> bool {
        if self.step < self.dialog_vec.len() as u16 {
            self.clone().dialog_vec[self.step as usize]
                .clone()
                .show_text(d, player);

            if let CameraShake {scale: x} = self.dialog_vec[self.step as usize].clone().dialog_action {
                if !*camera_shake.1 && !*camera_shake.2 {
                    *camera_shake.0 = Vector2::new(x, x);
                    *camera_shake.1 = true;
                }
            }
                
            if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                self.step += 1;
                *camera_shake.2 = false;
            }

            player.moving = false;

            return true;
        }

        self.step = 0;
        self.done = true;
        player.moving = true;
        return false;
    }
}
