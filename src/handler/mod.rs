mod fs;
mod music_controller;
mod player;

use crossterm::event::KeyCode;

use self::{
    fs::handle_fs,
    music_controller::{handle_music_controller, handle_playlist},
    player::handle_player,
};

use crate::app::{ActiveModules, App};

pub fn handle_active_modules(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Tab => {
            if app.active_modules == ActiveModules::Fs {
                app.active_modules = ActiveModules::PlayList;
            } else if app.active_modules == ActiveModules::PlayList {
                app.active_modules = ActiveModules::Fs;
            }
            return true;
        }
        _ => {}
    }
    false
}
pub fn handle_keyboard_event(app: &mut App, key: KeyCode) {
    let mut flag;

    flag = handle_active_modules(app, key);
    if flag {
        return;
    }

    match app.active_modules {
        ActiveModules::Fs => {
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
        ActiveModules::PlayList => {
            flag = handle_playlist(app, key);
            if flag {
                return;
            }
        }
    }
}
