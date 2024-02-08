use serde::Deserialize;
use std::{
    cell::{OnceCell, RefCell},
    fs,
    io::{self, Error, ErrorKind},
    path::{Path, PathBuf},
    process::Command
};
use gtk::{gio, glib, prelude::*, subclass::prelude::*};


glib::wrapper! {
    pub struct ListItemObject(ObjectSubclass<imp::ListItemObject>);
}

impl ListItemObject {
    fn new<I: IsA<gio::Icon>>(id: &str, label: &str, icon: Option<&I>, launch: Launch) -> Self {
        let obj = glib::Object::builder::<Self>()
            .property("id", id)
            .property("label", label)
            .property("icon", &icon)
            .build();

        obj.imp().launch
            .set(launch)
            .unwrap();

        obj
    }

    pub fn launch(&self) {
        match self.imp().launch.get().unwrap() {
            Launch::DestopApp => launch_app_id(self.id().as_str()),
            Launch::Echo => println!("{}", self.id()),
            Launch::Exec(exec) => launch_exec(exec).expect("Exec success")
        };
    }
}

#[derive(Debug)]
pub enum Launch {
    DestopApp,
    Echo,
    Exec(Vec<String>)
}

impl From<&gio::AppInfo> for ListItemObject {
    fn from(app_info: &gio::AppInfo) -> Self {
        Self::new(
            app_info.id().expect("AppInfo.id").as_str(),
            app_info.name().as_str(),
            app_info.icon().as_ref(),
            Launch::DestopApp
        )
    }
}

impl From<&ListItem> for ListItemObject {
    fn from(list_item: &ListItem) -> Self {
        let icon = list_item.icon.as_ref().map(|f| {
            let file = gio::File::for_path(f);
            gio::FileIcon::new(&file)
        });

        let launch = match &list_item.exec {
            Some(exec) => Launch::Exec(exec.clone()),
            None => Launch::Echo
        };

        Self::new(
            list_item.label.as_str(),
            list_item.label.as_str(),
            icon.as_ref(),
            launch
        )
    }
}

mod imp {
    use super::*;

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::ListItemObject)]
    pub struct ListItemObject {
        #[property(get, set)]
        pub id: RefCell<String>,
        #[property(get, set)]
        pub label: RefCell<String>,
        #[property(get, set)]
        pub icon: RefCell<Option<gio::Icon>>,
        pub launch: OnceCell<Launch>
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
    pub exec: Option<Vec<String>>
}

impl ListItem {
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<Self>> {
        let json = fs::read_to_string(file_path)?;
        Ok(serde_json::from_str(json.as_str())?)
    }
}

fn launch_app_id(id: &str) {
    let app_info = gio::DesktopAppInfo::new(id)
        .expect("DesktopAppInfo from id");
    app_info.launch(&[], gio::AppLaunchContext::NONE)
        .expect("Launch application");
}

fn launch_exec(exec: &Vec<String>) -> io::Result<()> {
    let mut exec_iter = exec.iter();

    let exec_cmd = exec_iter.next()
        .ok_or(Error::new(ErrorKind::Other, "exec[0] required for command to execute"))?;

    let mut cmd = Command::new(exec_cmd);
    cmd.args(exec_iter);

    cmd.spawn()?;

    Ok(())
}

pub fn get_app_list() -> Vec<ListItemObject> {
    gio::AppInfo::all().iter()
        .filter(|a| a.should_show())
        .map(ListItemObject::from)
        .collect()
}

pub fn get_menu_list<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<ListItemObject>> {
    Ok(ListItem::from_file(file_path)?.iter()
        .map(ListItemObject::from)
        .collect())
}
