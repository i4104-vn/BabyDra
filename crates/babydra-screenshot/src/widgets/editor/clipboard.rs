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

        let mut png_bytes = Vec::new();
        if surface.write_to_png(&mut png_bytes).is_ok() {
            let _ = std::fs::write(temp_copy_path, &png_bytes);

            let child = std::process::Command::new("wl-copy")
                .args(["-t", "image/png"])
                .stdin(std::process::Stdio::piped())
                .spawn();

            if let Ok(mut c) = child {
                use std::io::Write;
                if let Some(mut stdin) = c.stdin.take() {
                    let _ = stdin.write_all(&png_bytes);
                }
                if let Ok(s) = c.wait() {
                    if s.success() {
                        notify_copied();
                        return true;
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
