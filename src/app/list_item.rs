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

use serde::Deserialize;
use std::{
    cell::{OnceCell, RefCell},
    collections::HashMap,
    fs,
    io::{self, Error, ErrorKind},
    path::PathBuf,
    process::Command
};
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use crate::env;


glib::wrapper! {
    pub struct ListItemObject(ObjectSubclass<imp::ListItemObject>);
}

impl ListItemObject {
    fn new<I: IsA<gio::Icon>>(id: &str, label: &str, executable: &str, icon: Option<&I>, launch: Launch) -> Self {
        let obj = glib::Object::builder::<Self>()
            .property("id", id)
            .property("label", label)
            .property("executable", executable)
            .property("icon", &icon)
            .build();

        obj.imp().launch
            .set(launch)
            .unwrap();

        obj
    }

    pub fn launch(&self, history_size: usize) {
        match self.imp().launch.get().unwrap() {
            Launch::DesktopApp => launch_app_id(self.id().as_str(), history_size),
            Launch::Echo => println!("{}", self.id()),
            Launch::Exec(exec) => launch_exec(exec).expect("Exec success")
        };
    }

    pub fn app_list(history_size: usize) -> io::Result<Vec<Self>> {
        let history = read_history(history_size)?;

        let (mut recent, mut apps): (Vec<_>, Vec<_>) = gio::AppInfo::all().iter()
            .filter(|a| a.should_show())
            .map(Self::from)
            .partition(|e| history.contains(&e.id()));

        let history: HashMap<_, _> = history.iter()
            .enumerate()
            .map(|(i, e)| (e, i))
            .collect();

        // match order of launch history elements
        recent.sort_by_key(|a| history.get(&a.id()).unwrap());

        // sort non-recent apps alphabetically by label
        apps.sort_by(|a, b| a.label().cmp(&b.label()));

        recent.append(&mut apps);
        Ok(recent)
    }

    pub fn menu_list_from_json<R: io::Read>(reader: R) -> io::Result<Vec<Self>> {
        Ok(ListItem::from_json_reader(reader)?.iter()
            .map(Self::from)
            .collect())
    }
}

#[derive(Debug)]
pub enum Launch {
    DesktopApp,
    Echo,
    Exec(Vec<String>)
}

impl From<&gio::AppInfo> for ListItemObject {
    fn from(app_info: &gio::AppInfo) -> Self {
        Self::new(
            app_info.id().expect("AppInfo.id").as_str(),
            app_info.name().as_str(),
            app_info.executable().file_name().unwrap().to_str().unwrap(),
            app_info.icon().as_ref(),
            Launch::DesktopApp
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
        pub executable: RefCell<String>,

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
    fn from_json_reader<R: io::Read>(reader: R) -> io::Result<Vec<Self>> {
        Ok(serde_json::from_reader(reader)?)
    }
}

fn launch_app_id(id: &str, history_size: usize) {
    let app_info = gio::DesktopAppInfo::new(id)
        .expect("DesktopAppInfo from id");
    app_info.launch(&[], gio::AppLaunchContext::NONE)
        .expect("Launch application");
    let _ = save_history(id, history_size)
        .inspect_err(|e| glib::g_error!(env::app_name(), "Error {e} saving launch history"));
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

fn read_history(length: usize) -> io::Result<Vec<String>> {
    let history_file = env::get_history_path();
    let history = if history_file.exists() {
        fs::read_to_string(history_file)?
            .lines()
            .take(length)
            .map(String::from)
            .collect()
    } else {
        vec![]
    };

    Ok(history)
}

fn save_history(app_id: &str, length: usize) -> io::Result<()> {
    let history = read_history(length)?;

    let history: Vec<_> = vec![app_id.to_owned()].into_iter()
        .chain(history.into_iter().filter(|e| e != app_id))
        .take(length)
        .collect();

    let history_file = env::get_history_path();
    let state_dir = history_file.parent().unwrap();
    fs::create_dir_all(state_dir)?;

    let content = history.join("\n");
    fs::write(history_file, content)
}
