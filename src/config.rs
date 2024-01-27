use serde::Deserialize;
use std::{fs, io};

use super::env::get_config_path;


#[derive(Deserialize)]
pub struct Settings {
    #[serde(default = "default_width")]
    pub width: u32,
    #[serde(default = "default_height")]
    pub height: u32,
    #[serde(default)]
    pub orientation: Orientation
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            width: default_width(),
            height: default_height(),
            orientation: Default::default()
        }
    }
}

impl Settings {
    pub fn load() -> io::Result<Self> {
        let settings = match fs::read_to_string(get_config_path()) {
            Ok(json) => serde_json::from_str(json.as_str())?,
            Err(..) => {
                // TODO do I want/need fancy logging for this?
                println!("WARN: Using default settings");
                Default::default()
            }
        };

        Ok(settings)
    }
}

// FIXME make default a percentage and calculate from screen size at launch

fn default_width() -> u32 {
    320
}

fn default_height() -> u32 {
    240
}

#[derive(Deserialize)]
pub enum Orientation {
    Horizontal,
    Vertical
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Vertical
    }
}