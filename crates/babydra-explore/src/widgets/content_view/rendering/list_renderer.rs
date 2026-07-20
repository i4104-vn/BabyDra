use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align};
use babydra_common::FileEntry;
use babydra_common::ContentViewHandle;

/// Renders entries as list rows in the ListBox.
pub async fn render_list_view(
    handle_c: &ContentViewHandle,
    widgets: &babydra_common::ContentViewWidgets,
    entries: &[FileEntry],
    current_path: &std::path::PathBuf,
    start_path: &std::path::PathBuf,
    gen: u64,
    sort_mode: &str,
    nav_callback: &std::rc::Rc<dyn Fn(std::path::PathBuf)>,
    selected_paths: std::rc::Rc<std::cell::RefCell<Vec<std::path::PathBuf>>>,
) {
    let mut counter = 0;
    for (idx, entry) in entries.iter().enumerate() {
        if *handle_c.current_path.borrow() != *start_path || *handle_c.render_generation.borrow() != gen {
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
    if *handle_c.current_path.borrow() == *start_path && *handle_c.render_generation.borrow() == gen {
        let entries_clone = entries.to_vec();
        let sort_mode_clone = sort_mode.to_string();
        widgets.listbox.set_header_func(move |row, before| {
            if sort_mode_clone == "auto" {
                row.set_header(None::<&gtk4::Widget>);
                return;
            }

            let get_group = |r: &gtk4::ListBoxRow| -> String {
                let path_str = r.widget_name();
                let path = std::path::Path::new(path_str.as_str());
                if let Some(entry) = entries_clone.iter().find(|e| e.path == path) {
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
