mod fs;
mod music_controller;
mod player;

use crossterm::event::KeyCode;

use self::{fs::handle_fs, music_controller::handle_music_controller, player::handle_player};

use crate::app::App;

pub fn handle_keyboard_event(app: &mut App, key: KeyCode) {
    let mut flag;
    flag = handle_fs(app, key);
    if flag {
        return;
    }
    flag = handle_player(app, key);
    if flag {
        return;
    }
    flag = handle_music_controller(app, key);
    if flag {
        return;
    }
}
