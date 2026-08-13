//! Greeter widget modules and shared UI helpers.

use gtk4::prelude::*;

pub mod top_bar;
pub mod splash;
pub mod login;

/// Path to the file that persists the last successfully logged-in username.
/// Single source of truth shared by the `login` widget and `handlers`.
pub const LAST_USER_FILE: &str = "/tmp/babydra-last-user";

/// Resolves the avatar/logo image path used by splash and login screens.
/// Prefers the system logo installed by `install.sh`; falls back to the
/// in-repo asset for development builds.
pub fn find_avatar_path() -> String {
    let system_logo = babydra_common::get_logo_path();
    let resolved = if system_logo.exists() {
        system_logo.to_string_lossy().to_string()
    } else {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../libs/babydra-common/src/services/logo.png"
        )
        .to_string()
    };
    tracing::info!(target: "babydra-greeter", "Asset loaded: avatar logo resolved to {:?}", resolved);
    resolved
}

/// Builds a scaled-down avatar `Picture` at the requested size.
/// Shared by the splash screen and the login panel to avoid duplicated logic.
pub fn create_avatar_picture(size: i32) -> gtk4::Picture {
    let logo_path = find_avatar_path();
    tracing::info!(target: "babydra-greeter", "Asset loaded: avatar picture from {:?}", logo_path);

    let pic = if let Ok(pixbuf) = gtk4::gdk_pixbuf::Pixbuf::from_file_at_scale(&logo_path, size, size, true) {
        gtk4::Picture::for_pixbuf(&pixbuf)
    } else {
        gtk4::Picture::for_filename(&logo_path)
    };
    pic.set_size_request(size, size);
    pic.set_can_shrink(true);
    pic.set_content_fit(gtk4::ContentFit::ScaleDown);
    pic
}
