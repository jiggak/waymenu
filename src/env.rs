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