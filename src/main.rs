mod app;
mod cli;
mod config;
mod env;

use app::Application;
use cli::{Cli, Parser};
use config::Settings;

fn main() -> gtk::glib::ExitCode {
    let cli = Cli::parse();
    let config = Settings::load()
        .expect("Valid settings file");

    let app = Application::new(cli, config);
    app.run()
}