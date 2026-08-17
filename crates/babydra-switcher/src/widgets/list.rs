//! Switcher collection list renderer component.

use crate::widgets::item::create_app_button;
use babydra_core::DesktopApp;
use gtk4::prelude::*;

/// Populates a horizontal list of window switcher preview buttons from the list of running apps.
pub fn build_apps_list(apps: &[DesktopApp]) -> (gtk4::Box, Vec<gtk4::Button>) {
    let icons_column = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    icons_column.add_css_class("stage-manager-list");
    icons_column.set_halign(gtk4::Align::Start);
    icons_column.set_valign(gtk4::Align::Start);

    let mut item_buttons = Vec::new();

    for app_item in apps.iter() {
        let btn = create_app_button(app_item);
        icons_column.append(&btn);
        item_buttons.push(btn);
    }

    (icons_column, item_buttons)
}
