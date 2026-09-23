use babydra_core::FileEntry;
use gtk4::prelude::*;
use gtk4::{Align, Label};
use std::path::{Path, PathBuf};
use std::{cell::RefCell, rc::Rc};

use crate::widgets::content_view::grid_item::create_flow_child;
use crate::widgets::state::{ContentViewHandle, ContentViewWidgets};

pub struct GridRenderArgs<'a> {
    pub handle_c: &'a ContentViewHandle,
    pub widgets: &'a ContentViewWidgets,
    pub entries: &'a [FileEntry],
    pub current_path: &'a Path,
    pub start_path: &'a Path,
    pub gen: u64,
    pub sort_mode: &'a str,
    pub nav_callback: &'a Rc<dyn Fn(PathBuf)>,
    pub selected_paths: Rc<RefCell<Vec<PathBuf>>>,
}

/// Renders entries as a flat icon grid (no grouping headers).
pub async fn render_flat_grid(args: GridRenderArgs<'_>) {
    let GridRenderArgs {
        handle_c,
        widgets,
        entries,
        current_path,
        start_path,
        gen,
        nav_callback,
        selected_paths,
        ..
    } = args;

    let flowbox = crate::widgets::content_view::create_grid_flowbox(
        handle_c.entries.clone(),
        handle_c.nav_callback.clone(),
        handle_c.selection_callback.clone(),
        &widgets.grid_container,
        handle_c.tab.clone(),
        handle_c.selected_paths.clone(),
    );
    widgets.grid_container.append(&flowbox);

    let mut counter = 0;
    for (idx, entry) in entries.iter().enumerate() {
        if handle_c.tab.borrow().current_path != *start_path
            || *handle_c.render_generation.borrow() != gen
        {
            return;
        }

        let fraction = if entries.is_empty() {
            1.0
        } else {
            (idx + 1) as f64 / entries.len() as f64
        };

        let flow_child = create_flow_child(
            idx,
            entry,
            current_path,
            nav_callback,
            selected_paths.clone(),
        );
        flowbox.append(&flow_child);

        if selected_paths.borrow().contains(&entry.path) {
            flowbox.select_child(&flow_child);
            flow_child.grab_focus();
        }

        // Update the progress bar once per batch to avoid a repaint per item
        counter += 1;
        if counter >= 80 {
            counter = 0;
            handle_c.widgets.progress_bar.set_fraction(fraction);
            glib::timeout_future(std::time::Duration::from_millis(2)).await;
        }
    }
}

/// Renders entries as a grouped icon grid with category headers.
pub async fn render_grouped_grid(args: GridRenderArgs<'_>) {
    let GridRenderArgs {
        handle_c,
        widgets,
        entries,
        current_path,
        start_path,
        gen,
        sort_mode,
        nav_callback,
        selected_paths,
    } = args;

    let get_group_name =
        |entry: &FileEntry| -> String { babydra_core::get_group_name(entry, sort_mode) };

    let mut current_group_name = String::new();
    let mut current_flowbox: Option<gtk4::FlowBox> = None;

    let mut counter = 0;
    for (idx, entry) in entries.iter().enumerate() {
        if handle_c.tab.borrow().current_path != *start_path
            || *handle_c.render_generation.borrow() != gen
        {
            return;
        }

        let fraction = if entries.is_empty() {
            1.0
        } else {
            (idx + 1) as f64 / entries.len() as f64
        };

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
                handle_c.tab.clone(),
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

            if selected_paths.borrow().contains(&entry.path) {
                flowbox.select_child(&flow_child);
                flow_child.grab_focus();
            }
        }

        counter += 1;
        if counter >= 80 {
            counter = 0;
            handle_c.widgets.progress_bar.set_fraction(fraction);
            glib::timeout_future(std::time::Duration::from_millis(2)).await;
        }
    }
}
