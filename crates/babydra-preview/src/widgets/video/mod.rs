//! Video player module entry point.

pub mod handlers;
pub mod render;

use babydra_core::models::preview::VideoState;
use babydra_core::services::preview::probe_video;
use gtk4::prelude::*;
use gtk4::Application;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// Builds and presents the video player window for the given video path.
pub fn build_ui(app: &Application, path: PathBuf) {
    let meta = probe_video(&path);
    let ui = render::build_video_ui(app, &path, &meta);

    let state = Rc::new(RefCell::new(VideoState {
        duration_us: (meta.duration_secs * 1_000_000.0) as i64,
        position_us: 0,
        is_playing: true,
        is_seeking: false,
        volume: 1.0,
        is_muted: false,
        speed: 1.0,
    }));

    handlers::setup_video_handlers(&state, &ui, &meta);

    // Start video playback automatically
    ui.media_file.play();
    ui.play_pause_btn.set_icon_name("media-playback-pause-symbolic");

    ui.window.present();
}
