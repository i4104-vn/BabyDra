//! Custom loading indicator and card components for BabyDra.
//!
//! Provides an animated loading widget based on `loading.gif` and unified
//! loading cards that occupy the full width and height of pending containers.

use gtk4::prelude::*;

/// Embedded bytes of the BabyDra animated loading GIF.
pub const LOADING_GIF_BYTES: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/ui/loading.gif"));

/// Creates an animated loading spinner widget.
pub fn create_loading_icon(size: i32) -> gtk4::Widget {
    let spinner = gtk4::Spinner::new();
    spinner.set_size_request(size, size);
    spinner.set_halign(gtk4::Align::Center);
    spinner.set_valign(gtk4::Align::Center);
    spinner.start();
    spinner.upcast()
}

/// Creates a loading card that occupies the entire length and width of the
/// container waiting to load, with the animated loading icon positioned in the center.
pub fn create_loading_card(icon_size: i32) -> gtk4::Box {
    let card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    card.add_css_class("settings-card");
    card.add_css_class("loading-card");
    card.set_hexpand(true);
    card.set_vexpand(true);
    card.set_halign(gtk4::Align::Fill);
    card.set_valign(gtk4::Align::Fill);

    let center_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    center_box.set_hexpand(true);
    center_box.set_vexpand(true);
    center_box.set_halign(gtk4::Align::Center);
    center_box.set_valign(gtk4::Align::Center);

    let icon = create_loading_icon(icon_size);
    center_box.append(&icon);

    card.append(&center_box);
    card
}

/// Creates a ListBoxRow placeholder loading card that occupies the full row area.
pub fn create_loading_placeholder_row(icon_size: i32) -> gtk4::ListBoxRow {
    let row = gtk4::ListBoxRow::new();
    row.add_css_class("settings-card-row");
    row.add_css_class("loading-card-row");
    row.set_selectable(false);
    row.set_activatable(false);
    row.set_hexpand(true);
    row.set_vexpand(true);
    row.set_valign(gtk4::Align::Fill);
    row.set_halign(gtk4::Align::Fill);

    let loading_card = create_loading_card(icon_size);
    row.set_child(Some(&loading_card));
    row
}
