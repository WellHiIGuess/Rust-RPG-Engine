use raylib::prelude::*;
use crate::dialog_action::DialogAction;
use crate::dialog_action::DialogAction::PlayerTurn;
use crate::Player;

#[derive(Clone)]
pub struct Dialog {
    pub text: String,
    pub dialog_action: DialogAction,
}

impl Dialog {
    pub fn new(text: &str, dialog_action: DialogAction) -> Self {
        Self {
            text: text.to_string(),
            dialog_action,
        }
    }

    pub fn show_text(self, d: &mut RaylibDrawHandle, player: &mut Player) {
        // TODO: make it show who's talking

        let split_string = self.text.split(" ");
        let mut new_string = String::new();
        let mut line_length: i32 = 0;

        for i in split_string {
            line_length += 5 * i.len() as i32;

            if line_length < 200 {
                new_string += &*(i.to_owned() + " ");
            } else {
                new_string += "\n";
                new_string += &*(i.to_owned() + " ");
                line_length = 0;
            }
        }

        d.draw_rectangle(d.get_screen_width() / 2 - 355, d.get_screen_height() - 355, 710, 310, Color::WHITE);
        d.draw_rectangle(d.get_screen_width() / 2 - 350, d.get_screen_height() - 350, 700, 300, Color::BLACK);
        d.draw_text(new_string.as_str(), 50 + d.get_screen_width() / 2 - 350, 50 + d.get_screen_height() - 350, 24, Color::WHITE);

        if let Some(PlayerTurn {direction: x}) = Option::from(self.dialog_action.clone()) {
            player.facing = x;
        }
    }
}
