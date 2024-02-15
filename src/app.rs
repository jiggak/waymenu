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

use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::cell::OnceCell;

mod app_context;
mod app_window;
mod list_item;

pub use app_context::AppContext;
use app_window::AppWindow;

glib::wrapper! {
    pub struct App(ObjectSubclass<imp::App>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl App {
    pub fn new(ctx: AppContext) -> Self {
        let app = glib::Object::builder::<Self>()
            .property("application-id", "ca.slashdev.waymenu")
            .build();

        if app.imp().ctx.set(ctx).is_err() {
            panic!("App.ctx init failed");
        }

        app
    }

    pub fn start(&self) -> glib::ExitCode {
        // Run the application without args to avoid glib complaining
        // about unknown/unexpected args
        self.run_with_args(&[] as &[&str])
    }

    pub fn ctx(&self) -> &AppContext {
        self.imp().ctx.get().unwrap()
    }
}

mod imp {
    use super::*;

    #[derive(Default)]
    // #[derive(Default, glib::Properties)]
    // #[properties(wrapper_type = super::App)]
    pub struct App {
        pub ctx: OnceCell<AppContext>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for App {
        const NAME: &'static str = "App";
        type ParentType = gtk::Application;
        type Type = super::App;
    }

    // #[glib::derived_properties]
    impl ObjectImpl for App {
        fn constructed(&self) {
            self.parent_constructed();

            // Set keyboard accelerator to trigger "window.close".
            self.obj().set_accels_for_action("window.close", &["Escape"]);
        }
    }

    impl ApplicationImpl for App {
        fn activate(&self) {
            self.parent_activate();

            let window = AppWindow::new(&self.obj());
            window.present();
        }

        fn startup(&self) {
            self.parent_startup();

            let css = self.obj().ctx().get_css_content();
            load_css_content(css.as_str());
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
