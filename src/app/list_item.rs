use gtk::{
    gio,
    glib,
    prelude::*,
    subclass::prelude::*,
};


glib::wrapper! {
    pub struct ListItemObject(ObjectSubclass<imp::ListItemObject>);
}

impl ListItemObject {
    pub fn new(id: &str, label: &str, icon: Option<gio::Icon>) -> Self {
        glib::Object::builder()
            .property("id", id)
            .property("label", label)
            .property("icon", icon)
            .build()
    }

    pub fn launch(&self) {
        let app_info = gio::DesktopAppInfo::new(self.id().as_str())
            .expect("DesktopAppInfo from id");
        app_info.launch(&[], gio::AppLaunchContext::NONE)
            .expect("Launch application");
    }
}

impl From<&gio::AppInfo> for ListItemObject {
    fn from(app_info: &gio::AppInfo) -> Self {
        Self::new(
            app_info.id().expect("AppInfo.id").as_str(),
            app_info.name().as_str(),
            app_info.icon()
        )
    }
}

mod imp {
    use super::*;
    use std::cell::RefCell;

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::ListItemObject)]
    pub struct ListItemObject {
        #[property(get, set)]
        pub id: RefCell<String>,
        #[property(get, set)]
        pub label: RefCell<String>,
        #[property(get, set)]
        pub icon: RefCell<Option<gio::Icon>>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ListItemObject {
        const NAME: &'static str = "ListItemObject";
        type Type =  super::ListItemObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ListItemObject { }
}
