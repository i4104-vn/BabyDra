//! Cairo drawing loop and image rendering with hardware-optimized interpolation.

use babydra_core::models::preview::ImageState;
use gtk4::prelude::*;
use gtk4::DrawingArea;
use std::cell::RefCell;
use std::rc::Rc;

/// Sets up the Cairo draw function with fast, hardware-optimized bilinear interpolation.
pub fn setup_cairo_draw(state: &Rc<RefCell<ImageState>>, drawing_area: &DrawingArea) {
    let state_draw = state.clone();

    drawing_area.set_draw_func(move |_area, cr, width, height| {
        let state_ref = state_draw.borrow();
        let w = width as f64;
        let h = height as f64;

        // Draw Theme-Aware Solid Background
        if babydra_ui_kit::ui::theme::is_dark_mode() {
            cr.set_source_rgb(14.0 / 255.0, 14.0 / 255.0, 18.0 / 255.0);
        } else {
            cr.set_source_rgb(245.0 / 255.0, 245.0 / 255.0, 247.0 / 255.0);
        }
        let _ = cr.paint();

        if state_ref.img_w <= 0.0 || state_ref.img_h <= 0.0 || state_ref.scale <= 0.0 {
            return;
        }

        let target_w = state_ref.img_w * state_ref.scale;
        let target_h = state_ref.img_h * state_ref.scale;

        let start_x = (w - target_w) / 2.0 + state_ref.offset_x;
        let start_y = (h - target_h) / 2.0 + state_ref.offset_y;

        if cr.save().is_ok() {
            cr.translate(start_x, start_y);
            cr.scale(state_ref.scale, state_ref.scale);
            cr.set_source_pixbuf(&state_ref.pixbuf, 0.0, 0.0);
            if (state_ref.scale - 1.0).abs() < 0.001 {
                cr.source().set_filter(cairo::Filter::Nearest);
            } else {
                cr.source().set_filter(cairo::Filter::Bilinear);
            }
            let _ = cr.paint();
            let _ = cr.restore();
        }
    });
}
