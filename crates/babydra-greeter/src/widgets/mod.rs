//! Greeter widget modules and shared UI helpers.

use gtk4::prelude::*;

pub mod login;
pub mod splash;
pub mod top_bar;

/// Path to the file that persists the last successfully logged-in username.
/// Single source of truth shared by the `login` widget and `handlers`.
pub const LAST_USER_FILE: &str = "/tmp/babydra-last-user";

pub fn create_logo_picture(size: i32) -> gtk4::Widget {
    let logo_bytes = include_bytes!("../../../../libs/babydra-core/src/services/logo.png");
    let stream = gtk4::gio::MemoryInputStream::from_bytes(&gtk4::glib::Bytes::from(logo_bytes));

    if let Ok(pixbuf) = gtk4::gdk_pixbuf::Pixbuf::from_stream_at_scale(
        &stream,
        size,
        size,
        true,
        gtk4::gio::Cancellable::NONE,
    ) {
        let texture = gtk4::gdk::Texture::for_pixbuf(&pixbuf);
        let img = gtk4::Image::from_paintable(Some(&texture));
        img.set_pixel_size(size);
        img.upcast()
    } else {
        let img = gtk4::Image::new();
        img.set_pixel_size(size);
        img.upcast()
    }
}

/// Builds a scaled-down avatar `Image` at the requested size.
/// Shared by the splash screen and the login panel to avoid duplicated logic.
pub fn create_avatar_picture(size: i32) -> gtk4::Widget {
    if let Some(bytes) = babydra_core::get_avatar_bytes() {
        if let Some(pixbuf) = babydra_core::crop_to_circle_pixbuf(&bytes, size) {
            let texture = gtk4::gdk::Texture::for_pixbuf(&pixbuf);
            let img = gtk4::Image::from_paintable(Some(&texture));
            img.set_pixel_size(size);
            img.add_css_class("avatar-img");
            img.set_halign(gtk4::Align::Center);
            img.set_valign(gtk4::Align::Center);
            return img.upcast();
        }
    }

    create_logo_picture(size)
}
