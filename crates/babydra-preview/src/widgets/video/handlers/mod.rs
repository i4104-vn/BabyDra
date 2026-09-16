use babydra_core::models::preview::VideoState;
use crate::widgets::video::render::VideoViewerUi;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub mod autohide;
pub mod keys;
pub mod playback;
pub mod speed;
pub mod timeline;
pub mod volume;

pub use autohide::setup_autohide_controls;
pub use keys::setup_key_controller;
pub use playback::setup_playback_controls;
pub use speed::setup_speed_controls;
pub use timeline::setup_timeline_controls;
pub use volume::setup_volume_controls;

/// Sets up all interactive listeners and controllers for the video viewer.
pub fn setup_video_handlers(
    state: &Rc<RefCell<VideoState>>,
    ui: &VideoViewerUi,
    path: PathBuf,
    file_size_bytes: u64,
) {
    let is_popover_open = Rc::new(std::cell::Cell::new(false));

    setup_playback_controls(state, ui);
    setup_timeline_controls(state, ui, file_size_bytes);
    setup_volume_controls(state, ui);
    setup_speed_controls(state, ui, &is_popover_open);
    setup_key_controller(ui, path);
    setup_autohide_controls(state, ui, is_popover_open);
}
