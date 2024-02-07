pub use clap::Parser;
use clap::{Args, Subcommand};
use std::{io, path::PathBuf};

use crate::env;
use crate::config::{Orientation, Settings};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to stylesheet
    /// [default: $WAYMENU_HOME/style.css or $XDG_CONFIG_HOME/waymenu/style.css]
    #[arg(short, long, verbatim_doc_comment)]
    pub style: Option<PathBuf>,

    /// Path to config file
    /// [default: $WAYMENU_HOME/config.json or $XDG_CONFIG_HOME/waymenu/config.json]
    #[arg(short, long, verbatim_doc_comment)]
    pub config: Option<PathBuf>,

    #[command(flatten)]
    pub overrides: SettingsOverride,

    #[command(subcommand)]
    pub command: Commands
}

// TODO Should I make a derive macro to generate a function that applies
// the overrides for each struct field? Then I just have to write the field
// here when adding more overrides.
#[derive(Args)]
pub struct SettingsOverride {
    #[arg(long)]
    pub width: Option<i32>,

    #[arg(long)]
    pub height: Option<i32>,

    #[arg(long)]
    pub orientation: Option<Orientation>,

    #[arg(long)]
    pub hide_search: Option<bool>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show launcher for installed applications
    Launcher,

    /// Show menu of options and output selection to stdout
    Menu {
        /// Path to json file containing an array of menu item objects
        file: PathBuf
    }
}

impl Cli {
    /// Get path to stylesheet from cli option or fallback to path in config dir
    pub fn get_style_path(&self) -> PathBuf {
        match &self.style {
            Some(style_path) => style_path.to_path_buf(),
            None => env::get_css_path()
        }
    }

    pub fn load_settings(&self) -> io::Result<Settings> {
        // path to config.json from cli option or fallback to path in config dir
        let config_path = match &self.config {
            Some(config_path) => config_path.to_path_buf(),
            None => env::get_config_path()
        };

        let mut settings = Settings::load(&config_path)?;

        if let Some(width) = self.overrides.width {
            settings.width = width;
        }

        if let Some(height) = self.overrides.height {
            settings.height = height;
        }

        if let Some(orientation) = self.overrides.orientation {
            settings.orientation = orientation;
        }

        if let Some(hide_search) = self.overrides.hide_search {
            settings.hide_search = hide_search;
        }

        Ok(settings)
    }
}
