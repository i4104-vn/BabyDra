use babydra_core::DesktopApp;
use gtk4::prelude::*;

pub fn build_apps_list(apps: &[DesktopApp]) -> (gtk4::Box, Vec<gtk4::Button>) {
    let cards_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    cards_row.add_css_class("switcher-card-deck");
    cards_row.set_halign(gtk4::Align::Center);
    cards_row.set_valign(gtk4::Align::Center);

    let mut item_buttons = Vec::new();

    for app_item in apps.iter() {
        let btn = create_app_button(app_item);
        cards_row.append(&btn);
        item_buttons.push(btn);
    }

    (cards_row, item_buttons)
}

pub fn create_app_button(app_item: &DesktopApp) -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("switcher-card");

    let app_icon_str = app_item
        .icon
        .as_deref()
        .unwrap_or("application-x-executable");

    let icon_widget =
        babydra_ui_kit::ui::icon::get_fallback_icon(app_icon_str, "application-x-executable");
    icon_widget.set_pixel_size(56);
    icon_widget.add_css_class("switcher-card-icon");
    icon_widget.set_valign(gtk4::Align::Center);
    icon_widget.set_halign(gtk4::Align::Center);

    btn.set_child(Some(&icon_widget));
    btn.set_size_request(88, 88);
    btn.set_hexpand(false);
    btn.set_vexpand(false);

    btn
}
