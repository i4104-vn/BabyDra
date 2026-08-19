//! Individual widget builders for context menu elements (popovers, items, separators, headers, footers).

use crate::ui::icon::get_icon;
use gtk4::prelude::*;
use gtk4::{Align, Box, Button, Image, Label, Orientation, Popover, PositionType, Separator};

/// Spacing between the icon/checkmark and the label inside a menu row.
const MENU_ROW_SPACING: i32 = 8;

/// Creates a new context menu popover positioned at specific `(x, y)` coordinates.
pub fn create_menu_popover(parent: &gtk4::Widget, x: f64, y: f64) -> (Popover, Box) {
    let popover = Popover::builder().has_arrow(false).autohide(true).build();
    popover.set_parent(parent);
    popover.add_css_class("context-menu-popover");
    popover.add_css_class("explore-popover");
    popover.add_css_class("desktop-context-menu");

    let rect = gtk4::gdk::Rectangle::new(x as i32, y as i32, 1, 1);
    popover.set_pointing_to(Some(&rect));

    let vbox = create_menu_box(200);
    popover.set_child(Some(&vbox));
    (popover, vbox)
}

/// Creates a context menu popover anchored relative to a target widget (e.g. Tray or Header button).
pub fn create_menu_for(parent: &gtk4::Widget, position: PositionType) -> (Popover, Box) {
    let popover = Popover::builder()
        .has_arrow(true)
        .autohide(true)
        .position(position)
        .build();
    popover.set_parent(parent);
    popover.add_css_class("context-menu-popover");
    popover.add_css_class("tray-context-menu");

    let vbox = create_menu_box(190);
    popover.set_child(Some(&vbox));
    (popover, vbox)
}

/// Builds a standard vertical menu container with the given minimum width.
pub(super) fn create_menu_box(width: i32) -> Box {
    let vbox = Box::new(Orientation::Vertical, 2);
    vbox.set_css_classes(&["context-menu-box"]);
    vbox.set_width_request(width);
    vbox
}

/// Creates the popover that hosts a submenu, attached to its parent item button.
pub(super) fn create_submenu_popover(
    parent: &impl IsA<gtk4::Widget>,
    extra_class: &str,
) -> Popover {
    let sub_popover = Popover::builder()
        .has_arrow(true)
        .autohide(false)
        .position(PositionType::Right)
        .build();
    sub_popover.set_parent(parent);
    sub_popover.add_css_class("context-menu-popover");
    sub_popover.add_css_class(extra_class);
    sub_popover
}

/// Creates a standard menu item button with an icon and label.
pub fn create_menu_item(label: &str, icon: &str) -> Button {
    create_menu_full(label, icon, None, false, true)
}

/// Creates a menu item button with sensitivity control.
pub fn create_menu_sens(label: &str, icon: &str, sensitive: bool) -> Button {
    create_menu_full(label, icon, None, false, sensitive)
}

/// Creates a menu item button with an icon, label, and shortcut hint (e.g. "Ctrl+C").
pub fn create_menu_shortcut(label: &str, icon: &str, shortcut: &str) -> Button {
    create_menu_full(label, icon, Some(shortcut), false, true)
}

/// Creates a destructive/danger menu item button (e.g. Delete).
pub fn create_danger_item(label: &str, icon: &str) -> Button {
    create_menu_full(label, icon, None, true, true)
}

/// Creates a destructive/danger menu item button with sensitivity control.
pub fn create_danger_btn(label: &str, icon: &str, sensitive: bool) -> Button {
    create_menu_full(label, icon, None, true, sensitive)
}

/// Creates a full-featured menu item button with optional shortcut, destructive styling, and sensitivity.
pub fn create_menu_full(
    label: &str,
    icon: &str,
    shortcut: Option<&str>,
    is_destructive: bool,
    is_sensitive: bool,
) -> Button {
    let hbox = menu_row();
    hbox.append(&menu_icon(icon, 16));
    hbox.append(&menu_label(label));

    if let Some(sc) = shortcut {
        hbox.append(&shortcut_label(sc));
    }

    build_menu_button(hbox, is_destructive, is_sensitive)
}

/// Creates a text-only menu item button (no leading icon), with optional checkmark, destructive style, and sensitivity.
pub fn create_menu_text(
    label: &str,
    is_checked: bool,
    is_destructive: bool,
    is_sensitive: bool,
) -> Button {
    let hbox = menu_row();
    if is_checked {
        hbox.append(&menu_icon("check", 14));
    }
    hbox.append(&menu_label(label));

    build_menu_button(hbox, is_destructive, is_sensitive)
}

/// Creates a submenu menu item button with optional leading icon and forward arrow icon on the right.
pub fn create_submenu_item(label: &str, icon: Option<&str>, is_sensitive: bool) -> Button {
    let hbox = menu_row();
    if let Some(icon_name) = icon {
        hbox.append(&menu_icon(icon_name, 16));
    }
    hbox.append(&menu_label(label));

    let arrow = menu_icon("forward", 12);
    arrow.set_halign(Align::End);
    hbox.append(&arrow);

    build_menu_button(hbox, false, is_sensitive)
}

/// Creates a horizontal separator for dividing context menu sections.
pub fn create_menu_sep() -> Separator {
    let sep = Separator::new(Orientation::Horizontal);
    sep.add_css_class("context-menu-separator");
    sep.add_css_class("menu-sep");
    sep
}

/// Creates an uppercase group header label for dividing sections in the menu.
pub fn create_group_header(label: &str) -> Label {
    Label::builder()
        .label(label)
        .halign(Align::Start)
        .css_classes(vec![
            "context-menu-header".to_string(),
            "group-header-label".to_string(),
        ])
        .build()
}

/// Creates a horizontal footer container with a button box for compact quick actions.
pub fn create_footer_box() -> (Box, Box) {
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
pub fn create_footer_btn(icon: &str, tooltip: &str) -> Button {
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

/// Creates a horizontally-fillable menu row container.
fn menu_row() -> Box {
    let hbox = Box::new(Orientation::Horizontal, MENU_ROW_SPACING);
    hbox.set_halign(Align::Fill);
    hbox
}

/// Creates a left-aligned, expanding menu label.
fn menu_label(label: &str) -> Label {
    Label::builder()
        .label(label)
        .halign(Align::Start)
        .hexpand(true)
        .valign(Align::Center)
        .build()
}

/// Creates a menu icon image with standard size and vertical centering.
fn menu_icon(name: &str, size: i32) -> Image {
    let img = get_icon(name, size);
    img.set_pixel_size(size);
    img.set_valign(Align::Center);
    img
}

/// Creates the right-aligned keyboard shortcut hint label.
fn shortcut_label(sc: &str) -> Label {
    Label::builder()
        .label(sc)
        .halign(Align::End)
        .valign(Align::Center)
        .css_classes(vec!["shortcut-label".to_string()])
        .build()
}

/// Wraps a prebuilt menu row into a fully styled context menu button.
fn build_menu_button(hbox: Box, is_destructive: bool, is_sensitive: bool) -> Button {
    let mut css_classes = vec!["flat".to_string(), "context-menu-item".to_string()];
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
