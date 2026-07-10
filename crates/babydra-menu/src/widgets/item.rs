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
    let btn = baby_utils::components::create_icon_label_button(icon_name, label_text, "menu-item");
    if let Some(content_widget) = btn.child() {
        if let Ok(box_layout) = content_widget.downcast::<gtk4::Box>() {
            box_layout.set_halign(gtk4::Align::Start);
            box_layout.set_valign(gtk4::Align::Center);
        }
    }

    let win = window.clone();
    let mb = menu_box.clone();
    let act = action.clone();
    btn.connect_clicked(move |_| {
        close_menu_animated(&win, &mb, Some(act.clone()));
    });

    menu_box.append(&btn);
}

