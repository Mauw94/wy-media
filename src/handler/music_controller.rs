use std::cmp::{max, min};

use crossterm::event::KeyCode;

use crate::{
    app::{ActiveModules, App},
    media::player::Player,
};

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

pub fn handle_playlist(app: &mut App, key: KeyCode) -> bool {
    if app.active_modules != ActiveModules::PlayList {
        return false;
    }

    let playlist = &mut app.player.play_list;
    let len = playlist.lists.len() - 1;
    match key {
        KeyCode::Down => {
            if let Some(selected) = playlist.index.selected() {
                if selected == len {
                    playlist.index.select(Some(0));
                } else {
                    playlist.index.select(Some(min(len, selected + 1)));
                }
                return true;
            }
        }
        KeyCode::Up => {
            if let Some(selected) = playlist.index.selected() {
                if selected == 0 {
                    playlist.index.select(Some(len));
                } else {
                    playlist.index.select(Some(max(0, selected - 1)));
                }
                return true;
            }
        }
        KeyCode::Enter => {
            if let Some(selected) = playlist.index.selected() {
                play_selected_song(app, selected);
            }
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            if let Some(selected) = playlist.index.selected() {
                remove_from_playlist(app, selected);
            }
        }
        _ => {}
    }
    false
}

fn play_selected_song(app: &mut App, selected: usize) -> bool {
    let player = &mut app.player;
    player.play_selected(selected)
}

fn remove_from_playlist(app: &mut App, selected: usize) -> bool {
    let player = &mut app.player;
    player.remove_from_playlist(selected)
}
