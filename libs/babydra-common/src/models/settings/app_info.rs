use gtk4::{Box, Button, Entry, Label, ListBox, Overlay, ProgressBar, ScrolledWindow, Stack, TextBuffer, TextView};

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

#[derive(Clone)]
pub struct AppsWidget {
    pub root: Overlay,
    pub container: Box,
    pub search_entry: Entry,
    pub refresh_btn: Button,
    pub tab_apps_btn: Button,
    pub tab_packages_btn: Button,
    pub stack: Stack,
    pub apps_list_box: ListBox,
    pub pkgs_list_box: ListBox,
    pub console_card: Box,
    pub console_title_lbl: Label,
    pub console_close_btn: Button,
    pub progress_bar: ProgressBar,
    pub text_view: TextView,
    pub text_buffer: TextBuffer,
    pub console_scroll: ScrolledWindow,
}
