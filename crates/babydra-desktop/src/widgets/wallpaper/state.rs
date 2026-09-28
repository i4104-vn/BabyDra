//! Encapsulated wallpaper state for active surfaces, media files, and transitions.

use gtk4::cairo;
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

pub type PendingLivePlay = (
    PathBuf,
    babydra_core::wallpaper::WallpaperMode,
    Option<gtk4::MediaFile>,
);

#[derive(Clone)]
pub struct WallpaperState {
    pub current_wp_path: Rc<RefCell<Option<PathBuf>>>,
    pub current_wp_mode: Rc<RefCell<babydra_core::wallpaper::WallpaperMode>>,
    pub active_media_file: Rc<RefCell<Option<gtk4::MediaFile>>>,
    pub gif_source_id: Rc<RefCell<Option<glib::SourceId>>>,
    pub recycle_source_id: Rc<RefCell<Option<glib::SourceId>>>,
    pub old_surface: Rc<RefCell<Option<cairo::ImageSurface>>>,
    pub current_surface: Rc<RefCell<Option<cairo::ImageSurface>>>,
    pub transition_progress: Rc<Cell<f64>>,
    pub is_animating: Rc<Cell<bool>>,
    pub active_start_time: Rc<Cell<Option<i64>>>,
    pub ripple_origin: Rc<Cell<(f64, f64)>>,
    pub pending_live_play: Rc<RefCell<Option<PendingLivePlay>>>,
}

impl WallpaperState {
    pub fn new() -> Self {
        Self {
            current_wp_path: Rc::new(RefCell::new(None)),
            current_wp_mode: Rc::new(RefCell::new(babydra_core::wallpaper::WallpaperMode::Static)),
            active_media_file: Rc::new(RefCell::new(None)),
            gif_source_id: Rc::new(RefCell::new(None)),
            recycle_source_id: Rc::new(RefCell::new(None)),
            old_surface: Rc::new(RefCell::new(None)),
            current_surface: Rc::new(RefCell::new(None)),
            transition_progress: Rc::new(Cell::new(1.0)),
            is_animating: Rc::new(Cell::new(false)),
            active_start_time: Rc::new(Cell::new(None)),
            ripple_origin: Rc::new(Cell::new((1.0, 0.0))),
            pending_live_play: Rc::new(RefCell::new(None)),
        }
    }

    /// Stops any active live video or GIF playback safely.
    pub fn stop_active_media(&self) {
        if let Some(id) = self.gif_source_id.borrow_mut().take() {
            id.remove();
        }
        if let Some(id) = self.recycle_source_id.borrow_mut().take() {
            id.remove();
        }
        if let Some(mf) = self.active_media_file.borrow_mut().take() {
            super::player::safely_pause_media_file(&mf);
        }
        if let Some((_, _, Some(mf))) = self.pending_live_play.borrow_mut().take() {
            super::player::safely_pause_media_file(&mf);
        }
    }
}
