//! Topic-scoped handlers for image preview: canvas, zoom, gestures, and keys.

pub mod canvas;
pub mod gestures;
pub mod keys;
pub mod zoom;

pub use canvas::setup_cairo_draw;
pub use gestures::setup_gestures;
pub use keys::setup_key_controller;

use crate::widgets::image::render::ImageViewerUi;
use babydra_core::models::preview::ImageState;
use std::cell::RefCell;
use std::rc::Rc;

/// Wires up all image handlers (gestures and keyboard shortcuts).
pub fn setup_image_handlers(state: &Rc<RefCell<ImageState>>, ui: &ImageViewerUi) {
    setup_gestures(state, ui);
    setup_key_controller(state, ui);
}
