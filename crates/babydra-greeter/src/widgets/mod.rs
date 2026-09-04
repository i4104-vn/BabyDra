//! Greeter widget modules and shared UI helpers.

use gtk4::prelude::*;

pub mod login;
pub mod splash;
pub mod top_bar;

/// Builds a scaled-down avatar `Image` at the requested size.
/// Shared by the splash screen and the login panel to avoid duplicated logic.
pub fn create_avatar_img(size: i32) -> gtk4::Widget {
    if let Some(bytes) = babydra_core::get_avatar_bytes() {
        if let Some(img) = babydra_ui_kit::ui::image::create_circle_avatar(&bytes, size, Some("avatar-img")) {
            return img;
        }
    }

    babydra_ui_kit::ui::icon::get_logo_png(size).upcast()
}
