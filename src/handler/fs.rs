// Copyright (C) 2022 Kingtous
//
// This file is part of RustPlayer.
//
// RustPlayer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// RustPlayer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with RustPlayer.  If not, see <http://www.gnu.org/licenses/>.

use std::{
    cmp::{max, min},
    env::{current_dir, set_current_dir},
};

use crossterm::event::KeyCode;

use crate::{
    app::{ActiveModules, App, Routes},
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
            // 返回上一级[0]，文件夹
            match selected {
                0 => match dir.parent() {
                    Some(dir) => {
                        set_current_dir(dir);
                        fse.current_path = dir.to_string_lossy().to_string();
                        fse.index.select(Some(0));
                    }
                    None => {
                        return false;
                    }
                },
                num => {
                    let dir_entry = &fse.dirs[num - 1];
                    let path = dir_entry.path();
                    fse.current_path = String::from(path.to_string_lossy());
                    set_current_dir(path);
                    fse.index.select(Some(0));
                }
            }
            fse.refresh();
            return true;
        } else {
            // 文件
            let entry = &fse.files[selected - fse.dirs.len() - 1];
            let res = app.player.add_to_list(
                Media {
                    src: Source::Local(entry.file_name().to_string_lossy().to_string()),
                },
                once,
            );
            if !res {
                let msg = format!("Open failed: {}",entry.file_name().to_str().unwrap());
                app.set_msg(&msg);
            } else {
                let msg = "Start playing".to_string();
                app.set_msg(&msg);
            }
        }
        true
    } else {
        fse.index.select(Some(0));
        false
    }
}

pub fn handle_fs(app: &mut App, key: KeyCode) -> bool {
    if app.active_modules != ActiveModules::Fs {
        return false;
    }
    match app.route_stack.first() {
        Some(route) => {
            if *route != Routes::Main {
                return false;
            }
        }
        None => {
            return false;
        }
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
        KeyCode::Enter => {
            add_media_to_player(app, true);
        }
        _ => {}
    }
    false
}
