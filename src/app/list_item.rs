use gtk::{
    glib,
    prelude::*,
    subclass::prelude::*,
};


glib::wrapper! {
    pub struct ListItemObject(ObjectSubclass<imp::ListItemObject>);
}

impl ListItemObject {
    pub fn new(label: &str) -> Self {
        glib::Object::builder()
            .property("label", label)
            .build()
    }
}

impl From<&gtk::gio::AppInfo> for ListItemObject {
    fn from(value: &gtk::gio::AppInfo) -> Self {
        Self::new(value.name().as_str())
    }
}

mod imp {
    use super::*;
    use std::cell::RefCell;

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::ListItemObject)]
    pub struct ListItemObject {
        #[property(get, set)]
        pub label: RefCell<String>,
        // #[property(get, set)]
        // pub icon: RefCell<Option<String>>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ListItemObject {
        const NAME: &'static str = "ListItemObject";
        type Type =  super::ListItemObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ListItemObject { }
}
