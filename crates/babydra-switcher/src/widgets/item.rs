//! Individual switcher card layout component.
//! Renders active application thumbnail previews or falls back to system application icons.

use gtk4::prelude::*;
use babydra_common::desktop::DesktopApp;

/// Creates a card button displaying a window preview screenshot or a placeholder icon.
pub fn create_app_button(app_item: &DesktopApp) -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("switcher-item-btn");

    let app_icon_str = app_item.icon.as_deref().unwrap_or("application-x-executable");

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

    let preview_width = 310;
    let preview_height = 260;

    if let Some(ref path) = screenshot_path {
        let css_provider = gtk4::CssProvider::new();
        let css = format!(
            "button {{ \
                background-image: url('file://{}'); \
                background-size: cover; \
                background-position: center; \
                background-repeat: no-repeat; \
            }}",
            path
        );
        css_provider.load_from_data(&css);
        btn.style_context()
            .add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }

    let overlay = gtk4::Overlay::new();

    let base_widget: gtk4::Widget = if screenshot_path.is_some() {
        let spacer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        spacer.set_size_request(preview_width, preview_height);
        spacer.upcast()
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

    let icon_widget = babydra_utils::ui::icon::get_system_or_file_icon(
        app_icon_str,
        "application-x-executable",
    );
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

    btn
}

/// Creates a fallback placeholder widget displaying a centered application icon.
fn create_placeholder_preview(app_icon_str: &str, width: i32, height: i32) -> gtk4::Widget {
    let placeholder_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    placeholder_box.add_css_class("switcher-item-placeholder");
    placeholder_box.set_size_request(width, height);
    placeholder_box.set_valign(gtk4::Align::Center);
    placeholder_box.set_halign(gtk4::Align::Center);

    let icon_widget = babydra_utils::ui::icon::get_system_or_file_icon(
        app_icon_str,
        "application-x-executable",
    );
    icon_widget.set_pixel_size(48);
    icon_widget.add_css_class("switcher-item-icon");
    icon_widget.set_valign(gtk4::Align::Center);
    icon_widget.set_halign(gtk4::Align::Center);

    placeholder_box.append(&icon_widget);
    placeholder_box.upcast()
}

