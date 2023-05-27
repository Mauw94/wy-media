use crossterm::event::KeyCode;

use crate::{app::App, media::player::Player};

pub fn handle_music_controller(app: &mut App, code: KeyCode) -> bool {
    let player = &mut app.player;
    match code {
        KeyCode::Char('s') | KeyCode::Char('S') => {
            if player.is_playing() {
                player.pause();
            } else {
                player.resume();
            }
            true
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            player.next();
            true
        }
        _ => false,
    }
}
