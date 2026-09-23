use babydra_core::DesktopApp;
use gtk4::prelude::*;

pub const ITEM_SIZE: i32 = 54;
pub const ITEM_SPACING: i32 = 8;
pub const ITEM_PITCH: f64 = (ITEM_SIZE + ITEM_SPACING) as f64; // 62.0px
pub const SPACER_HEIGHT: i32 = 2 * ITEM_SIZE + ITEM_SPACING; // 116px (so 116px + 8px box spacing = 124px)

pub fn build_apps_list(apps: &[DesktopApp]) -> (gtk4::Box, Vec<gtk4::Button>) {
    let cards_col = gtk4::Box::new(gtk4::Orientation::Vertical, ITEM_SPACING);
    cards_col.add_css_class("switcher-card-deck");
    cards_col.set_halign(gtk4::Align::Center);
    cards_col.set_valign(gtk4::Align::Start);

    // Top spacer ensures item 0 is vertically centered at scroll position 0
    let top_spacer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    top_spacer.set_size_request(ITEM_SIZE, SPACER_HEIGHT);
    cards_col.append(&top_spacer);

    let mut item_buttons = Vec::with_capacity(apps.len());

    for app_item in apps.iter() {
        let btn = create_app_button(app_item);
        cards_col.append(&btn);
        item_buttons.push(btn);
    }

    // Bottom spacer ensures the last item can also be scrolled to the center
    let bottom_spacer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    bottom_spacer.set_size_request(ITEM_SIZE, SPACER_HEIGHT);
    cards_col.append(&bottom_spacer);

    (cards_col, item_buttons)
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
    icon_widget.set_pixel_size(32);
    icon_widget.add_css_class("switcher-card-icon");
    icon_widget.set_valign(gtk4::Align::Center);
    icon_widget.set_halign(gtk4::Align::Center);

    btn.set_child(Some(&icon_widget));
    btn.set_size_request(ITEM_SIZE, ITEM_SIZE);
    btn.set_hexpand(false);
    btn.set_vexpand(false);

    btn
}

