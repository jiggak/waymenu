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

    // Set keyboard accelerator to trigger "win.close".
    app.set_accels_for_action("win.close", &["Escape"]);

    // Run the application without args to avoid conflict with
    app.run_with_args(&[] as &[&str])
}
