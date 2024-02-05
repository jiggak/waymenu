use gtk::{
    gio, glib, glib::prelude::*, prelude::*, subclass::prelude::*
};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
use std::cell::RefCell;

use crate::app::{
    App, list_item::get_app_list, list_item::get_menu_list, list_item::ListItemObject
};
use crate::cli::Commands;


glib::wrapper! {
    pub struct AppWindow(ObjectSubclass<imp::AppWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[gtk::template_callbacks]
impl AppWindow {
    pub fn new(app: &App) -> Self {
        let (def_width, def_height) = app.get_default_size();

        // FIXME feels like this should be in main
        let items = match &app.imp().cli.command {
            Commands::Launcher => get_app_list(),
            Commands::Menu {file} => get_menu_list(file).unwrap()
        };

        let list_model = new_list_model(items);

        glib::Object::builder::<AppWindow>()
            .property("application", app)
            .property("name", "window")
            .property("default-width", def_width)
            .property("default-height", def_height)
            .property("list-model", list_model)
            .build()
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
        let template = include_bytes!("../../assets/ui/list_item.ui");
        let factory = gtk::BuilderListItemFactory::from_bytes(
            gtk::BuilderScope::NONE,
            &glib::Bytes::from_static(template)
        );

        self.imp().list.set_factory(Some(&factory));
    }

    fn list_filter(&self) -> gtk::StringFilter {
        self.imp().list_model.borrow()
            .model()
            .and_downcast::<gtk::SortListModel>()
            .expect("gtk::SortListModel")
            .model()
            .and_downcast::<gtk::FilterListModel>()
            .expect("gtk::FilterListModel")
            .filter()
            .and_downcast::<gtk::StringFilter>()
            .expect("gtk::StringFilter")
    }

    #[template_callback]
    fn on_list_activate(&self) {
        let item = self.list_model().selected_item();
        let item = item
            .and_downcast_ref::<ListItemObject>()
            .expect("ListItemObject");

        item.launch();

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

    #[derive(gtk::CompositeTemplate, glib::Properties, Default)]
    #[template(file = "../../assets/ui/window.ui")]
    #[properties(wrapper_type = super::AppWindow)]
    pub struct AppWindow {
        #[template_child]
        pub list: gtk::TemplateChild<gtk::ListView>,

        // Without `construct_only`, `list_model` is None inside `constructed()`
        // and getting filter for search field binding will fail
        #[property(name = "list-model", get, set, construct_only)]
        pub list_model: RefCell<gtk::SingleSelection>,

        #[template_child]
        pub search: gtk::TemplateChild<gtk::SearchEntry>
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
            self.search.property_expression("text")
                .bind(&self.obj().list_filter(), "search", gtk::Widget::NONE);

            // send key events to search when key pressed on list
            self.search.set_key_capture_widget(Some(&self.list.get()));
        }
    }

    impl WidgetImpl for AppWindow {}
    impl WindowImpl for AppWindow {}
    impl ApplicationWindowImpl for AppWindow {}
}

/// Compare two list items by label alphabetically a..z
fn cmp_list_item_alpha(obj1: &glib::Object, obj2: &glib::Object) -> gtk::Ordering {
    let list_item1 = obj1
        .downcast_ref::<ListItemObject>()
        .expect("ListItemObject");
    let list_item2 = obj2
        .downcast_ref::<ListItemObject>()
        .expect("ListItemObject");

    // sorted alphabetically a..z
    list_item1.label().cmp(&list_item2.label()).into()
}

fn new_list_model(items: impl IsA<gtk::gio::ListModel>) -> gtk::SingleSelection {
    let filter_expression = gtk::PropertyExpression::new(
        ListItemObject::static_type(),
        gtk::Expression::NONE,
        "label"
    );

    let filter = gtk::StringFilter::builder()
        .match_mode(gtk::StringFilterMatchMode::Substring)
        .ignore_case(true)
        .expression(filter_expression)
        .build();

    let filter_model = gtk::FilterListModel::new(
        Some(items),
        Some(filter)
    );

    let sorter = gtk::CustomSorter::new(cmp_list_item_alpha);
    let sort_model = gtk::SortListModel::new(
        Some(filter_model),
        Some(sorter)
    );

    gtk::SingleSelection::builder()
        .model(&sort_model)
        .build()
}
