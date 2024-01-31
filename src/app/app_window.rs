use gtk::{
    gio,
    glib,
    glib::prelude::*,
    prelude::*,
    subclass::prelude::*
};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

use crate::app::App;


glib::wrapper! {
    pub struct AppWindow(ObjectSubclass<imp::AppWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

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

    fn app(&self) -> App {
        self.application()
            .expect("Window.application has value")
            .downcast::<App>()
            .expect("type is App")
    }
}

mod imp {
    use super::*;

    #[derive(gtk::CompositeTemplate, Default)]
    #[template(resource = "/ca/slashdev/waymenu/window.ui")]
    pub struct AppWindow {
        #[template_child]
        pub button: gtk::TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "AppWindow";
        type Type = super::AppWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
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

            // Before the window is first realized, set it up to be a layer surface
            win.init_layer_shell();

            // Exclusive input so keyboard events come through
            win.set_keyboard_mode(KeyboardMode::Exclusive);

            // Display above normal windows
            win.set_layer(Layer::Top);

            // Add action "close" to `window` taking no parameter
            let action_close = gio::ActionEntry::builder("close")
                .activate(|win: &Self::Type, _, _| {
                    win.close();
                })
                .build();
            win.add_action_entries([action_close]);

            // Connect to "clicked" signal of `button`
            self.button.connect_clicked(move |button| {
                // Set the label to "Hello World!" after the button has been clicked on
                button.set_label("Hello World!");
            });
        }
    }

    impl WidgetImpl for AppWindow {}
    impl WindowImpl for AppWindow {}
    impl ApplicationWindowImpl for AppWindow {}
}