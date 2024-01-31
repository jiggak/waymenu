use gtk::{gio, glib, prelude::*};

mod app;
mod cli;
mod config;
mod env;


fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources");

    let app = app::App::new();

    // Set keyboard accelerator to trigger "window.close".
    app.set_accels_for_action("window.close", &["Escape"]);

    // Run the application without args to avoid conflict with
    app.run_with_args(&[] as &[&str])
}
