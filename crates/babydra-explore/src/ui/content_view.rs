use gtk4::prelude::*;
use gtk4::{Box, Orientation, ScrolledWindow, FlowBox, Align, Label, Button, Image, ListBox, Stack};
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use babydra_common::FileEntry;
use baby_utils::explore_helpers;
use crate::ui::window::MainWindow;

pub struct ContentView {
    container: ScrolledWindow,
    flowbox: FlowBox,
    listbox: ListBox,
    stack: Stack,
    current_mode: Cell<Option<&'static str>>,
    entries: RefCell<Vec<FileEntry>>,
    window_handle: RefCell<Option<Rc<MainWindow>>>,
    current_path: RefCell<PathBuf>,
    nav_callback: std::boxed::Box<dyn Fn(PathBuf)>,
}

impl ContentView {
    pub fn new(nav_callback: impl Fn(PathBuf) + 'static) -> Self {
        let container = ScrolledWindow::new();
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_css_classes(&["content-view"]);

        let stack = Stack::new();
        container.set_child(Some(&stack));

        // FlowBox (Icon View)
        let flowbox = FlowBox::new();
        flowbox.set_valign(Align::Start);
        flowbox.set_max_children_per_line(15);
        flowbox.set_min_children_per_line(3);
        flowbox.set_selection_mode(gtk4::SelectionMode::Multiple);
        flowbox.set_margin_top(12);
        flowbox.set_margin_bottom(12);
        flowbox.set_margin_start(12);
        flowbox.set_margin_end(12);
        flowbox.set_column_spacing(10);
        flowbox.set_row_spacing(10);
        stack.add_named(&flowbox, Some("icons"));

        // ListBox (List View)
        let listbox = ListBox::new();
        listbox.set_selection_mode(gtk4::SelectionMode::Multiple);
        listbox.set_margin_top(6);
        listbox.set_margin_bottom(6);
        listbox.set_margin_start(6);
        listbox.set_margin_end(6);
        stack.add_named(&listbox, Some("list"));

        Self {
            container,
            flowbox,
            listbox,
            stack,
            current_mode: Cell::new(Some("icons")),
            entries: RefCell::new(Vec::new()),
            window_handle: RefCell::new(None),
            current_path: RefCell::new(PathBuf::new()),
            nav_callback: std::boxed::Box::new(nav_callback),
        }
    }

    pub fn widget(&self) -> &ScrolledWindow {
        &self.container
    }

    pub fn set_view_mode(&self, mode: &str) {
        if mode == "icons" {
            self.current_mode.set(Some("icons"));
            self.stack.set_visible_child_name("icons");
        } else {
            self.current_mode.set(Some("list"));
            self.stack.set_visible_child_name("list");
        }
        self.update_ui();
    }

    pub fn update(&self, entries: &[FileEntry], window_handle: Rc<MainWindow>, current_path: PathBuf) {
        self.entries.replace(entries.to_vec());
        self.window_handle.replace(Some(window_handle));
        self.current_path.replace(current_path);
        self.update_ui();
    }

    fn update_ui(&self) {
        // Clear flowbox
        while let Some(child) = self.flowbox.first_child() {
            self.flowbox.remove(&child);
        }

        // Clear listbox
        while let Some(child) = self.listbox.first_child() {
            self.listbox.remove(&child);
        }

        let mode = self.current_mode.get().unwrap_or("icons");
        let entries = self.entries.borrow();
        let window_handle_opt = self.window_handle.borrow();
        let current_path_val = self.current_path.borrow().clone();

        let window_handle = match &*window_handle_opt {
            Some(win) => win.clone(),
            None => return, // Wait until handle is set
        };

        // Setup background right click gesture for FlowBox
        let gesture_flow = gtk4::GestureClick::new();
        gesture_flow.set_button(3);
        let win = window_handle.clone();
        let cp = current_path_val.clone();
        let flow_widget = self.flowbox.clone();
        gesture_flow.connect_pressed(move |gesture, _, x, y| {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            crate::ui::context_menu::ContextMenu::show_for_empty(
                flow_widget.upcast_ref(),
                x,
                y,
                win.clone(),
                cp.clone(),
            );
        });
        self.flowbox.add_controller(gesture_flow);

        // Setup background right click gesture for ListBox
        let gesture_list = gtk4::GestureClick::new();
        gesture_list.set_button(3);
        let win = window_handle.clone();
        let cp = current_path_val.clone();
        let list_widget = self.listbox.clone();
        gesture_list.connect_pressed(move |gesture, _, x, y| {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            crate::ui::context_menu::ContextMenu::show_for_empty(
                list_widget.upcast_ref(),
                x,
                y,
                win.clone(),
                cp.clone(),
            );
        });
        self.listbox.add_controller(gesture_list);

        if mode == "icons" {
            for entry in entries.iter() {
                let item_box = Box::new(Orientation::Vertical, 6);
                item_box.set_size_request(100, 100);
                item_box.set_css_classes(&["file-item"]);

                let img = Image::from_icon_name(&entry.icon_name);
                img.set_pixel_size(48);

                let lbl = Label::builder()
                    .label(&entry.display_name)
                    .max_width_chars(12)
                    .ellipsize(gtk4::pango::EllipsizeMode::End)
                    .halign(Align::Center)
                    .build();

                item_box.append(&img);
                item_box.append(&lbl);

                let btn = Button::builder()
                    .child(&item_box)
                    .css_classes(vec!["flat".to_string()])
                    .build();

                let target_path = entry.path.clone();
                let is_dir = matches!(entry.file_type, babydra_common::FileType::Directory);
                let nav_cb = self.nav_callback.as_ref() as *const dyn Fn(PathBuf);
                let nav_cb = unsafe { &*nav_cb };

                btn.connect_clicked(move |_| {
                    if is_dir {
                        nav_cb(target_path.clone());
                    } else {
                        let uri = format!("file://{}", target_path.to_string_lossy());
                        let _ = gio::AppInfo::launch_default_for_uri(&uri, gio::AppLaunchContext::NONE);
                    }
                });

                // Attach right click gesture to item
                let gesture = gtk4::GestureClick::new();
                gesture.set_button(3);
                let target_entry = entry.clone();
                let win = window_handle.clone();
                let cp = current_path_val.clone();
                let btn_widget = btn.clone();
                gesture.connect_pressed(move |gesture, _, x, y| {
                    gesture.set_state(gtk4::EventSequenceState::Claimed);
                    crate::ui::context_menu::ContextMenu::show_for_file(
                        btn_widget.upcast_ref(),
                        x,
                        y,
                        target_entry.clone(),
                        win.clone(),
                        cp.clone(),
                    );
                });
                btn.add_controller(gesture);

                self.flowbox.append(&btn);
            }
        } else {
            // Render list/details view
            for entry in entries.iter() {
                let item_box = Box::new(Orientation::Horizontal, 12);
                item_box.set_margin_top(6);
                item_box.set_margin_bottom(6);
                item_box.set_margin_start(10);
                item_box.set_margin_end(10);

                let img = Image::from_icon_name(&entry.icon_name);
                img.set_pixel_size(24);
                item_box.append(&img);

                let lbl_name = Label::builder()
                    .label(&entry.display_name)
                    .halign(Align::Start)
                    .hexpand(true)
                    .build();
                item_box.append(&lbl_name);

                // File size info
                let size_str = if matches!(entry.file_type, babydra_common::FileType::Directory) {
                    "--".to_string()
                } else {
                    explore_helpers::format_size(entry.size)
                };
                let lbl_size = Label::builder()
                    .label(&size_str)
                    .halign(Align::End)
                    .width_request(80)
                    .build();
                lbl_size.set_css_classes(&["dim-label"]);
                item_box.append(&lbl_size);

                // Date modified info
                let date_str = if let Some(mtime) = entry.modified {
                    explore_helpers::format_date(mtime)
                } else {
                    "--".to_string()
                };
                let lbl_date = Label::builder()
                    .label(&date_str)
                    .halign(Align::End)
                    .width_request(150)
                    .build();
                lbl_date.set_css_classes(&["dim-label"]);
                item_box.append(&lbl_date);

                let btn = Button::builder()
                    .child(&item_box)
                    .css_classes(vec!["flat".to_string(), "list-row-button".to_string()])
                    .halign(Align::Fill)
                    .build();

                let target_path = entry.path.clone();
                let is_dir = matches!(entry.file_type, babydra_common::FileType::Directory);
                let nav_cb = self.nav_callback.as_ref() as *const dyn Fn(PathBuf);
                let nav_cb = unsafe { &*nav_cb };

                btn.connect_clicked(move |_| {
                    if is_dir {
                        nav_cb(target_path.clone());
                    } else {
                        let uri = format!("file://{}", target_path.to_string_lossy());
                        let _ = gio::AppInfo::launch_default_for_uri(&uri, gio::AppLaunchContext::NONE);
                    }
                });

                // Attach right click gesture to item
                let gesture = gtk4::GestureClick::new();
                gesture.set_button(3);
                let target_entry = entry.clone();
                let win = window_handle.clone();
                let cp = current_path_val.clone();
                let btn_widget = btn.clone();
                gesture.connect_pressed(move |gesture, _, x, y| {
                    gesture.set_state(gtk4::EventSequenceState::Claimed);
                    crate::ui::context_menu::ContextMenu::show_for_file(
                        btn_widget.upcast_ref(),
                        x,
                        y,
                        target_entry.clone(),
                        win.clone(),
                        cp.clone(),
                    );
                });
                btn.add_controller(gesture);

                self.listbox.append(&btn);
            }
        }
    }
}
