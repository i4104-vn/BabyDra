//! Header view with title and status badge for the recording popover.

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Label, Orientation};

/// Creates the recording popover header containing icon, title, and status badge.
pub fn create_popover_header() -> (GtkBox, Label) {
    let header = GtkBox::new(Orientation::Horizontal, 8);
    header.add_css_class("recording-popover-header");

    let header_left = GtkBox::new(Orientation::Horizontal, 6);
    header_left.set_valign(Align::Center);
    let header_icon = babydra_ui_kit::ui::icon::get_icon("camera", 14);
    header_icon.set_valign(Align::Center);
    header_icon.add_css_class("recording-header-icon");
    header_left.append(&header_icon);

    let title = Label::new(Some(&trans("recorder.session_header")));
    title.add_css_class("recording-header-title");
    title.set_valign(Align::Center);
    header_left.append(&title);
    header.append(&header_left);

    let status_badge = Label::new(Some(&trans("recorder.status_idle")));
    status_badge.add_css_class("recording-status-badge");
    status_badge.add_css_class("badge-ready");
    status_badge.set_hexpand(true);
    status_badge.set_halign(Align::End);
    header.append(&status_badge);

    (header, status_badge)
}
