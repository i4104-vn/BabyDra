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
        for (idx, entry) in entries.iter().enumerate() {
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

            let flow_child = gtk4::FlowBoxChild::new();
            flow_child.set_child(Some(&item_box));
            flow_child.set_property("name", &format!("{}", idx));
            widgets.flowbox.append(&flow_child);
        }
    } else {
        // Render list/details view
        for (idx, entry) in entries.iter().enumerate() {
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

            let list_row = gtk4::ListBoxRow::new();
            list_row.set_child(Some(&item_box));
            list_row.set_property("name", &format!("{}", idx));
            widgets.listbox.append(&list_row);
        }

        // Set header function for grouping in ListBox
        let entries_clone = entries.to_vec();
        widgets.listbox.set_header_func(move |row, before| {
            let get_group = |r: &gtk4::ListBoxRow| -> String {
                let idx = r.property::<String>("name").parse::<usize>().unwrap_or(usize::MAX);
                if idx < entries_clone.len() {
                    let entry = &entries_clone[idx];
                    if matches!(entry.file_type, babydra_common::FileType::Directory) {
                        "Folders".to_string()
                    } else {
                        match entry.path.extension() {
                            Some(ext) => format!("{} Files", ext.to_string_lossy().to_uppercase()),
                            None => "Other Files".to_string(),
                        }
                    }
                } else {
                    "".to_string()
                }
            };

            let group_curr = get_group(row);
            if group_curr.is_empty() {
                row.set_header(None::<&gtk4::Widget>);
                return;
            }

            let show_header = if let Some(before) = before {
                let group_prev = get_group(before);
                group_curr != group_prev
            } else {
                true
            };

            if show_header {
                let box_container = Box::new(Orientation::Vertical, 0);
                box_container.set_margin_top(12);
                box_container.set_margin_bottom(6);
                box_container.set_margin_start(14);
                box_container.set_margin_end(14);

                let header_lbl = Label::new(Some(&group_curr));
                header_lbl.add_css_class("group-header-label");
                header_lbl.set_halign(Align::Start);
                box_container.append(&header_lbl);

                row.set_header(Some(&box_container));
            } else {
                row.set_header(None::<&gtk4::Widget>);
            }
        });
    }
}
