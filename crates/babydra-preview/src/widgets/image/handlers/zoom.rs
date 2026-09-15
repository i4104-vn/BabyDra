//! Zoom calculation and coordinate clamping logic for image viewport.

use babydra_core::models::preview::ImageState;
use gtk4::prelude::*;
use gtk4::{DrawingArea, Label};
use std::cell::RefCell;
use std::rc::Rc;

/// Clamps the image position so it cannot be dragged away from the viewport.
pub fn clamp_position(state_ref: &mut ImageState, area_w: f64, area_h: f64) {
    let scaled_w = state_ref.img_w * state_ref.scale;
    let scaled_h = state_ref.img_h * state_ref.scale;

    let limit_x = ((scaled_w - area_w) / 2.0).max(0.0);
    let limit_y = ((scaled_h - area_h) / 2.0).max(0.0);

    state_ref.offset_x = state_ref.offset_x.clamp(-limit_x, limit_x);
    state_ref.offset_y = state_ref.offset_y.clamp(-limit_y, limit_y);
}

/// Updates the zoom percentage label.
pub fn update_zoom_display(state_ref: &ImageState, lbl: &Label) {
    lbl.set_text(&format!("{:.0}%", state_ref.scale * 100.0));
}

/// Fits the image to the viewport and sets the minimum zoom scale.
pub fn fit_to_screen(state_ref: &mut ImageState, area_w: f64, area_h: f64) {
    let scale_x = area_w / state_ref.img_w;
    let scale_y = area_h / state_ref.img_h;
    state_ref.min_scale = scale_x.min(scale_y).min(1.0).max(0.05);
    state_ref.scale = state_ref.min_scale;
    state_ref.offset_x = 0.0;
    state_ref.offset_y = 0.0;
}

/// Applies a zoom delta around the current position, clamped to the allowed range.
pub fn do_zoom(
    state: &Rc<RefCell<ImageState>>,
    area: &DrawingArea,
    lbl: &Label,
    delta: f64,
) {
    let mut state_ref = state.borrow_mut();
    let area_w = area.width() as f64;
    let area_h = area.height() as f64;

    let next_scale = (state_ref.scale + delta).clamp(state_ref.min_scale, 5.0);
    state_ref.scale = next_scale;

    if state_ref.scale <= state_ref.min_scale + 0.001 {
        state_ref.offset_x = 0.0;
        state_ref.offset_y = 0.0;
    } else {
        clamp_position(&mut *state_ref, area_w, area_h);
    }

    update_zoom_display(&state_ref, lbl);
    area.queue_draw();
}
