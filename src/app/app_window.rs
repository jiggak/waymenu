use gtk::{
    gio, glib, glib::prelude::*, prelude::*, subclass::prelude::*
};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
use std::cell::RefCell;

use crate::app::{App, list_item::ListItemObject};


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
        glib::Object::builder()
            .property("application", app)
            .property("name", "window")
            .property("default-width", def_width)
            .property("default-height", def_height)
            .build()
    }

    fn _app(&self) -> App {
        self.application()
            .expect("Window.application has value")
            .downcast::<App>()
            .expect("type is App")
    }

    fn setup_layer(&self) {
        // Before the window is first realized, set it up to be a layer surface
        self.init_layer_shell();

        // Exclusive input so keyboard events come through
        self.set_keyboard_mode(KeyboardMode::Exclusive);

        // Display above normal windows
        self.set_layer(Layer::Top);
    }

    fn setup_list(&self) {
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

        // bind search field to search filter
        self.imp().search.property_expression("text")
            .bind(&filter, "search", gtk::Widget::NONE);

        // TODO maybe use an empty gio::ListStore for base model and update its values later?
        let items = gio::AppInfo::all().iter()
            .filter(|a| a.should_show())
            .map(ListItemObject::from)
            .collect::<gio::ListStore>();

        let filter_model = gtk::FilterListModel::new(Some(items), Some(filter));

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

        let model = gtk::SingleSelection::builder()
            .model(&sort_model)
            .build();

        self.imp().list.set_model(Some(&model));
        self.imp().list_model.replace(model);

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

        self.imp().list.set_factory(Some(&factory));
    }

    #[template_callback]
    fn on_list_activate(&self) {
        let item = self.imp().list_model.borrow().selected_item();
        let item = item
            .and_downcast_ref::<ListItemObject>()
            .expect("ListItemObject");

        item.launch();

        self.close();
    }

    #[template_callback]
    fn on_key_pressed(&self, keyval: gtk::gdk::Key, _keycode: u32, _state: gtk::gdk::ModifierType) -> glib::Propagation {
        // I couldn't find a combination of properties to make keyboard
        // navigation work in a nice way with ListView so I had to set
        // can-focus = false and add this key handler routine

        if let Some(key_name) = keyval.name() {
            let model = self.imp().list_model.borrow();
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

    #[derive(gtk::CompositeTemplate, Default)]
    #[template(resource = "/ca/slashdev/waymenu/window.ui")]
    pub struct AppWindow {
        #[template_child]
        pub list: gtk::TemplateChild<gtk::ListView>,

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

    impl ObjectImpl for AppWindow {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            let win = self.obj();

            win.setup_layer();

            win.setup_list();

            // send key events to search when key pressed on list
            self.search.set_key_capture_widget(Some(&self.list.get()));
        }
    }

    impl WidgetImpl for AppWindow {}
    impl WindowImpl for AppWindow {}
    impl ApplicationWindowImpl for AppWindow {}
}