use gtk4::prelude::*;

/// Builds the system tray container box.
pub fn build_tray_container() -> gtk4::Box {
    let tray_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    tray_container.add_css_class("panel-tray-box");
    tray_container.set_valign(gtk4::Align::Center);
    tray_container.set_halign(gtk4::Align::Center);
    tray_container
}

/// Builds a single tray icon button.
pub fn build_tray_button(icon_name: &str, title: &str) -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("panel-tray-item-btn");
    btn.set_tooltip_text(Some(title));
    btn.set_valign(gtk4::Align::Center);
    btn.set_halign(gtk4::Align::Center);
    btn.set_receives_default(false);

    let icon = babydra_ui_kit::ui::icon::get_fallback_icon(icon_name, "image-missing");
    icon.set_pixel_size(16);
    btn.set_child(Some(&icon));

    btn
}
