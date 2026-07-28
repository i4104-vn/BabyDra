use gtk4::{Box, Button, Entry, ListBox, Overlay, Stack};

#[derive(Debug, Clone)]
pub struct InstalledApp {
    pub name: String,
    pub description: String,
    pub desktop_file: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
}

pub struct AppsWidget {
    pub root: Overlay,
    pub container: Box,
    pub search_entry: Entry,
    pub tab_apps_btn: Button,
    pub tab_packages_btn: Button,
    pub stack: Stack,
    pub apps_list_box: ListBox,
    pub pkgs_list_box: ListBox,
}
