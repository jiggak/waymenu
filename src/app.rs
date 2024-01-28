use gtk::prelude::*;
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
use std::{fs, path::PathBuf, sync::Arc};

mod list_item;

use list_item::ListItemObject;
use super::{
    cli::{Cli, Commands},
    config::Settings,
    env::get_css_path,
};


pub struct Application {
    cli: Cli,
    config: Settings
}

impl Application {
    pub fn new(cli: Cli, config: Settings) -> Self {
        Self { cli, config }
    }

    pub fn run(self) -> gtk::glib::ExitCode {
         // Create a new application
        let app = gtk::Application::builder()
            .application_id("ca.slashdev.waymenu")
            .build();

        // FIXME this feels fuggly, there must be a cleaner way to use `self` in signals
        let self_arc = Arc::new(self);

        let self1 = self_arc.clone();
        app.connect_startup(move |_| self1.load_css());

        let self2 = self_arc.clone();
        app.connect_activate(move |a| self2.show_window(a));

        // Set keyboard accelerator to trigger "win.close".
        app.set_accels_for_action("win.close", &["Escape"]);

        // Run the application
        app.run_with_args(&[] as &[&str])
    }

    fn get_css_path(&self) -> PathBuf {
        match &self.cli.style {
            Some(style_path) => style_path.to_path_buf(),
            None => get_css_path()
        }
    }

    fn load_css(&self) {
        match fs::read_to_string(self.get_css_path()) {
            Ok(css) => load_css_content(css.as_str()),
            Err(..) => {
                // TODO do I want/need fancy logging for this?
                println!("WARN: Unable to load stylesheet, using builtin style");
                load_css_content(include_str!("style.css"))
            }
        }
    }

    fn show_window(&self, app: &gtk::Application) {
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .name("window")
            .title("Waymenu")
            .width_request(self.config.width as i32)
            .height_request(self.config.height as i32)
            .build();

        // Before the window is first realized, set it up to be a layer surface
        window.init_layer_shell();

        // Exclusive input so keyboard events come through
        window.set_keyboard_mode(KeyboardMode::Exclusive);

        // Display above normal windows
        window.set_layer(Layer::Top);

        // Add action "close" to `window` taking no parameter
        let action_close = gtk::gio::ActionEntry::builder("close")
            .activate(|window: &gtk::ApplicationWindow, _, _| {
                window.close();
            })
            .build();
        window.add_action_entries([action_close]);

        let event_ctrl = gtk::EventControllerKey::new();
        event_ctrl.connect_key_pressed(|keyval, keycode, x, y| {
            println!("{keyval} {keycode} {x} {y}");
            gtk::glib::Propagation::Proceed
        });
        // connect_key_pressed
        window.add_controller(event_ctrl);

        let items = match &self.cli.command {
            Commands::Launcher => load_app_list(),
            Commands::Menu => panic!("Menu not implemented")
        };
        self.build_ui(&window, &items);

        // Present window
        window.present();
    }

    fn build_ui(&self, window: &gtk::ApplicationWindow, items: &impl IsA<gtk::gio::ListModel>) {
        let window_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .name("window-box")
            .build();

        let entry = gtk::SearchEntry::builder()
            .name("input")
            .build();

        let model = gtk::SingleSelection::builder()
            .model(items)
            .build();

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let label = gtk::Label::new(None);
            let list_item = list_item.downcast_ref::<gtk::ListItem>()
                .expect("Needs to be ListItem");

            list_item.set_child(Some(&label));

            list_item.property_expression("item")
                .chain_property::<ListItemObject>("label")
                .bind(&label, "label", gtk::Widget::NONE);
        });

        // factory.connect_bind(move |_, list_item| {
        //     // Get `GString` from `ListItem`
        //     let string_object = list_item
        //         .downcast_ref::<gtk::ListItem>()
        //         .expect("Needs to be ListItem")
        //         .item()
        //         .and_downcast::<gtk::StringObject>()
        //         .expect("The item has to be an `StringObject`.");

        //     // Get `Label` from `ListItem`
        //     let label = list_item
        //         .downcast_ref::<gtk::ListItem>()
        //         .expect("Needs to be ListItem")
        //         .child()
        //         .and_downcast::<gtk::Label>()
        //         .expect("The child has to be a `Label`.");

        //     label.set_label(&string_object.string());
        // });

        let list = gtk::ListView::builder()
            .name("list")
            .model(&model)
            .factory(&factory)
            .build();

        let scroll = gtk::ScrolledWindow::builder()
            .name("scroll")
            .hexpand(true)
            .vexpand(true)
            .child(&list)
            .build();

        window_box.append(&entry);
        window_box.append(&scroll);

        window.set_child(Some(&window_box));

        /* Simpler concept, but less scallable for large lists

        let inner_box = gtk::FlowBox::builder()
            .name("inner-box")
            .selection_mode(gtk::SelectionMode::Browse)
            .max_children_per_line(1)
            // .orientation(gtk::Orientation::Vertical)
            .build();

        for n in 1..11 {
            let label = gtk::Label::builder()
                .label(format!("Entry {}", n))
                .build();

            let entry = gtk::FlowBoxChild::builder()
                .name("entry")
                .child(&label)
                .build();

            inner_box.append(&entry);
        }

         */
    }
}

fn load_css_content(css: &str) {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_string(css);

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn load_app_list() -> gtk::gio::ListStore {
    gtk::gio::AppInfo::all().iter()
        .filter(|a| a.should_show())
        .map(ListItemObject::from)
        .collect()
}
