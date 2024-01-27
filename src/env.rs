use std::{env, path::PathBuf};

pub fn get_waymenu_home() -> PathBuf {
    let home_dir = env::var("HOME")
        .expect("$HOME env var expected");

    match env::var("WAYMENU_HOME") {
        Ok(v) => PathBuf::from(v),
        Err(..) => match env::var("XDG_CONFIG_HOME") {
            Ok(v) => PathBuf::from(v).join("waymenu"),
            Err(..) => PathBuf::from(home_dir).join(".config/waymenu")
        }
    }
}

pub fn get_css_path() -> PathBuf {
    get_waymenu_home().join("style.css")
}

pub fn get_config_path() -> PathBuf {
    get_waymenu_home().join("config.json")
}