use gtk::{gio, glib, prelude::*};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
use std::{fs, path::PathBuf, sync::Arc};

mod list_item;

use list_item::ListItemObject;
use super::{
    cli::{Cli, Commands},
    config::Settings,
    env
};


pub struct Application {
    cli: Cli,
    config: Settings
}

impl Application {
    pub fn new(cli: Cli, config: Settings) -> Self {
        Self { cli, config }
    }

    pub fn run(self) -> glib::ExitCode {
         // Create a new application
        let app = gtk::Application::builder()
            .application_id("ca.slashdev.waymenu")
            .build();

        // FIXME this feels wrong, there must be a cleaner way to use `self` in signals
        let arc_self = Arc::new(self);

        app.connect_startup(glib::clone!(
            @strong arc_self as this => move |_| this.load_css()
        ));
        app.connect_activate(glib::clone!(
            @strong arc_self as this => move |app| this.show_window(app)
        ));

        // Set keyboard accelerator to trigger "win.close".
        app.set_accels_for_action("win.close", &["Escape"]);

        // Run the application
        app.run_with_args(&[] as &[&str])
    }

    fn get_css_path(&self) -> PathBuf {
        match &self.cli.style {
            Some(style_path) => style_path.to_path_buf(),
            None => env::get_css_path()
        }
    }

    fn load_css(&self) {
        match fs::read_to_string(self.get_css_path()) {
            Ok(css) => load_css_content(css.as_str()),
            Err(..) => {
                glib::g_warning!(env::app_name(), "Unable to load stylesheet, using builtin style");
                load_css_content(include_str!("../assets/style.css"))
            }
        }
    }

    fn show_window(&self, app: &gtk::Application) {
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .name("window")
            .default_width(self.config.width as i32)
            .default_height(self.config.height as i32)
            .build();

        // Before the window is first realized, set it up to be a layer surface
        window.init_layer_shell();

        // Exclusive input so keyboard events come through
        window.set_keyboard_mode(KeyboardMode::Exclusive);

        // Display above normal windows
        window.set_layer(Layer::Top);

        // Add action "close" to `window` taking no parameter
        let action_close = gio::ActionEntry::builder("close")
            .activate(|window: &gtk::ApplicationWindow, _, _| {
                window.close();
            })
            .build();
        window.add_action_entries([action_close]);

        let items = match &self.cli.command {
            Commands::Launcher => get_app_list(),
            Commands::Menu => panic!("Menu not implemented")
        };
        self.build_ui(&window, items);

        // Present window
        window.present();
    }

    fn build_ui(&self, window: &gtk::ApplicationWindow, items: impl IsA<gio::ListModel>) {
        let window_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .name("window-box")
            .build();

        let entry = gtk::SearchEntry::builder()
            .name("search")
            .build();

        entry.connect_activate(|this| {
            this.activate_action("win.launch", None)
                .expect("Activate action win.launch");
        });

        let filter = list_view_filter();
        let model = list_view_model(items, filter.clone());
        let factory = list_view_item_factory();

        // bind search field to search filter
        entry.property_expression("text")
            .bind(&filter, "search", gtk::Widget::NONE);

        let list = gtk::ListView::builder()
            .name("list")
            .single_click_activate(true)
            // .tab_behavior(gtk::ListTabBehavior::Item)
            .can_focus(false)
            .model(&model)
            .factory(&factory)
            .build();

        list.connect_activate(|this, _| {
            this.activate_action("win.launch", None)
                .expect("Activate action win.launch");
        });

        // send key events to search field when list has focus
        entry.set_key_capture_widget(Some(&list));

        let scroll = gtk::ScrolledWindow::builder()
            .name("scroll")
            .hexpand(true)
            .vexpand(true)
            .child(&list)
            .build();

        window_box.append(&entry);
        window_box.append(&scroll);

        window.set_child(Some(&window_box));

        // I couldn't find a combination of properties to make keyboard
        // navigation work in a nice way with ListView so I had to set
        // can_focus = false and add this key handler routine
        let event_ctrl = gtk::EventControllerKey::new();
        event_ctrl.connect_key_pressed(glib::clone!(@weak model => @default-return glib::Propagation::Stop, move |_ctrl, keyval, _keycode, _state| {
            if let Some(key_name) = keyval.name() {
                if model.n_items() > 0 {
                    if key_name == "Down" || key_name == "Tab" {
                        let i = model.selected();
                        if i < (model.n_items() - 1) {
                            list.scroll_to(i+1, gtk::ListScrollFlags::SELECT, None);
                        }
                    } else if key_name == "Up" || key_name == "ISO_Left_Tab" {
                        let i = model.selected();
                        if i > 0 {
                            list.scroll_to(i-1, gtk::ListScrollFlags::SELECT, None);
                        }
                    }
                }
            }
            glib::Propagation::Stop
        }));
        window.add_controller(event_ctrl);

        let action_choose = gio::ActionEntry::builder("launch")
            .activate(glib::clone!(@weak model, @weak window => move |_, _, _| {
                let item = model.selected_item();
                let item = item
                    .and_downcast_ref::<ListItemObject>()
                    .expect("ListItemObject");

                item.launch();
                window.close();
            }))
            .build();
        window.add_action_entries([action_choose]);
    }
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

fn get_app_list() -> gio::ListStore {
    gio::AppInfo::all().iter()
        .filter(|a| a.should_show())
        .map(ListItemObject::from)
        .collect()
}

fn list_view_filter() -> gtk::StringFilter {
    let filter_expression = gtk::PropertyExpression::new(
        ListItemObject::static_type(),
        gtk::Expression::NONE,
        "label"
    );

    gtk::StringFilter::builder()
        .match_mode(gtk::StringFilterMatchMode::Substring)
        .ignore_case(true)
        .expression(filter_expression)
        .build()
}

fn list_view_model(items: impl IsA<gtk::gio::ListModel>, filter: impl IsA<gtk::Filter>) -> gtk::SingleSelection {
    let filter_model = gtk::FilterListModel::new(Some(items), Some(filter.clone()));

    let sorter = gtk::CustomSorter::new(|obj1, obj2| {
        let list_item1 = obj1
            .downcast_ref::<ListItemObject>()
            .expect("ListItemObject");
        let list_item2 = obj2
            .downcast_ref::<ListItemObject>()
            .expect("ListItemObject");

        // sorted alphabetically a..z
        list_item1.label().cmp(&list_item2.label()).into()
    });
    let sort_model = gtk::SortListModel::new(Some(filter_model), Some(sorter));

    gtk::SingleSelection::builder()
        .model(&sort_model)
        .build()
}

fn list_view_item_factory() -> gtk::SignalListItemFactory {
    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let icon = gtk::Image::builder()
            .icon_size(gtk::IconSize::Large)
            .build();

        let label = gtk::Label::builder()
            .build();

        let row_box = gtk::Box::builder()
            // .css_classes(["row-box"])
            .orientation(gtk::Orientation::Horizontal)
            .build();

        row_box.append(&icon);
        row_box.append(&label);

        let list_item = list_item
            .downcast_ref::<gtk::ListItem>()
            .expect("gtk::ListItem");

        list_item.set_child(Some(&row_box));

        list_item.property_expression("item")
            .chain_property::<ListItemObject>("label")
            .bind(&label, "label", gtk::Widget::NONE);
        list_item.property_expression("item")
            .chain_property::<ListItemObject>("icon")
            .bind(&icon, "gicon", gtk::Widget::NONE);
    });
    factory
}
