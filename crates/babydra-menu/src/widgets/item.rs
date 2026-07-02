//! UI helper functions to build individual context menu rows.

use gtk4::prelude::*;
use std::rc::Rc;
use crate::render::close_menu_animated;

/// Adds a standard hoverable button option row inside the menu container.
/// Automatically executes the assigned callback after closing the window.
pub fn add_menu_item(
    window: &gtk4::ApplicationWindow,
    menu_box: &gtk4::Box,
    label_text: &str,
    icon_name: &str,
    action: Rc<dyn Fn()>,
) {
    let btn = gtk4::Button::new();
    btn.add_css_class("menu-item");

    let item_layout = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    item_layout.set_halign(gtk4::Align::Start);
    item_layout.set_valign(gtk4::Align::Center);

    let icon = babydra_common::icon::get_icon_colored(icon_name, 16, "#ffffff");
    let label = gtk4::Label::new(Some(label_text));
    label.set_halign(gtk4::Align::Start);

    item_layout.append(&icon);
    item_layout.append(&label);
    btn.set_child(Some(&item_layout));

    let win = window.clone();
    let mb = menu_box.clone();
    let act = action.clone();
    btn.connect_clicked(move |_| {
        close_menu_animated(&win, &mb, Some(act.clone()));
    });

    menu_box.append(&btn);
}

