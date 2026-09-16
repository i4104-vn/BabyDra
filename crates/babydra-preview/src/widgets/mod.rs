use babydra_core::models::preview::MediaKind;
use gtk4::Application;
use std::path::PathBuf;

pub mod image;
pub mod playback;
pub mod utils;
pub mod video;
pub mod window;

/// Application dispatcher: inspects the media file and launches either the image or video viewer.
pub fn build_ui(app: &Application, path: PathBuf) {
    match MediaKind::detect(&path) {
        MediaKind::Video => {
            video::build_ui(app, path);
        }
        MediaKind::Image | MediaKind::Unsupported => {
            image::build_ui(app, path);
        }
    }
}
