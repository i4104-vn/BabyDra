//! Topic-scoped handlers for video preview: playback, timeline, volume, speed, and keys.

pub mod keys;
pub mod playback;
pub mod speed;
pub mod timeline;
pub mod volume;

pub use keys::setup_key_controller;
pub use playback::setup_playback_controls;
pub use speed::setup_speed_controls;
pub use timeline::setup_timeline_controls;
pub use volume::setup_volume_controls;

use babydra_core::models::preview::{VideoMetadata, VideoState};
use crate::widgets::video::render::VideoViewerUi;
use std::cell::RefCell;
use std::rc::Rc;

/// Sets up all interactive listeners and controllers for the video viewer.
pub fn setup_video_handlers(
    state: &Rc<RefCell<VideoState>>,
    ui: &VideoViewerUi,
    meta: &VideoMetadata,
) {
    let total_secs = meta.duration_secs.max(1.0);

    setup_playback_controls(state, ui);
    setup_timeline_controls(state, ui, total_secs);
    setup_volume_controls(state, ui);
    setup_speed_controls(state, ui);
    setup_key_controller(ui);
}
