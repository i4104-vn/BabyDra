use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align};
use std::path::PathBuf;
use std::rc::Rc;
use babydra_common::FileEntry;
use baby_utils::explore;
use babydra_common::ContentViewWidgets;
use babydra_common::ContentViewHandle;

use super::helpers::create_flow_child;

/// Refreshes the FlowBox or ListBox view with the latest directories and file entries.
pub fn update_content_view_ui(handle: &ContentViewHandle) {
    let widgets = &handle.widgets;
    let entries_borrow = handle.entries.borrow();
    let entries = &*entries_borrow;
    let nav_callback = &handle.nav_callback;
    let current_path_borrow = handle.current_path.borrow();
    let current_path = &*current_path_borrow;
    let current_mode_borrow = handle.current_mode.borrow();
    let current_mode = &*current_mode_borrow;
    let sort_mode_borrow = handle.sort_mode.borrow();
    let sort_mode = &*sort_mode_borrow;

    // Clear grid_container (for icons/grid view)
    while let Some(child) = widgets.grid_container.first_child() {
        widgets.grid_container.remove(&child);
    }

    // Clear listbox
    while let Some(child) = widgets.listbox.first_child() {
        widgets.listbox.remove(&child);
    }


    if current_mode == "icons" {
        if sort_mode == "auto" {
            // Flat grid, no headers
            let flowbox = crate::widgets::content_view::create_grid_flowbox(
                handle.entries.clone(),
                handle.nav_callback.clone(),
                handle.selection_callback.clone(),
                &widgets.grid_container,
            );
            
            for (idx, entry) in entries.iter().enumerate() {
                let flow_child = create_flow_child(idx, entry, current_path, nav_callback);
                flowbox.append(&flow_child);
            }
            widgets.grid_container.append(&flowbox);
        } else {
            // Grouping/categories active!
            let get_group_name = |entry: &FileEntry| -> String {
                if sort_mode == "date" {
                    if let Some(modified) = entry.modified {
                        let datetime: chrono::DateTime<chrono::Local> = modified.into();
                        let now = chrono::Local::now();
                        let date_naive = datetime.date_naive();
                        let now_naive = now.date_naive();
                        let date_str = datetime.format(" (%d/%m)").to_string();
                        if date_naive == now_naive {
                            format!("Today{}", date_str)
                        } else if date_naive == now_naive - chrono::Duration::days(1) {
                            format!("Yesterday{}", date_str)
                        } else {
                            let diff = (now_naive - date_naive).num_days();
                            if diff >= 2 && diff <= 7 {
                                format!("{}{}", datetime.format("%A"), date_str)
                            } else if diff > 7 {
                                "Older than a week".to_string()
                            } else {
                                format!("Today{}", date_str)
                            }
                        }
                    } else {
                        "Unknown Date".to_string()
                    }
                } else { // "group"
                    if matches!(entry.file_type, babydra_common::FileType::Directory) {
                        "Folders".to_string()
                    } else {
                        match entry.path.extension() {
                            Some(ext) => format!("{} Files", ext.to_string_lossy().to_uppercase()),
                            None => "Other Files".to_string(),
                        }
                    }
                }
            };

            let mut current_group_name = String::new();
            let mut current_flowbox: Option<gtk4::FlowBox> = None;

            for (idx, entry) in entries.iter().enumerate() {
                let group_name = get_group_name(entry);
                if group_name != current_group_name {
                    current_group_name = group_name.clone();

                    let header_lbl = Label::new(Some(&current_group_name));
                    header_lbl.add_css_class("group-header-label");
                    header_lbl.set_halign(Align::Start);
                    header_lbl.set_margin_top(12);
                    header_lbl.set_margin_bottom(6);
                    header_lbl.set_margin_start(14);
                    header_lbl.set_margin_end(14);
                    widgets.grid_container.append(&header_lbl);

                    let flowbox = crate::widgets::content_view::create_grid_flowbox(
                        handle.entries.clone(),
                        handle.nav_callback.clone(),
                        handle.selection_callback.clone(),
                        &widgets.grid_container,
                    );
                    widgets.grid_container.append(&flowbox);
                    current_flowbox = Some(flowbox);
                }

                if let Some(ref flowbox) = current_flowbox {
                    let flow_child = create_flow_child(idx, entry, current_path, nav_callback);
                    flowbox.append(&flow_child);
                }
            }
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
                explore::format_size(entry.size)
            };
            let lbl_size = Label::new(Some(&size_str));
            lbl_size.set_css_classes(&["list-col-meta"]);
            lbl_size.set_size_request(80, -1);
            lbl_size.set_halign(Align::End);
            lbl_size.set_tooltip_text(Some("Size"));
            item_box.append(&lbl_size);

            // Permissions info
            let perm_str = format!("{:o}", entry.permissions & 0o777);
            let lbl_perm = Label::new(Some(&perm_str));
            lbl_perm.set_css_classes(&["list-col-meta"]);
            lbl_perm.set_size_request(80, -1);
            lbl_perm.set_halign(Align::End);
            lbl_perm.set_tooltip_text(Some("Permissions"));
            item_box.append(&lbl_perm);

            // Modified info
            let mod_str = if let Some(mtime) = entry.modified {
                explore::format_date(mtime)
            } else {
                "--".to_string()
            };
            let lbl_date = Label::new(Some(&mod_str));
            lbl_date.set_css_classes(&["list-col-meta"]);
            lbl_date.set_size_request(140, -1);
            lbl_date.set_halign(Align::End);
            lbl_date.set_tooltip_text(Some("Modified Date"));
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
                crate::widgets::context_menu::show_for_file(
                    widget_clone.upcast_ref(),
                    x,
                    y,
                    target_entry.clone(),
                    cp.clone(),
                    nav.clone(),
                );
            });
            item_box.add_controller(gesture);

            // Add Drag Source to item_box
            let drag_source = gtk4::DragSource::new();
            drag_source.set_actions(gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY);
            let path_clone = entry.path.clone();
            drag_source.connect_prepare(move |_, _, _| {
                let file = gtk4::gio::File::for_path(&path_clone);
                Some(gtk4::gdk::ContentProvider::for_value(&file.to_value()))
            });
            item_box.add_controller(drag_source);

            // If directory, add Drop Target to item_box
            if matches!(entry.file_type, babydra_common::FileType::Directory) {
                let drop_target = gtk4::DropTarget::new(
                    gtk4::gio::File::static_type(),
                    gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY,
                );
                let dest_path = entry.path.clone();
                drop_target.connect_drop(move |_, value, _, _| {
                    if let Ok(file) = value.get::<gtk4::gio::File>() {
                        if let Some(src_path) = file.path() {
                            let dest = dest_path.join(src_path.file_name().unwrap());
                            if src_path != dest {
                                let _ = std::fs::rename(&src_path, &dest);
                            }
                        }
                        return true;
                    }
                    false
                });
                item_box.add_controller(drop_target);
            }

            let list_row = gtk4::ListBoxRow::new();
            list_row.set_child(Some(&item_box));
            list_row.set_property("name", &format!("{}", idx));
            widgets.listbox.append(&list_row);
        }

        // Set header function for grouping in ListBox
        let entries_clone = entries.to_vec();
        let sort_mode_clone = sort_mode.to_string();
        widgets.listbox.set_header_func(move |row, before| {
            if sort_mode_clone == "auto" {
                row.set_header(None::<&gtk4::Widget>);
                return;
            }

            let get_group = |r: &gtk4::ListBoxRow| -> String {
                let idx = r.property::<String>("name").parse::<usize>().unwrap_or(usize::MAX);
                if idx < entries_clone.len() {
                    let entry = &entries_clone[idx];
                    if sort_mode_clone == "date" {
                        if let Some(modified) = entry.modified {
                            let datetime: chrono::DateTime<chrono::Local> = modified.into();
                            let now = chrono::Local::now();
                            let date_naive = datetime.date_naive();
                            let now_naive = now.date_naive();
                            let date_str = datetime.format(" (%d/%m)").to_string();
                            if date_naive == now_naive {
                                format!("Today{}", date_str)
                            } else if date_naive == now_naive - chrono::Duration::days(1) {
                                format!("Yesterday{}", date_str)
                            } else {
                                let diff = (now_naive - date_naive).num_days();
                                if diff >= 2 && diff <= 7 {
                                    format!("{}{}", datetime.format("%A"), date_str)
                                } else if diff > 7 {
                                    "Older than a week".to_string()
                                } else {
                                    format!("Today{}", date_str)
                                }
                            }
                        } else {
                            "Unknown Date".to_string()
                        }
                    } else { // "group"
                        if matches!(entry.file_type, babydra_common::FileType::Directory) {
                            "Folders".to_string()
                        } else {
                            match entry.path.extension() {
                                Some(ext) => format!("{} Files", ext.to_string_lossy().to_uppercase()),
                                None => "Other Files".to_string(),
                            }
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
