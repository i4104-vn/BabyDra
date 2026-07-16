use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align};
use babydra_common::FileEntry;
use babydra_common::ContentViewHandle;

use super::helpers::create_flow_child;

/// Refreshes the FlowBox or ListBox view with the latest directories and file entries.
pub fn update_content_view_ui(handle: &ContentViewHandle) {
    let widgets = handle.widgets.clone();
    let entries = handle.entries.borrow().clone();
    let nav_callback = handle.nav_callback.clone();
    let current_path = handle.current_path.borrow().clone();
    let start_path = current_path.clone();
    let current_mode = handle.current_mode.borrow().clone();
    let sort_mode = handle.sort_mode.borrow().clone();
    let selected_paths = handle.selected_paths.clone();
    let handle_c = handle.clone();

    // Increment and capture the render generation
    let gen = {
        let mut g = handle.render_generation.borrow_mut();
        *g += 1;
        *g
    };

    // Reset and show progress bar at start of layout transaction
    widgets.progress_bar.set_visible(true);
    widgets.progress_bar.set_fraction(0.0);

    // Clear grid_container (for icons/grid view)
    while let Some(child) = widgets.grid_container.first_child() {
        widgets.grid_container.remove(&child);
    }

    // Clear listbox
    while let Some(child) = widgets.listbox.first_child() {
        widgets.listbox.remove(&child);
    }

    glib::spawn_future_local(async move {
        if current_mode == "icons" {
            if sort_mode == "auto" {
                // Flat grid, no headers
                let flowbox = crate::widgets::content_view::create_grid_flowbox(
                    handle_c.entries.clone(),
                    handle_c.nav_callback.clone(),
                    handle_c.selection_callback.clone(),
                    &widgets.grid_container,
                    handle_c.current_path.clone(),
                    handle_c.selected_paths.clone(),
                );
                widgets.grid_container.append(&flowbox);
                
                let mut counter = 0;
                for (idx, entry) in entries.iter().enumerate() {
                    if *handle_c.current_path.borrow() != start_path || *handle_c.render_generation.borrow() != gen {
                        return;
                    }
                    
                    let fraction = if entries.is_empty() { 1.0 } else { (idx + 1) as f64 / entries.len() as f64 };
                    handle_c.widgets.progress_bar.set_fraction(fraction);
                    
                    let flow_child = create_flow_child(idx, entry, &current_path, &nav_callback, selected_paths.clone());
                    flowbox.append(&flow_child);
                    
                    counter += 1;
                    if counter >= 40 {
                        counter = 0;
                        glib::timeout_future(std::time::Duration::from_millis(2)).await;
                    }
                }
            } else {
                // Grouping/categories active!
                let get_group_name = |entry: &FileEntry| -> String {
                    babydra_common::get_group_name(entry, &sort_mode)
                };

                let mut current_group_name = String::new();
                let mut current_flowbox: Option<gtk4::FlowBox> = None;
                
                let mut counter = 0;
                for (idx, entry) in entries.iter().enumerate() {
                    if *handle_c.current_path.borrow() != start_path || *handle_c.render_generation.borrow() != gen {
                        return;
                    }

                    let fraction = if entries.is_empty() { 1.0 } else { (idx + 1) as f64 / entries.len() as f64 };
                    handle_c.widgets.progress_bar.set_fraction(fraction);

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
                            handle_c.entries.clone(),
                            handle_c.nav_callback.clone(),
                            handle_c.selection_callback.clone(),
                            &widgets.grid_container,
                            handle_c.current_path.clone(),
                            handle_c.selected_paths.clone(),
                        );
                        widgets.grid_container.append(&flowbox);
                        current_flowbox = Some(flowbox);
                    }

                    if let Some(ref flowbox) = current_flowbox {
                        let flow_child = create_flow_child(idx, entry, &current_path, &nav_callback, selected_paths.clone());
                        flowbox.append(&flow_child);
                    }
                    
                    counter += 1;
                    if counter >= 40 {
                        counter = 0;
                        glib::timeout_future(std::time::Duration::from_millis(2)).await;
                    }
                }
            }
        } else {
            // Render list/details view
            let mut counter = 0;
            for (idx, entry) in entries.iter().enumerate() {
                if *handle_c.current_path.borrow() != start_path || *handle_c.render_generation.borrow() != gen {
                    return;
                }

                let fraction = if entries.is_empty() { 1.0 } else { (idx + 1) as f64 / entries.len() as f64 };
                handle_c.widgets.progress_bar.set_fraction(fraction);

                let target_entry = entry.clone();
                let cp = current_path.clone();
                let nav = nav_callback.clone();
                let sel_paths = selected_paths.clone();
                let list_row = babydra_utils::explore::create_list_row(
                    idx,
                    entry,
                    selected_paths.clone(),
                    nav_callback.clone(),
                    move |widget, x, y| {
                        let mut target_paths = sel_paths.borrow().clone();
                        if !target_paths.contains(&target_entry.path) {
                            target_paths = vec![target_entry.path.clone()];
                        }

                        babydra_utils::explore::context_menu::show_for_file(
                            widget,
                            x,
                            y,
                            target_paths,
                            cp.clone(),
                            nav.clone(),
                        );
                    },
                );
                widgets.listbox.append(&list_row);
                
                counter += 1;
                if counter >= 40 {
                    counter = 0;
                    glib::timeout_future(std::time::Duration::from_millis(2)).await;
                }
            }

            // Set header function for grouping in ListBox
            if *handle_c.current_path.borrow() == start_path && *handle_c.render_generation.borrow() == gen {
                let entries_clone = entries.clone();
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
                            babydra_common::get_group_name(entry, &sort_mode_clone)
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

        // Hide progress bar when layout completes successfully
        if *handle_c.current_path.borrow() == start_path && *handle_c.render_generation.borrow() == gen {
            handle_c.widgets.progress_bar.set_visible(false);
        }
    });
}
