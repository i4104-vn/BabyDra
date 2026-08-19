use crate::widgets::state::ContentViewHandle;
use babydra_core::FileEntry;
use gtk4::prelude::*;
use gtk4::{Align, Label};

use crate::widgets::content_view::grid_item::create_flow_child;

/// Renders entries as a flat icon grid (no grouping headers).
pub async fn render_flat_grid(
    handle_c: &ContentViewHandle,
    widgets: &crate::widgets::state::ContentViewWidgets,
    entries: &[FileEntry],
    current_path: &std::path::PathBuf,
    start_path: &std::path::PathBuf,
    gen: u64,
    nav_callback: &std::rc::Rc<dyn Fn(std::path::PathBuf)>,
    selected_paths: std::rc::Rc<std::cell::RefCell<Vec<std::path::PathBuf>>>,
) {
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
        if *handle_c.current_path.borrow() != *start_path
            || *handle_c.render_generation.borrow() != gen
        {
            return;
        }

        let fraction = if entries.is_empty() {
            1.0
        } else {
            (idx + 1) as f64 / entries.len() as f64
        };
        handle_c.widgets.progress_bar.set_fraction(fraction);

        let flow_child = create_flow_child(
            idx,
            entry,
            current_path,
            nav_callback,
            selected_paths.clone(),
        );
        flowbox.append(&flow_child);

        counter += 1;
        if counter >= 80 {
            counter = 0;
            glib::timeout_future(std::time::Duration::from_millis(2)).await;
        }
    }
}

/// Renders entries as a grouped icon grid with category headers.
pub async fn render_grouped_grid(
    handle_c: &ContentViewHandle,
    widgets: &crate::widgets::state::ContentViewWidgets,
    entries: &[FileEntry],
    current_path: &std::path::PathBuf,
    start_path: &std::path::PathBuf,
    gen: u64,
    sort_mode: &str,
    nav_callback: &std::rc::Rc<dyn Fn(std::path::PathBuf)>,
    selected_paths: std::rc::Rc<std::cell::RefCell<Vec<std::path::PathBuf>>>,
) {
    let get_group_name =
        |entry: &FileEntry| -> String { babydra_core::get_group_name(entry, sort_mode) };

    let mut current_group_name = String::new();
    let mut current_flowbox: Option<gtk4::FlowBox> = None;

    let mut counter = 0;
    for (idx, entry) in entries.iter().enumerate() {
        if *handle_c.current_path.borrow() != *start_path
            || *handle_c.render_generation.borrow() != gen
        {
            return;
        }

        let fraction = if entries.is_empty() {
            1.0
        } else {
            (idx + 1) as f64 / entries.len() as f64
        };
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
            let flow_child = create_flow_child(
                idx,
                entry,
                current_path,
                nav_callback,
                selected_paths.clone(),
            );
            flowbox.append(&flow_child);
        }

        counter += 1;
        if counter >= 80 {
            counter = 0;
            glib::timeout_future(std::time::Duration::from_millis(2)).await;
        }
    }
}
