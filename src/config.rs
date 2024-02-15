use clap::ValueEnum;
use gtk::glib;
use json_comments::StripComments;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::{fs, io, path::Path};

use super::env;


#[derive(Copy, Clone, Deserialize)]
pub struct Settings {
    #[serde(default = "Settings::default_width")]
    pub width: i32,
    #[serde(default = "Settings::default_height")]
    pub height: i32,
    #[serde(default = "Settings::default_orientation")]
    pub orientation: Orientation,
    #[serde(default = "Settings::default_hide_search")]
    pub hide_search: bool,
    #[serde(default = "Settings::default_history_size")]
    pub history_size: usize
}

impl Settings {
    pub fn load(file_path: &Path) -> io::Result<Self> {
        match fs::read_to_string(file_path) {
            Ok(json) => Self::load_json(json.as_str()),
            Err(..) => {
                glib::g_debug!(env::app_name(), "Unable to load {}, using default settings", file_path.to_string_lossy());
                Ok(Self::defaults().clone())
            }
        }
    }

    fn load_json(json: &str) -> io::Result<Self> {
        let stripped = StripComments::new(json.as_bytes());
        Ok(serde_json::from_reader(stripped)?)
    }

    fn load_defaults() -> io::Result<Self> {
        Self::load_json(include_str!("../assets/config.jsonc"))
    }

    fn defaults() -> &'static Self {
        static INSTANCE: OnceCell<Settings> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            Settings::load_defaults()
                .expect("Default config should be valid json")
        })
    }

    pub fn default_width() -> i32 { Self::defaults().width }
    pub fn default_height() -> i32 { Self::defaults().height }
    pub fn default_orientation() -> Orientation { Self::defaults().orientation }
    pub fn default_hide_search() -> bool { Self::defaults().hide_search }
    pub fn default_history_size() -> usize { Self::defaults().history_size }
}

#[derive(Copy, Clone, Deserialize, ValueEnum)]
pub enum Orientation {
    #[serde(alias = "horizontal")]
    Horizontal,
    #[serde(alias = "vertical")]
    Vertical
}

impl From<Orientation> for gtk::Orientation {
    fn from(v: Orientation) -> Self {
        match v {
            Orientation::Horizontal => gtk::Orientation::Horizontal,
            Orientation::Vertical => gtk::Orientation::Vertical
        }
    }
}

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::Horizontal => f.write_str("horizontal"),
            Orientation::Vertical => f.write_str("vertical")
        }
    }
}
