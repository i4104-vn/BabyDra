//! UI layout renderer for file search result list rows.

use gtk4::prelude::*;
use std::path::Path;

/// Constructs a horizontal list item representing a found file with an appropriate type icon.
pub fn build_file_row_ui(path: &Path) -> (gtk4::Button, gtk4::Box, gtk4::Label) {
    let btn = gtk4::Button::new();
    btn.add_css_class("launcher-file-item");
    
    let path_str = path.to_string_lossy().to_string();
    btn.set_tooltip_text(Some(&path_str));
    
    let content_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    content_box.set_valign(gtk4::Align::Center);
    
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    let icon_name = if path.is_dir() {
        "folder".to_string()
    } else {
        match extension.as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" => "image-x-generic".to_string(),
            "pdf" => "document-pdf".to_string(),
            "zip" | "tar" | "gz" | "xz" | "rar" | "7z" => "package-x-generic".to_string(),
            "mp3" | "wav" | "ogg" | "flac" => "audio-x-generic".to_string(),
            "mp4" | "mkv" | "avi" | "mov" => "video-x-generic".to_string(),
            "html" | "htm" | "css" | "js" | "ts" => "text-html".to_string(),
            _ => "text-x-generic".to_string(),
        }
    };
    
    let icon_widget = babydra_common::icon::get_system_or_file_icon(&icon_name, "text-x-generic");
    icon_widget.set_pixel_size(20);
    icon_widget.set_valign(gtk4::Align::Center);
    
    let file_name_str = path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let name_label = gtk4::Label::new(Some(&file_name_str));
    name_label.set_halign(gtk4::Align::Start);
    name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    name_label.set_max_width_chars(35);
    name_label.add_css_class("launcher-file-label");
    name_label.set_valign(gtk4::Align::Center);
    
    content_box.append(&icon_widget);
    content_box.append(&name_label);
    btn.set_child(Some(&content_box));
    
    (btn, content_box, name_label)
}

