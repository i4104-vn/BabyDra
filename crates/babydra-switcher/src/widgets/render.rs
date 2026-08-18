//! Switcher card and list rendering.
//! Renders active application thumbnail previews or falls back to system application icons.

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

/// Creates a card button displaying a window preview screenshot or a placeholder icon.
pub fn create_app_button(app_item: &DesktopApp) -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("stage-manager-item-btn");

    let app_icon_str = app_item
        .icon
        .as_deref()
        .unwrap_or("application-x-executable");

    let mut screenshot_path: Option<String> = None;
    if let Some(hash) = app_item.get_screenshot_hash() {
        let path = format!("/tmp/babydra-switcher-cache/{}.png", hash);
        if std::path::Path::new(&path).exists() {
            screenshot_path = Some(path);
        }
    }
    if screenshot_path.is_none() {
        if let Some(ref app_id) = app_item.app_id {
            let path = format!("/tmp/babydra-switcher-cache/{}.png", app_id);
            if std::path::Path::new(&path).exists() {
                screenshot_path = Some(path);
            }
        }
    }
    if screenshot_path.is_none() && !app_item.exec.is_empty() {
        let path = format!("/tmp/babydra-switcher-cache/{}.png", app_item.exec);
        if std::path::Path::new(&path).exists() {
            screenshot_path = Some(path);
        }
    }

    let preview_width = 200;
    let preview_height = 130;

    let overlay = gtk4::Overlay::new();

    let base_widget: gtk4::Widget = if let Some(ref path) = screenshot_path {
        let picture = gtk4::Picture::for_filename(path);
        picture.set_content_fit(gtk4::ContentFit::Cover);
        picture.set_can_shrink(true);
        picture.set_size_request(preview_width, preview_height);
        picture.add_css_class("switcher-preview-image");
        picture.upcast()
    } else {
        create_placeholder_preview(app_icon_str, preview_width, preview_height)
    };
    overlay.set_child(Some(&base_widget));

    let icon_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    icon_container.add_css_class("switcher-item-icon-container");
    icon_container.set_valign(gtk4::Align::Start);
    icon_container.set_halign(gtk4::Align::Start);
    icon_container.set_margin_top(8);
    icon_container.set_margin_start(8);

    let icon_widget =
        babydra_ui_kit::ui::icon::get_system_or_file_icon(app_icon_str, "application-x-executable");
    icon_widget.set_pixel_size(20);
    icon_widget.add_css_class("switcher-item-icon");
    icon_container.append(&icon_widget);
    overlay.add_overlay(&icon_container);

    let title_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    title_container.add_css_class("switcher-item-title-container");
    title_container.set_valign(gtk4::Align::End);
    title_container.set_halign(gtk4::Align::Fill);

    let display_title = app_item.window_title.as_deref().unwrap_or(&app_item.name);
    let title_label = gtk4::Label::new(Some(display_title));
    title_label.add_css_class("switcher-app-title");
    title_label.set_halign(gtk4::Align::Center);
    title_label.set_hexpand(true);
    title_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    title_label.set_max_width_chars(18);

    title_container.append(&title_label);
    overlay.add_overlay(&title_container);

    btn.set_child(Some(&overlay));
    btn.set_size_request(preview_width, preview_height);
    btn.set_hexpand(false);
    btn.set_vexpand(false);
    btn.set_halign(gtk4::Align::Start);

    btn
}

/// Creates a fallback placeholder widget displaying a centered application icon.
fn create_placeholder_preview(app_icon_str: &str, width: i32, height: i32) -> gtk4::Widget {
    let placeholder_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    placeholder_box.add_css_class("switcher-item-placeholder");
    placeholder_box.set_size_request(width, height);
    placeholder_box.set_halign(gtk4::Align::Fill);
    placeholder_box.set_valign(gtk4::Align::Fill);

    let icon_widget =
        babydra_ui_kit::ui::icon::get_system_or_file_icon(app_icon_str, "application-x-executable");
    icon_widget.set_pixel_size(48);
    icon_widget.add_css_class("switcher-item-icon");
    icon_widget.set_valign(gtk4::Align::Center);
    icon_widget.set_halign(gtk4::Align::Center);

    placeholder_box.append(&icon_widget);
    placeholder_box.upcast()
}
