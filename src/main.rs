use gtk::{glib, prelude::*};

mod app;
mod cli;
mod config;
mod env;


fn main() -> glib::ExitCode {
    let app = app::App::new();

    // Set keyboard accelerator to trigger "window.close".
    app.set_accels_for_action("window.close", &["Escape"]);

    // Run the application without args to avoid conflict with
    app.run_with_args(&[] as &[&str])
}
