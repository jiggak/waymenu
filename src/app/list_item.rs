use serde::Deserialize;
use std::{fs, io, path::{Path, PathBuf}};
use gtk::{gio, glib, prelude::*, subclass::prelude::*};


glib::wrapper! {
    pub struct ListItemObject(ObjectSubclass<imp::ListItemObject>);
}

impl ListItemObject {
    pub fn new<I: IsA<gio::Icon>>(id: &str, label: &str, icon: Option<&I>) -> Self {
        glib::Object::builder()
            .property("id", id)
            .property("label", label)
            .property("icon", &icon)
            .build()
    }

    pub fn launch(&self) {
        // FIXME this obviously won't work for ListItem's from JSON
        // they need to either output `id` to stdout, or run their exec field
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
            app_info.icon().as_ref()
        )
    }
}

impl From<&ListItem> for ListItemObject {
    fn from(list_item: &ListItem) -> Self {
        let icon = list_item.icon.as_ref().map(|f| {
            let file = gio::File::for_path(f);
            gio::FileIcon::new(&file)
        });

        Self::new(
            list_item.label.as_str(),
            list_item.label.as_str(),
            icon.as_ref()
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

#[derive(Deserialize)]
pub struct ListItem {
    pub label: String,
    pub icon: Option<PathBuf>,
    pub exec: Option<String>
}

impl ListItem {
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<Self>> {
        let json = fs::read_to_string(file_path)?;
        Ok(serde_json::from_str(json.as_str())?)
    }
}

pub fn get_app_list() -> gio::ListStore {
    gio::AppInfo::all().iter()
        .filter(|a| a.should_show())
        .map(ListItemObject::from)
        .collect()
}

pub fn get_menu_list<P: AsRef<Path>>(file_path: P) -> io::Result<gio::ListStore> {
    Ok(ListItem::from_file(file_path)?.iter()
        .map(ListItemObject::from)
        .collect())
}
