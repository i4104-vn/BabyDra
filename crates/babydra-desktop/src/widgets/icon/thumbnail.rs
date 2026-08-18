//! Icon and thumbnail resolution for desktop file entries.

use babydra_core::models::explore::{FileEntry, FileType};
use babydra_ui_kit::prelude::*;
use gtk4::prelude::*;
use gtk4::{Align, Box, Orientation, Picture};
use std::path::Path;

struct SendWrapper<T>(pub T);
unsafe impl<T> Send for SendWrapper<T> {}

/// Checks if a file path is a supported image format for async thumbnailing.
pub fn is_image_path(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|ext| {
            matches!(
                ext.to_lowercase().as_str(),
                "png" | "jpg" | "jpeg" | "webp" | "gif" | "svg" | "bmp"
            )
        })
        .unwrap_or(false)
}

/// Builds the visual icon container for a file entry (with async image thumbnailing where applicable).
pub fn build_icon_image_frame(entry: &FileEntry, icon_px: i32) -> Box {
    let icon_frame = Box::new(Orientation::Vertical, 0);
    icon_frame.set_size_request(icon_px, icon_px);
    icon_frame.set_halign(Align::Center);
    icon_frame.set_valign(Align::Center);
    icon_frame.add_css_class("desktop-icon-image");

    if entry.file_type == FileType::Directory {
        let folder_icon = get_system_or_file_icon(&entry.icon_name, "folder");
        folder_icon.set_pixel_size(icon_px);
        folder_icon.set_halign(Align::Center);
        folder_icon.set_valign(Align::Center);
        icon_frame.append(&folder_icon);
        return icon_frame;
    }

    if is_image_path(&entry.path) {
        let overlay = gtk4::Overlay::new();
        overlay.set_size_request(icon_px, icon_px);
        overlay.set_halign(Align::Center);
        overlay.set_valign(Align::Center);

        let fallback_icon = get_system_or_file_icon(&entry.icon_name, "image-x-generic");
        fallback_icon.set_pixel_size(icon_px - 4);
        fallback_icon.set_halign(Align::Center);
        fallback_icon.set_valign(Align::Center);
        overlay.set_child(Some(&fallback_icon));
        icon_frame.append(&overlay);

        let path_clone = entry.path.clone();
        let overlay_c = overlay.clone();
        let thumb_size = icon_px;

        glib::spawn_future_local(async move {
            let res = tokio::task::spawn_blocking(move || {
                babydra_core::load_cropped_square_pixbuf(&path_clone, thumb_size)
                    .map(SendWrapper)
            })
            .await;

            if let Ok(Ok(wrapper)) = res {
                let pixbuf = wrapper.0;
                let texture = gtk4::gdk::Texture::for_pixbuf(&pixbuf);
                let picture = Picture::for_paintable(&texture);
                picture.set_size_request(thumb_size, thumb_size);
                picture.set_halign(Align::Center);
                picture.set_valign(Align::Center);
                picture.set_content_fit(gtk4::ContentFit::Cover);
                overlay_c.set_child(Some(&picture));
            }
        });
    } else {
        let fallback = if entry.path.extension().map(|e| e == "desktop").unwrap_or(false) {
            "application-x-executable"
        } else {
            "text-x-generic"
        };

        let icon = get_system_or_file_icon(&entry.icon_name, fallback);
        icon.set_pixel_size(icon_px);
        icon.set_halign(Align::Center);
        icon.set_valign(Align::Center);
        icon_frame.append(&icon);
    }

    icon_frame
}
