//! Switcher collection list renderer component.

use gtk4::prelude::*;
use babydra_common::DesktopApp;
use crate::widgets::item::create_app_button;

/// Populates a horizontal list of window switcher preview buttons from the list of running apps.
pub fn build_apps_list(apps: &[DesktopApp]) -> (gtk4::Box, Vec<gtk4::Button>) {
    let icons_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    icons_row.add_css_class("switcher-list-row");
    icons_row.set_halign(gtk4::Align::Center);
    icons_row.set_valign(gtk4::Align::Fill);

    let mut item_buttons = Vec::new();

    for app_item in apps {
        let btn = create_app_button(app_item);
        icons_row.append(&btn);
        item_buttons.push(btn);
    }

    (icons_row, item_buttons)
}

