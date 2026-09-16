//! Shared building blocks used by the image and video previewers.

use crate::widgets::window::format_aspect_ratio;
use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, ApplicationWindow, Box, Grid, Label, Orientation, Spinner};
use std::path::Path;

/// Creates the common loading state shown while media is being prepared.
pub fn create_loading_view() -> Box {
    let loading_box = Box::new(Orientation::Vertical, 10);
    loading_box.set_halign(Align::Center);
    loading_box.set_valign(Align::Center);
    loading_box.set_hexpand(true);
    loading_box.set_vexpand(true);
    loading_box.add_css_class("preview-loading-view");

    let spinner = Spinner::new();
    spinner.set_spinning(true);
    spinner.set_size_request(32, 32);
    loading_box.append(&spinner);

    let label = Label::new(Some(&trans("common.pending")));
    label.add_css_class("dim-label");
    loading_box.append(&label);
    loading_box
}

/// Removes all children from a GTK container.
pub fn clear_box(container: &Box) {
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }
}

/// Adds a consistent two-column metadata row to a details grid.
pub fn append_detail_row(grid: &Grid, row: &mut i32, label: &str, value: &str) {
    let label_widget = Label::new(Some(label));
    label_widget.add_css_class("exif-label");
    label_widget.set_halign(Align::Start);
    label_widget.set_hexpand(true);
    grid.attach(&label_widget, 0, *row, 1, 1);

    let value_widget = Label::new(Some(value));
    value_widget.add_css_class("exif-value");
    value_widget.set_halign(Align::End);
    value_widget.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    value_widget.set_max_width_chars(42);
    grid.attach(&value_widget, 1, *row, 1, 1);
    *row += 1;
}

/// Adds the shared title used by image EXIF and video details panels.
pub fn append_details_title(container: &Box, translation_key: &str) {
    let title = Label::new(Some(&trans(translation_key)));
    title.add_css_class("exif-title");
    title.set_halign(Align::Start);
    container.append(&title);
}

/// Builds the standard media dimensions label, including a reduced aspect ratio.
pub fn format_dimensions(width: u32, height: u32) -> String {
    let aspect = format_aspect_ratio(width, height);
    if aspect.is_empty() {
        format!("{width}x{height}")
    } else {
        format!("{width}x{height} ({aspect})")
    }
}

pub fn file_size(path: &Path) -> u64 {
    std::fs::metadata(path).map_or(0, |metadata| metadata.len())
}

pub fn format_speed(speed: f64) -> String {
    if speed.fract().abs() < f64::EPSILON {
        return format!("{speed:.1}x");
    }

    format!("{speed:.2}x")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

/// Displays a localized failure state in a viewer window.
pub fn show_error(window: &ApplicationWindow) {
    let label = Label::new(Some(&trans("preview.failed_load")));
    label.add_css_class("preview-error");
    label.set_halign(Align::Center);
    label.set_valign(Align::Center);
    label.set_hexpand(true);
    label.set_vexpand(true);
    window.set_child(Some(&label));
}
