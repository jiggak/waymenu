pub use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

use crate::env;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to stylesheet
    /// [default: $WAYMENU_HOME/style.css or $XDG_CONFIG_HOME/waymenu/style.css]
    #[arg(short, verbatim_doc_comment)]
    pub style: Option<PathBuf>,

    /// Path to config file
    /// [default: $WAYMENU_HOME/config.json or $XDG_CONFIG_HOME/waymenu/config.json]
    #[arg(short, verbatim_doc_comment)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show launcher for installed applications
    Launcher,

    /// Show menu of options and output selection to stdout
    Menu
}

impl Cli {
    /// Get path to stylesheet from cli option or fallback to path in config dir
    pub fn get_style_path(&self) -> PathBuf {
        match &self.style {
            Some(style_path) => style_path.to_path_buf(),
            None => env::get_css_path()
        }
    }
}