use gtk::glib;
use std::{fs::{self, File}, io::{self, BufReader}, path::PathBuf};
use crate::{cli::Cli, config::Settings, env};
use super::list_item::ListItemObject;


pub struct AppContext {
    pub cli: Cli,
    pub config: Settings,
    pub list_items: Vec<ListItemObject>
}

impl AppContext {
    fn new(cli: Cli, list_items: Vec<ListItemObject>) -> io::Result<Self> {
        let config = cli.load_settings()?;

        Ok(Self { cli, config, list_items })
    }

    pub fn with_app_list(cli: Cli) -> io::Result<Self> {
        Self::new(cli, ListItemObject::app_list())
    }

    pub fn with_menu_list(cli: Cli, file_path: Option<PathBuf>) -> io::Result<Self> {
        let stream: Box<dyn io::Read> = match file_path {
            Some(file_path) => Box::new(File::open(file_path)?),
            None => Box::new(io::stdin())
        };

        let reader = BufReader::new(stream);

        Self::new(cli, ListItemObject::menu_list_from_json(reader)?)
    }

    pub fn get_window_size(&self) -> (i32, i32) {
        (self.config.width, self.config.height)
    }

    pub fn get_css_content(&self) -> String {
        let css_path = self.cli.get_style_path();

        match fs::read_to_string(&css_path) {
            Ok(css) => css,
            Err(..) => {
                glib::g_debug!(env::app_name(), "Unable to load {}, using builtin style", css_path.to_string_lossy());
                include_str!("../../assets/style.css").to_owned()
            }
        }
    }
}