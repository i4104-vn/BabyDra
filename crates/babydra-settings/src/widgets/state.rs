//! Settings page widget state structs.
//!
//! These hold GTK widget handles for each settings page. They were
//! historically part of `babydra-core`; they live here so core stays
//! GTK-free and settings owns its own UI state.

#![allow(dead_code)]

use gtk4::{
    Box, Button, DropDown, Entry, Label, ListBox, Overlay, ProgressBar, ScrolledWindow, Spinner,
    Stack, TextBuffer, TextView,
};

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

#[derive(Clone)]
pub struct CertificatesWidget {
    pub root: Overlay,
    pub container: Box,
    pub add_btn: Button,
    pub list_box: ListBox,
}

pub struct DisplayCardRow {
    pub container: Box,
    pub resolution_dropdown: DropDown,
    pub rate_dropdown: DropDown,
    pub orientation_dropdown: DropDown,
}

pub struct DisplaysWidget {
    pub container: Box,
    pub save_btn: Button,
    pub refresh_btn: Button,
    pub card_rows: Vec<DisplayCardRow>,
}

#[derive(Clone)]
pub struct EnvWidget {
    pub container: Box,
    pub list_box: Box,
    pub add_btn: Button,
    pub save_btn: Button,
}

#[derive(Clone)]
pub struct HostsWidget {
    pub root: Overlay,
    pub container: Box,
    pub title_label: Label,
    pub status_badge: Label,
    pub save_btn: Button,
    pub reload_btn: Button,
    pub glass_card: Box,
    pub text_view: TextView,
    pub text_buffer: TextBuffer,
}

#[derive(Clone)]
pub struct KeybindsWidget {
    pub container: Box,
    pub table_box: Box,
    pub add_btn: Button,
    pub refresh_btn: Button,
    pub save_btn: Button,
}

#[derive(Clone)]
pub struct StartupWidget {
    pub container: Box,
    pub list_box: Box,
    pub add_btn: Button,
    pub save_btn: Button,
    pub entries: Vec<Entry>,
}

#[derive(Clone)]
pub struct SystemUpdateWidget {
    pub root: Overlay,
    pub container: Box,
    pub count_badge: Label,
    pub spinner: Spinner,
    pub update_all_btn: Button,
    pub refresh_btn: Button,
    pub progress_bar: ProgressBar,
    pub status_label: Label,
    pub glass_card: Box,
    pub list_box: ListBox,
}
