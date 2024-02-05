use gtk::glib;
use json_comments::StripComments;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::{fs, io, path::Path};

use super::env;


#[derive(Deserialize, Clone, Copy)]
pub struct Settings {
    #[serde(default = "Settings::default_width")]
    pub width: i32,
    #[serde(default = "Settings::default_height")]
    pub height: i32,
    #[serde(default = "Settings::default_orientation")]
    pub orientation: Orientation,
    #[serde(default)]
    pub hide_search: bool
}

impl Settings {
    pub fn load<P: AsRef<Path>>(file_path: P) -> io::Result<Self> {
        match fs::read_to_string(file_path) {
            Ok(json) => Self::load_json(json.as_str()),
            Err(..) => {
                glib::g_warning!(env::app_name(), "Using default settings");
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

    fn default_width() -> i32 { Self::defaults().width }
    fn default_height() -> i32 { Self::defaults().height }
    fn default_orientation() -> Orientation { Self::defaults().orientation }
}

#[derive(Deserialize, Clone, Copy)]
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
