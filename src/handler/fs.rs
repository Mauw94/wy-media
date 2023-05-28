use std::{
    cmp::{max, min},
    env::{current_dir, set_current_dir},
};

use crossterm::event::KeyCode;

use crate::{
    app::{ActiveModules, App},
    media::{
        media::{Media, Source},
        player::Player,
    },
};

fn add_media_to_player(app: &mut App, once: bool) -> bool {
    let fse = &mut app.fs;
    if let Some(selected) = fse.index.selected() {
        if selected <= fse.dirs.len() {
            let dir = current_dir().unwrap();
            match selected {
                0 => match dir.parent() {
                    Some(dir) => {
                        set_current_dir(dir).unwrap();
                        fse.current_path = dir.to_string_lossy().to_string();
                        fse.index.select(Some(0));
                    }
                    None => return false,
                },
                num => {
                    let dir_entry = &fse.dirs[num - 1];
                    let path = dir_entry.path();
                    fse.current_path = String::from(path.to_string_lossy());
                    set_current_dir(path).unwrap();
                    fse.index.select(Some(0));
                }
            }
            fse.refresh();
            true
        } else {
            let entry = &fse.files[selected - fse.dirs.len() - 1];
            let mut res = app.player.add_to_list(
                Media {
                    src: Source::Local(entry.file_name().to_string_lossy().to_string()),
                },
                once,
            );
            if once {
                for i in selected - fse.dirs.len()..fse.files.len() {
                    let entry = &fse.files[i];
                    res = app.player.add_to_list(
                        Media {
                            src: Source::Local(entry.file_name().to_string_lossy().to_string()),
                        },
                        once,
                    );
                }
            }
            if !res {
                let msg = format!("Open failed: {}", entry.file_name().to_str().unwrap());
                app.set_msg(&msg);
            } else {
                let msg = format!("Start playing");
                app.set_msg(&msg);
            }
            res
        }
    } else {
        fse.index.select(Some(0));
        false
    }
}

pub fn handle_fs(app: &mut App, key: KeyCode) -> bool {
    if app.active_modules != ActiveModules::Fs {
        return false;
    }

    let fse = &mut app.fs;
    let len = fse.dirs.len() + fse.files.len();
    match key {
        KeyCode::Down => {
            if let Some(selected) = fse.index.selected() {
                if selected == len {
                    fse.index.select(Some(0));
                } else {
                    fse.index.select(Some(min(len, selected + 1)));
                }
                return true;
            } else {
                fse.index.select(Some(0));
            }
        }
        KeyCode::Up => {
            if let Some(selected) = fse.index.selected() {
                if selected == 0 {
                    fse.index.select(Some(len));
                    return true;
                }
                fse.index.select(Some(max(0, selected - 1)));
                return true;
            } else {
                fse.index.select(Some(0));
            }
        }
        KeyCode::Right => {
            add_media_to_player(app, false);
        }
        KeyCode::Left => {
            add_media_to_player(app, false);
        }
        _ => {}
    }
    false
}
