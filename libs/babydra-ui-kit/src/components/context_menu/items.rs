//! Individual widget builders for context menu elements (popovers, items, separators, headers, footers).

use crate::ui::icon::get_icon;
use gtk4::prelude::*;
use gtk4::{Align, Box, Button, Label, Orientation, Popover, PositionType, Separator};

/// Creates a new context menu popover positioned at specific `(x, y)` coordinates.
pub fn create_context_menu_popover(parent: &gtk4::Widget, x: f64, y: f64) -> (Popover, Box) {
    let popover = Popover::builder()
        .has_arrow(false)
        .autohide(true)
        .build();
    popover.set_parent(parent);
    popover.add_css_class("context-menu-popover");
    popover.add_css_class("explore-popover");
    popover.add_css_class("desktop-context-menu");

    let rect = gtk4::gdk::Rectangle::new(x as i32, y as i32, 1, 1);
    popover.set_pointing_to(Some(&rect));

    let vbox = Box::new(Orientation::Vertical, 2);
    vbox.set_css_classes(&["context-menu-box"]);
    vbox.set_width_request(200);

    popover.set_child(Some(&vbox));
    (popover, vbox)
}

/// Creates a context menu popover anchored relative to a target widget (e.g. Tray or Header button).
pub fn create_context_menu_for_widget(
    parent: &gtk4::Widget,
    position: PositionType,
) -> (Popover, Box) {
    let popover = Popover::builder()
        .has_arrow(true)
        .autohide(true)
        .position(position)
        .build();
    popover.set_parent(parent);
    popover.add_css_class("context-menu-popover");
    popover.add_css_class("tray-context-menu");

    let vbox = Box::new(Orientation::Vertical, 2);
    vbox.set_css_classes(&["context-menu-box"]);
    vbox.set_width_request(190);

    popover.set_child(Some(&vbox));
    (popover, vbox)
}

/// Creates a standard menu item button with an icon and label.
pub fn create_menu_item(label: &str, icon: &str) -> Button {
    create_menu_item_full(label, icon, None, false, true)
}

/// Creates a menu item button with sensitivity control.
pub fn create_menu_item_sensitive(label: &str, icon: &str, sensitive: bool) -> Button {
    create_menu_item_full(label, icon, None, false, sensitive)
}

/// Creates a menu item button with an icon, label, and shortcut hint (e.g. "Ctrl+C").
pub fn create_menu_item_with_shortcut(label: &str, icon: &str, shortcut: &str) -> Button {
    create_menu_item_full(label, icon, Some(shortcut), false, true)
}

/// Creates a destructive/danger menu item button (e.g. Delete).
pub fn create_menu_item_destructive(label: &str, icon: &str) -> Button {
    create_menu_item_full(label, icon, None, true, true)
}

/// Creates a destructive/danger menu item button with sensitivity control.
pub fn create_menu_item_destructive_sensitive(label: &str, icon: &str, sensitive: bool) -> Button {
    create_menu_item_full(label, icon, None, true, sensitive)
}

/// Creates a full-featured menu item button with optional shortcut, destructive styling, and sensitivity.
pub fn create_menu_item_full(
    label: &str,
    icon: &str,
    shortcut: Option<&str>,
    is_destructive: bool,
    is_sensitive: bool,
) -> Button {
    let hbox = Box::new(Orientation::Horizontal, 8);
    hbox.set_halign(Align::Fill);

    let img = get_icon(icon, 16);
    img.set_pixel_size(16);
    img.set_valign(Align::Center);
    hbox.append(&img);

    let lbl = Label::builder()
        .label(label)
        .halign(Align::Start)
        .hexpand(true)
        .valign(Align::Center)
        .build();
    hbox.append(&lbl);

    if let Some(sc) = shortcut {
        let sc_lbl = Label::builder()
            .label(sc)
            .halign(Align::End)
            .valign(Align::Center)
            .css_classes(vec!["shortcut-label".to_string()])
            .build();
        hbox.append(&sc_lbl);
    }

    let mut css_classes = vec![
        "flat".to_string(),
        "context-menu-item".to_string(),
    ];
    if is_destructive {
        css_classes.push("destructive".to_string());
    }

    let btn = Button::builder()
        .child(&hbox)
        .css_classes(css_classes)
        .halign(Align::Fill)
        .focusable(false)
        .sensitive(is_sensitive)
        .build();

    btn.set_cursor_from_name(Some("pointer"));
    btn
}

/// Creates a horizontal separator for dividing context menu sections.
pub fn create_menu_separator() -> Separator {
    let sep = Separator::new(Orientation::Horizontal);
    sep.add_css_class("context-menu-separator");
    sep.add_css_class("menu-sep");
    sep
}

/// Creates an uppercase group header label for dividing sections in the menu.
pub fn create_menu_group_header(label: &str) -> Label {
    Label::builder()
        .label(label)
        .halign(Align::Start)
        .css_classes(vec!["context-menu-header".to_string(), "group-header-label".to_string()])
        .build()
}

/// Creates a horizontal footer container with a button box for compact quick actions.
pub fn create_footer_container() -> (Box, Box) {
    let footer_container = Box::new(Orientation::Horizontal, 0);
    footer_container.add_css_class("context-menu-footer");
    footer_container.set_halign(Align::Fill);

    let footer_box = Box::new(Orientation::Horizontal, 6);
    footer_box.set_halign(Align::Start);
    footer_box.set_homogeneous(false);

    footer_container.append(&footer_box);
    (footer_container, footer_box)
}

/// Creates a compact icon button for context menu footers.
pub fn create_footer_icon_button(icon: &str, tooltip: &str) -> Button {
    let img = get_icon(icon, 14);
    img.set_pixel_size(14);

    let btn = Button::builder()
        .child(&img)
        .tooltip_text(tooltip)
        .css_classes(vec![
            "flat".to_string(),
            "context-menu-footer-btn".to_string(),
        ])
        .halign(Align::Center)
        .valign(Align::Center)
        .focusable(false)
        .build();

    btn.set_cursor_from_name(Some("pointer"));
    btn
}
