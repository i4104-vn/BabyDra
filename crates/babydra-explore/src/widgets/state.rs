//! Explore window widget state structs.
//!
//! These hold GTK widget handles for the main explore window (header bar,
//! content view, preview/info panels). They were historically part of
//! `babydra-core`; they live here so core stays GTK-free.

use babydra_core::models::explore::file_entry::FileEntry;
use babydra_core::services::explore::FileWatcher;
use gtk4::{ApplicationWindow, Box, Button, Entry, Image, Label, Paned, ScrolledWindow, Stack};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub struct MainWindowWidgets {
    pub window: ApplicationWindow,
    pub vbox: Box,
    pub split_paned: Paned,
    pub main_paned: Box,
    pub content_vbox: Box,
    pub layout_paned: Paned,
}

#[derive(Clone)]
pub struct HeaderBarWidgets {
    pub container: Box,
    pub btn_back: Button,
    pub btn_forward: Button,
    pub btn_up: Button,
    pub btn_refresh: Button,
    pub breadcrumb_box: Box,
    pub entry_address: Entry,
    pub address_stack: Stack,
    pub address_wrap: Box,
    pub search: Entry,
    pub btn_view_icons: Button,
    pub btn_view_list: Button,
    pub dropdown_sort: gtk4::DropDown,
    pub btn_new_folder: Button,
    pub btn_cut: Button,
    pub btn_copy: Button,
    pub btn_paste: Button,
    pub btn_rename: Button,
    pub btn_delete: Button,
    pub btn_settings: Button,
}

#[derive(Clone)]
pub struct ContentViewWidgets {
    pub container: Box,
    pub flowbox: gtk4::FlowBox,
    pub listbox: gtk4::ListBox,
    pub grid_container: gtk4::Box,
    pub stack: Stack,
    pub grid_fixed: gtk4::Fixed,
    pub grid_rubberband: gtk4::Box,
    pub list_fixed: gtk4::Fixed,
    pub list_rubberband: gtk4::Box,
    pub progress_bar: gtk4::ProgressBar,
    pub btn_back: Button,
    pub btn_forward: Button,
    pub btn_up: Button,
    pub btn_refresh: Button,
    pub breadcrumb_box: Box,
    pub entry_address: Entry,
    pub address_stack: Stack,
    pub address_wrap: Box,
    pub search: Entry,
}

#[derive(Clone)]
pub struct ContentViewHandle {
    pub widgets: ContentViewWidgets,
    pub entries: Rc<RefCell<Vec<FileEntry>>>,
    pub all_entries: Rc<RefCell<Vec<FileEntry>>>,
    pub current_path: Rc<RefCell<PathBuf>>,
    pub current_mode: Rc<RefCell<String>>,
    pub sort_mode: Rc<RefCell<String>>,
    pub nav_callback: Rc<dyn Fn(PathBuf)>,
    pub selection_callback: Rc<dyn Fn(Vec<PathBuf>)>,
    pub selected_paths: Rc<RefCell<Vec<PathBuf>>>,
    pub render_generation: Rc<RefCell<u64>>,
    pub history: Rc<RefCell<Vec<PathBuf>>>,
    pub history_index: Rc<RefCell<usize>>,
}

#[derive(Clone)]
pub struct PreviewPanelWidgets {
    pub container: ScrolledWindow,
    pub lbl_content: Label,
    pub lbl_status: Label,
    pub current_file: Rc<RefCell<Option<PathBuf>>>,
    pub watcher: Rc<RefCell<Option<FileWatcher>>>,
}

pub struct InfoPanelWidgets {
    pub container: ScrolledWindow,
    pub img_preview: Image,
    pub preview_widgets: PreviewPanelWidgets,
    pub stack: Stack,
    pub lbl_name: Label,
    pub lbl_type: Label,
    pub lbl_size: Label,
    pub lbl_modified: Label,
    pub lbl_owner: Label,
    pub lbl_permissions: Label,
}
