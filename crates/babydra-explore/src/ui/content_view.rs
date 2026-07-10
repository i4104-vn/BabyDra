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
    entries: Rc<RefCell<Vec<FileEntry>>>,
    all_entries: Rc<RefCell<Vec<FileEntry>>>,
    window_handle: RefCell<Option<Rc<MainWindow>>>,
    current_path: RefCell<PathBuf>,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    selection_callback: RefCell<Option<Rc<dyn Fn(Vec<FileEntry>)>>>,
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
        flowbox.set_activate_on_single_click(false);
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
        listbox.set_activate_on_single_click(false);
        listbox.set_margin_top(6);
        listbox.set_margin_bottom(6);
        listbox.set_margin_start(6);
        listbox.set_margin_end(6);
        stack.add_named(&listbox, Some("list"));

        let nav_callback_rc = Rc::new(nav_callback);
        let entries = Rc::new(RefCell::new(Vec::<FileEntry>::new()));
        let all_entries = Rc::new(RefCell::new(Vec::<FileEntry>::new()));

        // Wire activation events for double-click / Enter navigation
        let nav_clone1 = nav_callback_rc.clone();
        let entries_clone1 = entries.clone();
        flowbox.connect_child_activated(move |_, child| {
            let idx = child.index() as usize;
            let borrowed = entries_clone1.borrow();
            if idx < borrowed.len() {
                let entry = &borrowed[idx];
                let is_dir = matches!(entry.file_type, babydra_common::FileType::Directory);
                if is_dir {
                    nav_clone1(entry.path.clone());
                } else {
                    let uri = format!("file://{}", entry.path.to_string_lossy());
                    let _ = gio::AppInfo::launch_default_for_uri(&uri, gio::AppLaunchContext::NONE);
                }
            }
        });

        let nav_clone2 = nav_callback_rc.clone();
        let entries_clone2 = entries.clone();
        listbox.connect_row_activated(move |_, row| {
            let idx = row.index() as usize;
            let borrowed = entries_clone2.borrow();
            if idx < borrowed.len() {
                let entry = &borrowed[idx];
                let is_dir = matches!(entry.file_type, babydra_common::FileType::Directory);
                if is_dir {
                    nav_clone2(entry.path.clone());
                } else {
                    let uri = format!("file://{}", entry.path.to_string_lossy());
                    let _ = gio::AppInfo::launch_default_for_uri(&uri, gio::AppLaunchContext::NONE);
                }
            }
        });

        Self {
            container,
            flowbox,
            listbox,
            stack,
            current_mode: Cell::new(Some("icons")),
            entries,
            all_entries,
            window_handle: RefCell::new(None),
            current_path: RefCell::new(PathBuf::new()),
            nav_callback: nav_callback_rc,
            selection_callback: RefCell::new(None),
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
        self.all_entries.replace(entries.to_vec());
        self.entries.replace(entries.to_vec());
        self.window_handle.replace(Some(window_handle));
        self.current_path.replace(current_path.clone());

        let mode = self.current_mode.get().unwrap_or("icons");
        self.stack.set_visible_child_name(mode);
        self.update_ui();
    }

    pub fn filter(&self, query: &str) {
        if query.is_empty() {
            let all = self.all_entries.borrow().clone();
            self.entries.replace(all);
        } else {
            use fuzzy_matcher::skim::SkimMatcherV2;
            use fuzzy_matcher::FuzzyMatcher;
            use rayon::prelude::*;

            let matcher = SkimMatcherV2::default();
            let all = self.all_entries.borrow().clone();
            
            // Match and rank items using Rayon
            let mut scored_entries: Vec<(i64, FileEntry)> = all
                .into_par_iter()
                .filter_map(|entry| {
                    if let Some(score) = matcher.fuzzy_match(&entry.display_name, query) {
                        Some((score, entry))
                    } else {
                        None
                    }
                })
                .collect();

            // Sort by score descending (highest score first)
            scored_entries.sort_by(|a, b| b.0.cmp(&a.0));

            let filtered: Vec<FileEntry> = scored_entries.into_iter().map(|(_, e)| e).collect();
            self.entries.replace(filtered);
        }
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

                let img = babydra_common::icon::get_system_or_file_icon(&entry.icon_name, "text-x-generic");
                img.set_pixel_size(48);

                let lbl = Label::builder()
                    .label(&entry.display_name)
                    .max_width_chars(12)
                    .ellipsize(gtk4::pango::EllipsizeMode::End)
                    .halign(Align::Center)
                    .build();

                item_box.append(&img);
                item_box.append(&lbl);

                // Attach right click gesture to item_box
                let gesture = gtk4::GestureClick::new();
                gesture.set_button(3);
                let target_entry = entry.clone();
                let win = window_handle.clone();
                let cp = current_path_val.clone();
                let widget_clone = item_box.clone();
                gesture.connect_pressed(move |gesture, _, x, y| {
                    gesture.set_state(gtk4::EventSequenceState::Claimed);
                    crate::ui::context_menu::ContextMenu::show_for_file(
                        widget_clone.upcast_ref(),
                        x,
                        y,
                        target_entry.clone(),
                        win.clone(),
                        cp.clone(),
                    );
                });
                item_box.add_controller(gesture);

                self.flowbox.append(&item_box);
            }
        } else {
            // Render list/details view
            for entry in entries.iter() {
                let item_box = Box::new(Orientation::Horizontal, 12);
                item_box.set_css_classes(&["list-row"]);
                item_box.set_margin_top(2);
                item_box.set_margin_bottom(2);
                item_box.set_margin_start(6);
                item_box.set_margin_end(6);

                let img = babydra_common::icon::get_system_or_file_icon(&entry.icon_name, "text-x-generic");
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

                // Attach right click gesture to item_box
                let gesture = gtk4::GestureClick::new();
                gesture.set_button(3);
                let target_entry = entry.clone();
                let win = window_handle.clone();
                let cp = current_path_val.clone();
                let widget_clone = item_box.clone();
                gesture.connect_pressed(move |gesture, _, x, y| {
                    gesture.set_state(gtk4::EventSequenceState::Claimed);
                    crate::ui::context_menu::ContextMenu::show_for_file(
                        widget_clone.upcast_ref(),
                        x,
                        y,
                        target_entry.clone(),
                        win.clone(),
                        cp.clone(),
                    );
                });
                item_box.add_controller(gesture);

                self.listbox.append(&item_box);
            }
        }
    }

    pub fn connect_selection_changed(&self, callback: impl Fn(Vec<FileEntry>) + 'static) {
        let cb = Rc::new(callback);
        self.selection_callback.replace(Some(cb.clone()));

        let flowbox = self.flowbox.clone();
        let listbox = self.listbox.clone();
        let self_weak = Rc::downgrade(&self.entries);
        let cb_flow = cb.clone();
        self.flowbox.connect_selected_children_changed(move |_| {
            if let Some(entries) = self_weak.upgrade() {
                let borrowed = entries.borrow();
                let mut selected = Vec::new();
                for child in flowbox.selected_children() {
                    let idx = child.index() as usize;
                    if idx < borrowed.len() {
                        selected.push(borrowed[idx].clone());
                    }
                }
                cb_flow(selected);
            }
        });

        let self_weak = Rc::downgrade(&self.entries);
        let cb_list = cb;
        self.listbox.connect_selected_rows_changed(move |_| {
            if let Some(entries) = self_weak.upgrade() {
                let borrowed = entries.borrow();
                let mut selected = Vec::new();
                for row in listbox.selected_rows() {
                    let idx = row.index() as usize;
                    if idx < borrowed.len() {
                        selected.push(borrowed[idx].clone());
                    }
                }
                cb_list(selected);
            }
        });
    }
}
