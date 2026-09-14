//! Icon resolution helper for desktop notifications and app badges.

use std::path::Path;

/// Resolves a notification or app icon from a file path, URI, or theme icon name.
pub fn resolve_notification_icon(icon_raw: &str, app_key: &str, pixel_size: i32) -> gtk4::Image {
    let raw = icon_raw.trim();
    if !raw.is_empty() {
        if raw.starts_with('/') && Path::new(raw).exists() {
            let img = gtk4::Image::from_file(raw);
            img.set_pixel_size(pixel_size);
            return img;
        }
        let clean = raw.strip_prefix("file://").unwrap_or(raw);
        if clean.starts_with('/') && Path::new(clean).exists() {
            let img = gtk4::Image::from_file(clean);
            img.set_pixel_size(pixel_size);
            return img;
        }
        if let Some(display) = gdk4::Display::default() {
            let theme = gtk4::IconTheme::for_display(&display);
            let clean_name = clean
                .trim_end_matches(|c| c == '.')
                .rsplitn(2, '.')
                .last()
                .unwrap_or(clean);
            if theme.has_icon(clean_name) {
                let img = gtk4::Image::from_icon_name(clean_name);
                img.set_pixel_size(pixel_size);
                return img;
            }
        }
    }
    let app_clean = app_key.to_lowercase();
    if !app_clean.is_empty() && app_clean != "babydra" {
        if let Some(display) = gdk4::Display::default() {
            let theme = gtk4::IconTheme::for_display(&display);
            if theme.has_icon(&app_clean) {
                let img = gtk4::Image::from_icon_name(&app_clean);
                img.set_pixel_size(pixel_size);
                return img;
            }
        }
    }
    babydra_ui_kit::ui::icon::get_logo_png(pixel_size)
}
