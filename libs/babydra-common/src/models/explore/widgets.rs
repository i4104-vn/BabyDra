use gtk4::{ApplicationWindow, Box, Paned, Button, Entry, Stack, ScrolledWindow, Label, Image};
use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use crate::models::explore::file_entry::FileEntry;
use crate::services::explore::FileWatcher;

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
