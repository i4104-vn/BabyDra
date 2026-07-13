use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align};
use std::path::PathBuf;
use std::rc::Rc;
use babydra_common::FileEntry;
use baby_utils::explore_helpers;
use babydra_common::ContentViewWidgets;

/// Refreshes the FlowBox or ListBox view with the latest directories and file entries.
pub fn update_content_view_ui(
    widgets: &ContentViewWidgets,
    entries: &[FileEntry],
    nav_callback: &Rc<dyn Fn(PathBuf)>,
    current_path: &PathBuf,
    current_mode: &str,
) {
    // Sort: directories first (by name), then files (by name), both case-insensitive
    let mut sorted: Vec<FileEntry> = entries.to_vec();
    sorted.sort_by(|a, b| {
        let a_is_dir = matches!(a.file_type, babydra_common::FileType::Directory);
        let b_is_dir = matches!(b.file_type, babydra_common::FileType::Directory);
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()),
        }
    });
    let entries = sorted.as_slice();

    // Clear flowbox
    while let Some(child) = widgets.flowbox.first_child() {
        widgets.flowbox.remove(&child);
    }

    // Clear listbox
    while let Some(child) = widgets.listbox.first_child() {
        widgets.listbox.remove(&child);
    }

    // Setup background right click gesture for FlowBox
    {
        let gesture_flow = gtk4::GestureClick::new();
        gesture_flow.set_button(3);
        let cp = current_path.clone();
        let flow_widget = widgets.flowbox.clone();
        let nav = nav_callback.clone();
        gesture_flow.connect_pressed(move |gesture, _, x, y| {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            crate::widgets::context_menu::ContextMenu::show_for_empty(
                flow_widget.upcast_ref(),
                x,
                y,
                cp.clone(),
                nav.clone(),
            );
        });
        widgets.flowbox.add_controller(gesture_flow);
    }

    // Setup background right click gesture for ListBox
    {
        let gesture_list = gtk4::GestureClick::new();
        gesture_list.set_button(3);
        let cp = current_path.clone();
        let list_widget = widgets.listbox.clone();
        let nav = nav_callback.clone();
        gesture_list.connect_pressed(move |gesture, _, x, y| {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            crate::widgets::context_menu::ContextMenu::show_for_empty(
                list_widget.upcast_ref(),
                x,
                y,
                cp.clone(),
                nav.clone(),
            );
        });
        widgets.listbox.add_controller(gesture_list);
    }

    if current_mode == "icons" {
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
            let cp = current_path.clone();
            let widget_clone = item_box.clone();
            let nav = nav_callback.clone();
            gesture.connect_pressed(move |gesture, _, x, y| {
                gesture.set_state(gtk4::EventSequenceState::Claimed);
                crate::widgets::context_menu::ContextMenu::show_for_file(
                    widget_clone.upcast_ref(),
                    x,
                    y,
                    target_entry.clone(),
                    cp.clone(),
                    nav.clone(),
                );
            });
            item_box.add_controller(gesture);

            widgets.flowbox.append(&item_box);
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
            let lbl_size = Label::new(Some(&size_str));
            lbl_size.set_css_classes(&["list-col-meta"]);
            lbl_size.set_size_request(80, -1);
            lbl_size.set_halign(Align::End);
            item_box.append(&lbl_size);

            // Modified info
            let mod_str = if let Some(mtime) = entry.modified {
                explore_helpers::format_date(mtime)
            } else {
                "--".to_string()
            };
            let lbl_date = Label::new(Some(&mod_str));
            lbl_date.set_css_classes(&["list-col-meta"]);
            lbl_date.set_size_request(140, -1);
            lbl_date.set_halign(Align::End);
            item_box.append(&lbl_date);

            // Attach right click gesture to list item_box
            let gesture = gtk4::GestureClick::new();
            gesture.set_button(3);
            let target_entry = entry.clone();
            let cp = current_path.clone();
            let widget_clone = item_box.clone();
            let nav = nav_callback.clone();
            gesture.connect_pressed(move |gesture, _, x, y| {
                gesture.set_state(gtk4::EventSequenceState::Claimed);
                crate::widgets::context_menu::ContextMenu::show_for_file(
                    widget_clone.upcast_ref(),
                    x,
                    y,
                    target_entry.clone(),
                    cp.clone(),
                    nav.clone(),
                );
            });
            item_box.add_controller(gesture);

            widgets.listbox.append(&item_box);
        }
    }
}
