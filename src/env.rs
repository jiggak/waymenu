/*
 * Waymenu - A launcher/menu for wlroots based wayland compositors
 * Copyright (C) 2024 Josh Kropf <josh@slashdev.ca>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{env, path::PathBuf};


pub fn app_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

fn waymenu_home_dir() -> PathBuf {
    let home_dir = env::var("HOME")
        .expect("$HOME env var expected");

    match env::var("WAYMENU_HOME") {
        Ok(v) => PathBuf::from(v),
        Err(..) => match env::var("XDG_CONFIG_HOME") {
            Ok(v) => PathBuf::from(v).join(app_name()),
            Err(..) => PathBuf::from(home_dir).join(".config").join(app_name())
        }
    }
}

fn waymenu_state_dir() -> PathBuf {
    let home_dir = env::var("HOME")
        .expect("$HOME env var expected");

    match env::var("WAYMENU_HOME") {
        Ok(v) => PathBuf::from(v),
        Err(..) => match env::var("XDG_STATE_HOME") {
            Ok(v) => PathBuf::from(v).join(app_name()),
            Err(..) => PathBuf::from(home_dir).join(".local/state").join(app_name())
        }
    }
}

pub fn get_css_path() -> PathBuf {
    waymenu_home_dir().join("style.css")
}

pub fn get_config_path() -> PathBuf {
    waymenu_home_dir().join("config.jsonc")
}

pub fn get_history_path() -> PathBuf {
    waymenu_state_dir().join("history")
}