use crate::widgets::window::create_viewer_window;
use babydra_core::models::preview::VideoState;
use gtk4::prelude::*;
use gtk4::Application;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub mod handlers;
pub mod render;

/// Builds and presents the video player window immediately with a loading placeholder,
/// then mounts the playback engine and controls without blocking startup on ffprobe.
pub fn build_ui(app: &Application, path: PathBuf) {
    let title = path.file_name().unwrap_or_default().to_string_lossy();
    let (window, _) = create_viewer_window(app, &title, 1280, 720);

    let loading_view = render::create_loading_view();
    window.set_child(Some(&loading_view));
    window.present();

    let path_clone = path;
    let window_clone = window;

    glib::idle_add_local_once(move || {
        let size_bytes = std::fs::metadata(&path_clone).map(|m| m.len()).unwrap_or(0);
        let ui = render::build_video_content(&window_clone, &path_clone);

        let state = Rc::new(RefCell::new(VideoState {
            duration_us: 0,
            position_us: 0,
            is_playing: true,
            is_seeking: false,
            volume: 1.0,
            is_muted: false,
            speed: 1.0,
        }));

        handlers::setup_video_handlers(&state, &ui, path_clone, size_bytes);

        // Start video playback automatically
        ui.media_file.play();
        let pause_icon = babydra_ui_kit::ui::icon::get_icon("pause", 16);
        ui.play_pause_btn.set_child(Some(&pause_icon));
    });
}
