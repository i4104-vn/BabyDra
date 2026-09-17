//! Helper widget for layout of single configuration rows.

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Label, Orientation};

/// Creates a standardized settings row with icon, title, and target widget.
pub fn setting_row(icon_name: &str, label: &str, widget: &impl IsA<gtk4::Widget>) -> GtkBox {
    let row = GtkBox::new(Orientation::Horizontal, 10);
    row.add_css_class("recording-setting-row");

    let left_box = GtkBox::new(Orientation::Horizontal, 8);
    left_box.set_hexpand(true);
    left_box.set_halign(Align::Start);
    left_box.set_valign(Align::Center);

    let icon = babydra_ui_kit::ui::icon::get_icon(icon_name, 14);
    icon.set_valign(Align::Center);
    icon.add_css_class("recording-setting-icon");
    left_box.append(&icon);

    let label = Label::new(Some(label));
    label.add_css_class("recording-setting-label");
    label.set_valign(Align::Center);
    left_box.append(&label);

    row.append(&left_box);
    widget.as_ref().set_valign(Align::Center);
    row.append(widget);
    row
}
