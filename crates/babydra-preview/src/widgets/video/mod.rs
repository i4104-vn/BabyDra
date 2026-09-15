use babydra_core::models::preview::VideoState;
use babydra_core::services::preview::probe_video;
use gtk4::prelude::*;
use gtk4::Application;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub mod handlers;
pub mod render;

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
    let pause_icon = babydra_ui_kit::ui::icon::get_icon("pause", 16);
    ui.play_pause_btn.set_child(Some(&pause_icon));

    ui.window.present();
}
