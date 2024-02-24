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
    /// [default: $WAYMENU_HOME/config.jsonc or $XDG_CONFIG_HOME/waymenu/config.jsonc]
    #[arg(short, long, verbatim_doc_comment)]
    pub config: Option<PathBuf>,

    /// Enable verbose logging (or set env var G_MESSAGES_DEBUG=all)
    #[arg(short, default_value_t = false)]
    pub verbose: bool,

    #[command(flatten)]
    pub overrides: SettingsOverride,

    #[command(subcommand)]
    pub command: Commands
}

#[derive(Args)]
pub struct SettingsOverride {
    #[arg(long, help = format!("Window width [default: {}]", Settings::default_width()))]
    pub width: Option<i32>,

    #[arg(long, help = format!("Window height [default: {}]", Settings::default_height()))]
    pub height: Option<i32>,

    #[arg(long, help = format!("Display menu in vertical or horizontal orientation\ndefault: {}", Settings::default_orientation()))]
    pub orientation: Option<Orientation>,

    #[arg(long, help = "Hide search field")]
    pub hide_search: bool,

    #[arg(long, help = format!("Max recent apps at top of launcher list\ndefault: {} [0 disables launch history]", Settings::default_history_size()))]
    pub history_size: Option<usize>,
}

impl SettingsOverride {
    fn apply(&self, settings: &mut Settings) {
        assign_some(self.width, &mut settings.width);
        assign_some(self.height, &mut settings.height);
        assign_some(self.orientation, &mut settings.orientation);
        if self.hide_search {
            settings.hide_search = true;
        }
        assign_some(self.history_size, &mut settings.history_size);
    }
}

/// If `a` is `Some`, assign value to `b`
#[inline]
fn assign_some<T>(a: Option<T>, b: &mut T) {
    if let Some(val) = a {
        *b = val;
    }
}

#[derive(Clone, Subcommand)]
pub enum Commands {
    /// Show launcher for installed applications
    Launcher,

    /// Show custom menu of options and optionally output selection to stdout
    Menu {
        /// Path to json file containing an array of menu item objects,
        /// or read from stdin when file not provided
        file: Option<PathBuf>
    },

    /// Write default config.jsonc, style.css files and exit
    InitConfig
}

impl Cli {
    /// Get path to stylesheet from cli option or fallback to path in config dir
    pub fn get_style_path(&self) -> PathBuf {
        match &self.style {
            Some(style_path) => style_path.to_path_buf(),
            None => env::get_css_path()
        }
    }

    /// Get path to config.json from cli option or fallback to path in config dir
    pub fn get_config_path(&self) -> PathBuf {
        match &self.config {
            Some(config_path) => config_path.to_path_buf(),
            None => env::get_config_path()
        }
    }

    pub fn load_settings(&self) -> io::Result<Settings> {
        let config_path = self.get_config_path();

        let mut settings = Settings::load(&config_path)?;

        self.overrides.apply(&mut settings);

        Ok(settings)
    }
}
