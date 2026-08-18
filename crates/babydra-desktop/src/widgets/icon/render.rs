//! Desktop icon widget rendering for files and shortcuts on ~/Desktop.

use super::thumbnail::build_icon_frame;
use babydra_core::models::explore::FileEntry;
use gtk4::prelude::*;
use gtk4::{Align, Box, Label, Orientation};

/// Creates a desktop icon widget for a single file entry.
pub fn create_desktop_icon(
    entry: &FileEntry,
    icon_size: u32,
    is_selected: bool,
) -> Box {
    let container = Box::new(Orientation::Vertical, 2);
    container.set_css_classes(&["desktop-icon"]);
    if is_selected {
        container.add_css_class("selected");
    }
    if entry.is_hidden || entry.display_name.starts_with('.') {
        container.add_css_class("hidden-item");
    }

    let icon_px = (icon_size as i32).clamp(32, 72);
    let card_w = (icon_px + 36).clamp(80, 110);
    let card_h = icon_px + 38;

    container.set_size_request(card_w, card_h);
    container.set_halign(Align::Center);
    container.set_valign(Align::Center);

    // 1. Icon Container with thumbnail / theme icon
    let icon_frame = build_icon_frame(entry, icon_px);
    container.append(&icon_frame);

    // 2. Multiline Label with text-shadow
    let label = Label::builder()
        .label(&entry.display_name)
        .max_width_chars(11)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .lines(2)
        .wrap(true)
        .justify(gtk4::Justification::Center)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();
    label.add_css_class("desktop-icon-label");

    container.append(&label);

    container
}
