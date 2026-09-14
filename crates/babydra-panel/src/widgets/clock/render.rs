//! UI construction for the clock panel button.

use gtk4::prelude::*;

/// Builds the clock button widget in the panel.
pub fn build_clock_ui() -> (gtk4::Button, gtk4::Label, gtk4::Widget, gtk4::Box) {
    let clock_button = gtk4::Button::new();
    clock_button.add_css_class("panel-clock-btn");

    let clock_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    clock_box.set_valign(gtk4::Align::Center);
    clock_box.set_halign(gtk4::Align::Center);

    let overlay = gtk4::Overlay::new();
    overlay.set_valign(gtk4::Align::Center);
    overlay.set_halign(gtk4::Align::Center);

    let bell_icon = babydra_ui_kit::ui::icon::get_icon("bell", 14);
    bell_icon.add_css_class("clock-bell-icon");
    overlay.set_child(Some(&bell_icon));

    let red_dot = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    red_dot.add_css_class("clock-bell-dot");
    red_dot.set_valign(gtk4::Align::Start);
    red_dot.set_halign(gtk4::Align::End);
    overlay.add_overlay(&red_dot);

    let bell_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    bell_box.add_css_class("clock-bell-box");
    bell_box.set_valign(gtk4::Align::Center);
    bell_box.set_halign(gtk4::Align::Center);
    bell_box.append(&overlay);

    let clock_label = gtk4::Label::new(None);
    clock_label.add_css_class("panel-clock");

    clock_box.append(&clock_label);
    clock_box.append(&bell_box);

    clock_button.set_child(Some(&clock_box));

    (
        clock_button,
        clock_label,
        red_dot.upcast::<gtk4::Widget>(),
        bell_box,
    )
}
