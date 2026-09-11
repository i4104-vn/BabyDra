//! Item row widget for the clipboard popover.

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, GestureClick, Label, Orientation};

#[derive(Clone)]
pub struct ClipboardItemRow {
    pub container: GtkBox,
    pub num_label: Label,
    pub icon_holder: GtkBox,
    pub content_box: GtkBox,
    pub text_label: Label,
    pub more_label: Label,
    pub click_gesture: GestureClick,
}

impl Default for ClipboardItemRow {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardItemRow {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 8);
        container.add_css_class("clipboard-popover-item");
        container.set_halign(Align::Fill);
        container.set_hexpand(true);
        container.set_valign(Align::Center);
        container.set_cursor_from_name(Some("pointer"));

        let num_label = Label::new(None);
        num_label.add_css_class("clipboard-popover-num");
        num_label.set_valign(Align::Start);
        num_label.set_margin_top(2);
        container.append(&num_label);

        let icon_holder = GtkBox::new(Orientation::Horizontal, 0);
        icon_holder.add_css_class("clipboard-popover-icon");
        icon_holder.set_valign(Align::Start);
        icon_holder.set_margin_top(2);
        container.append(&icon_holder);

        let content_box = GtkBox::new(Orientation::Vertical, 2);
        content_box.set_hexpand(true);
        content_box.set_halign(Align::Fill);
        content_box.set_valign(Align::Center);

        let text_label = Label::new(None);
        text_label.add_css_class("clipboard-popover-preview");
        text_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        text_label.set_max_width_chars(36);
        text_label.set_xalign(0.0);
        text_label.set_hexpand(true);
        text_label.set_wrap(false);
        content_box.append(&text_label);

        let more_label = Label::new(None);
        more_label.add_css_class("clipboard-popover-more");
        more_label.set_xalign(0.0);
        more_label.set_visible(false);
        content_box.append(&more_label);

        container.append(&content_box);

        let click_gesture = GestureClick::new();
        container.add_controller(click_gesture.clone());

        Self {
            container,
            num_label,
            icon_holder,
            content_box,
            text_label,
            more_label,
            click_gesture,
        }
    }
}
