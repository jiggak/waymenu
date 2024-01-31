use gtk::{
    gio,
    glib,
    prelude::*,
    subclass::prelude::*
};
use std::fs;

mod app_window;
mod list_item;

use app_window::AppWindow;
use crate::{
    cli::{Cli, Parser},
    config::Settings,
    env
};

glib::wrapper! {
    pub struct App(ObjectSubclass<imp::App>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl App {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", "ca.slashdev.waymenu")
            .build()
    }

    // TODO concider putting this in a model
    pub fn get_default_size(&self) -> (i32, i32) {
        (self.imp().config.width, self.imp().config.height)
    }
}

mod imp {
    use super::*;

    // #[derive(Default)]
    // #[derive(Default, glib::Properties)]
    // #[properties(wrapper_type = super::App)]
    pub struct App {
        pub cli: Cli,
        pub config: Settings
    }

    impl Default for App {
        fn default() -> Self {
            // FIXME this is yucky
            // cli parsing and settings loading shouldn't be in default impl
            Self {
                cli: Cli::parse(),
                config: Settings::load()
                    .expect("Valid settings file")
            }
        }
    }

    impl App {
        fn load_css(&self) {
            let css_path = self.cli.get_style_path();

            match fs::read_to_string(css_path) {
                Ok(css) => load_css_content(css.as_str()),
                Err(..) => {
                    glib::g_warning!(env::app_name(), "Unable to load stylesheet, using builtin style");
                    load_css_content(include_str!("../resources/style.css"))
                }
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for App {
        const NAME: &'static str = "App";
        type ParentType = gtk::Application;
        type Type = super::App;
    }

    // #[glib::derived_properties]
    impl ObjectImpl for App {}

    impl ApplicationImpl for App {
        fn activate(&self) {
            self.parent_activate();

            let window = AppWindow::new(&self.obj());
            window.present();
        }

        fn startup(&self) {
            self.parent_startup();

            self.load_css();
        }
    }

    impl GtkApplicationImpl for App {}
}

fn load_css_content(css: &str) {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_string(css);

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
