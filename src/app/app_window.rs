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

use gtk::{gio, glib::{self, prelude::*}, prelude::*, subclass::prelude::*};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
use std::cell::{Cell, RefCell};

use super::{App, list_item::ListItemObject};


glib::wrapper! {
    pub struct AppWindow(ObjectSubclass<imp::AppWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[gtk::template_callbacks]
impl AppWindow {
    pub fn new(app: &App) -> Self {
        let ctx = app.ctx();

        let (def_width, def_height) = ctx.get_window_size();

        let items = gio::ListStore::builder()
            .item_type(ListItemObject::static_type())
            .build();

        ctx.list_items.iter()
            .for_each(|i| items.append(i));

        let list_model = new_list_model(items);
        let orientation: gtk::Orientation = ctx.config.orientation.into();

        glib::Object::builder()
            .property("application", app)
            .property("name", "window")
            .property("default-width", def_width)
            .property("default-height", def_height)
            .property("list-model", list_model)
            .property("orientation", orientation)
            .property("show-search", !ctx.config.hide_search)
            .build()
    }

    fn app(&self) -> App {
        self.application().and_downcast().unwrap()
    }

    fn setup_layer(&self) {
        // Before the window is first realized, set it up to be a layer surface
        self.init_layer_shell();

        // Exclusive input so keyboard events are captured
        self.set_keyboard_mode(KeyboardMode::Exclusive);

        // Display above normal windows
        self.set_layer(Layer::Top);
    }

    fn setup_list(&self) {
        // invert orientation applied to list items
        // i.e. icon above label (vertical) when list is horizontal
        let orientation = match self.orientation() {
            gtk::Orientation::Horizontal => gtk::Orientation::Vertical,
            _ => gtk::Orientation::Horizontal
        };

        let scope = gtk::BuilderRustScope::new();
        scope.add_callback("get_orientation", move |_| Some(orientation.to_value()));

        let template = include_bytes!("../../assets/ui/list_item.ui");
        let factory = gtk::BuilderListItemFactory::from_bytes(
            Some(&scope),
            &glib::Bytes::from_static(template)
        );

        self.imp().list.set_factory(Some(&factory));

        // search field delegates "activate" signal to list when enter is pressed
        // when search is hidden, we let list field have focus so "activate" works
        if !self.show_search() {
            self.imp().list.set_can_focus(true);
        }
    }

    #[template_callback]
    fn on_list_activate(&self) {
        let item = self.list_model().selected_item();
        let item = item
            .and_downcast_ref::<ListItemObject>()
            .expect("ListItemObject");

        item.launch(self.app().ctx().config.history_size);

        self.close();
    }

    #[template_callback]
    fn on_key_pressed(&self,
        keyval: gtk::gdk::Key,
        _keycode: u32,
        _state: gtk::gdk::ModifierType
    ) -> glib::Propagation {
        // I couldn't find a combination of properties to make keyboard
        // navigation work in a nice way with ListView so I had to set
        // can-focus = false and add this key handler routine

        if let Some(key_name) = keyval.name() {
            let model = self.list_model();
            let list = self.imp().list.get();

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
    }
}

mod imp {
    use super::*;

    #[derive(gtk::CompositeTemplate, glib::Properties)]
    #[template(file = "../../assets/ui/window.ui")]
    #[properties(wrapper_type = super::AppWindow)]
    pub struct AppWindow {
        #[template_child]
        pub list: gtk::TemplateChild<gtk::ListView>,

        #[template_child]
        pub search: gtk::TemplateChild<gtk::SearchEntry>,

        // I don't know why, but the values set for properties in AppWindow::new()
        // are not available in constructed method, unless `construct_only` is set

        #[property(name = "list-model", get, set, construct_only)]
        pub list_model: RefCell<gtk::SingleSelection>,

        #[property(get, set, construct_only, builder(gtk::Orientation::Vertical))]
        pub orientation: Cell<gtk::Orientation>,

        #[property(name = "show-search", get, set, construct_only)]
        pub show_search: Cell<bool>,

        #[property(set = Self::set_search_filter)]
        pub search_filter: RefCell<String>
    }

    impl AppWindow {
        fn set_search_filter(&self, search: &glib::Value) {
            let filter = self.list_model.borrow()
                .model()
                .and_downcast::<gtk::FilterListModel>()
                .expect("gtk::FilterListModel")
                .filter()
                .and_downcast::<gtk::AnyFilter>()
                .expect("gtk::AnyFilter");

            for str_filter in filter.iter::<glib::Object>() {
                str_filter.unwrap()
                    .set_property("search", search);
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "AppWindow";
        type Type = super::AppWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }

        fn new() -> Self {
            Self {
                list: TemplateChild::default(),
                search: TemplateChild::default(),
                list_model: RefCell::default(),
                orientation: gtk::Orientation::Vertical.into(),
                show_search: true.into(),
                search_filter: "".to_string().into()
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for AppWindow {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            let win = self.obj();

            win.setup_layer();
            win.setup_list();

            // bind search field to search filter
            self.search.bind_property("text", win.as_ref(), "search_filter")
                .sync_create()
                .build();

            // send key events to search when key pressed on list
            self.search.set_key_capture_widget(Some(&self.list.get()));
        }
    }

    impl WidgetImpl for AppWindow {}
    impl WindowImpl for AppWindow {}
    impl ApplicationWindowImpl for AppWindow {}
}

fn new_list_model(items: impl IsA<gtk::gio::ListModel>) -> gtk::SingleSelection {
    let label_prop_expr = gtk::PropertyExpression::new(
        ListItemObject::static_type(),
        gtk::Expression::NONE,
        "label"
    );

    let exec_prop_expr = gtk::PropertyExpression::new(
        ListItemObject::static_type(),
        gtk::Expression::NONE,
        "executable"
    );

    let filter = gtk::AnyFilter::new();

    filter.append(
        gtk::StringFilter::builder()
            .match_mode(gtk::StringFilterMatchMode::Substring)
            .ignore_case(true)
            .expression(label_prop_expr)
            .build()
    );

    filter.append(
        gtk::StringFilter::builder()
            .match_mode(gtk::StringFilterMatchMode::Substring)
            .ignore_case(true)
            .expression(exec_prop_expr)
            .build()
    );

    let filter_model = gtk::FilterListModel::new(
        Some(items),
        Some(filter)
    );

    gtk::SingleSelection::builder()
        .model(&filter_model)
        .build()
}
