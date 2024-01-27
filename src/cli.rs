pub use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to stylesheet
    /// [default: $WAYMENU_HOME/style.css or $XDG_CONFIG_HOME/waymenu/style.css]
    #[arg(short, verbatim_doc_comment)]
    pub style: Option<PathBuf>,

    /// Path to config file
    /// [default: $WAYMENU_HOME/config.json or $XDG_CONFIG_HOME/waymenu/config.json]
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    Launcher
}