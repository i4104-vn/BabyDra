//! Clipboard handling for the screenshot editor.
//!
//! This is GTK-dependent (window + clipboard + texture), so it lives in the
//! screenshot crate instead of `babydra-core` (which stays GTK-free).

use gtk4::prelude::*;

use babydra_core::models::EditorState;
use babydra_core::services::screenshot::save_cropped_surface;

/// Copies the cropped annotated screenshot to the clipboard using `wl-copy` or
/// a GTK clipboard fallback. Returns `true` on success.
pub fn copy_to_clipboard(state: &EditorState, window: &gtk4::ApplicationWindow) -> bool {
    if let Some(mut surface) = save_cropped_surface(state) {
        let temp_copy_path = "/tmp/babydra-screenshot-copy.png";

        if let Ok(mut file) = std::fs::File::create(temp_copy_path) {
            if surface.write_to_png(&mut file).is_ok() {
                if let Ok(file_in) = std::fs::File::open(temp_copy_path) {
                    let status = std::process::Command::new("wl-copy")
                        .args(["-t", "image/png"])
                        .stdin(file_in)
                        .status();

                    if let Ok(s) = status {
                        if s.success() {
                            notify_copied();
                            return true;
                        }
                    }
                }
            }
        }

        let w = surface.width();
        let h = surface.height();
        let stride = surface.stride();
        if let Ok(data) = surface.data() {
            let pixbuf = gdk_pixbuf::Pixbuf::from_mut_slice(
                data.to_vec(),
                gdk_pixbuf::Colorspace::Rgb,
                true,
                8,
                w,
                h,
                stride,
            );

            let texture = gtk4::gdk::Texture::for_pixbuf(&pixbuf);
            let clipboard = window.upcast_ref::<gtk4::Widget>().display().clipboard();
            clipboard.set_texture(&texture);

            notify_copied();
            return true;
        }
    }
    false
}

fn notify_copied() {
    let notif_title = babydra_core::i18n::trans("screenshot.copied_title");
    let notif_msg = babydra_core::i18n::trans("screenshot.copied_msg");
    babydra_core::send_notification(&notif_title, &notif_msg);
}
