use gtk4::prelude::*;
use gtk4::{Box, Button, Orientation, Align, Label, Popover};

pub fn create_menu_popover(parent: &gtk4::Widget, x: f64, y: f64) -> (Popover, Box) {
    let popover = Popover::builder()
        .has_arrow(false)
        .autohide(true)
        .build();
    popover.set_parent(parent);
    popover.add_css_class("explore-popover");

    // Position popover at click coordinates
    let rect = gtk4::gdk::Rectangle::new(x as i32, y as i32, 1, 1);
    popover.set_pointing_to(Some(&rect));

    let vbox = Box::new(Orientation::Vertical, 2);
    vbox.set_css_classes(&["context-menu-box"]);
    vbox.set_width_request(200);

    popover.set_child(Some(&vbox));

    (popover, vbox)
}

pub fn create_menu_button(label: &str, icon: &str) -> Button {
    let hbox = Box::new(Orientation::Horizontal, 8);
    let img = crate::ui::icon::get_icon(icon, 16);
    img.set_pixel_size(16);
    let lbl = Label::builder()
        .label(label)
        .halign(Align::Start)
        .build();

    hbox.append(&img);
    hbox.append(&lbl);

    Button::builder()
        .child(&hbox)
        .css_classes(vec!["flat".to_string(), "context-menu-item".to_string()])
        .halign(Align::Fill)
        .build()
}

pub fn create_footer_icon_button(icon: &str, tooltip: &str) -> Button {
    let img = crate::ui::icon::get_icon(icon, 16);
    img.set_pixel_size(16);

    Button::builder()
        .child(&img)
        .tooltip_text(tooltip)
        .css_classes(vec!["flat".to_string(), "context-menu-footer-btn".to_string()])
        .halign(Align::Center)
        .valign(Align::Center)
        .build()
}
