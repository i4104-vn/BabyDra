use crate::widgets::image::render::ImageViewerUi;
use babydra_core::models::preview::ImageState;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub mod autohide;
pub mod canvas;
pub mod gestures;
pub mod keys;
pub mod zoom;

pub use autohide::setup_autohide_controls;
pub use canvas::setup_cairo_draw;
pub use gestures::setup_gestures;
pub use keys::setup_key_controller;

/// Wires up all image handlers (gestures, keyboard shortcuts, and autohide).
pub fn setup_image_handlers(state: &Rc<RefCell<ImageState>>, ui: &ImageViewerUi, path: PathBuf) {
    setup_gestures(state, ui);
    setup_key_controller(state, ui, path);
    setup_autohide_controls(ui);
}
