//! UI layout renderer for file search result list rows.

use gtk4::prelude::*;
use std::path::Path;

/// Constructs a horizontal list item representing a found file with an appropriate type icon.
pub fn build_file_row_ui(path: &Path) -> (gtk4::Button, gtk4::Box, gtk4::Label) {
    let btn = gtk4::Button::new();
    btn.add_css_class("launcher-list-item"); // Use unified item class
    btn.set_cursor_from_name(Some("pointer"));

    let path_str = path.to_string_lossy().to_string();
    btn.set_tooltip_text(Some(&path_str));

    let content_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    content_box.set_valign(gtk4::Align::Center);

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    let (icon_name, badge_text, badge_class) = if path.is_dir() {
        ("folder".to_string(), "Folder", "folder")
    } else {
        let name = match extension.as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" => "image-x-generic".to_string(),
            "pdf" => "document-pdf".to_string(),
            "zip" | "tar" | "gz" | "xz" | "rar" | "7z" => "package-x-generic".to_string(),
            "mp3" | "wav" | "ogg" | "flac" => "audio-x-generic".to_string(),
            "mp4" | "mkv" | "avi" | "mov" => "video-x-generic".to_string(),
            "html" | "htm" | "css" | "js" | "ts" => "text-html".to_string(),
            _ => "text-x-generic".to_string(),
        };
        (name, "File", "file")
    };

    // Icon wrapper
    let icon_wrapper = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    icon_wrapper.add_css_class("app-icon-wrapper");
    icon_wrapper.set_size_request(42, 42);
    icon_wrapper.set_halign(gtk4::Align::Center);
    icon_wrapper.set_valign(gtk4::Align::Center);

    let icon_widget =
        babydra_ui_kit::ui::icon::get_system_or_file_icon(&icon_name, "text-x-generic");
    icon_widget.set_pixel_size(24);
    icon_widget.set_halign(gtk4::Align::Center);
    icon_widget.set_valign(gtk4::Align::Center);
    icon_wrapper.append(&icon_widget);

    // Info box
    let info_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    info_box.add_css_class("app-info");
    info_box.set_hexpand(true);
    info_box.set_valign(gtk4::Align::Center);

    // Title row
    let title_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    title_row.add_css_class("app-title-row");

    let file_name_str = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let name_label = gtk4::Label::new(Some(&file_name_str));
    name_label.set_halign(gtk4::Align::Start);
    name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    name_label.set_max_width_chars(25);
    name_label.add_css_class("app-title");

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);

    let badge_label = gtk4::Label::new(Some(badge_text));
    badge_label.add_css_class("item-badge");
    badge_label.add_css_class(badge_class);

    title_row.append(&name_label);
    title_row.append(&spacer);
    title_row.append(&badge_label);

    // Description row
    let desc_label = gtk4::Label::new(Some(&path_str));
    desc_label.set_halign(gtk4::Align::Start);
    desc_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    desc_label.add_css_class("app-desc");

    info_box.append(&title_row);
    info_box.append(&desc_label);

    content_box.append(&icon_wrapper);
    content_box.append(&info_box);
    btn.set_child(Some(&content_box));

    (btn, content_box, name_label)
}
