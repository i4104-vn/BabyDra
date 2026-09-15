//! Cairo drawing loop and image rendering with hyper interpolation.

use babydra_core::models::preview::ImageState;
use gdk_pixbuf::Pixbuf;
use gtk4::prelude::*;
use gtk4::DrawingArea;
use std::cell::RefCell;
use std::rc::Rc;

/// Sets up the Cairo draw function with caching and high-quality interpolation.
pub fn setup_cairo_draw(state: &Rc<RefCell<ImageState>>, drawing_area: &DrawingArea) {
    let state_draw = state.clone();
    let scaled_cache: Rc<RefCell<Option<(i32, i32, Pixbuf)>>> = Rc::new(RefCell::new(None));
    let scaled_cache_draw = scaled_cache.clone();

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

        // Calculate layout coordinates
        let target_w = (state_ref.img_w * state_ref.scale).round() as i32;
        let target_h = (state_ref.img_h * state_ref.scale).round() as i32;
        if target_w <= 0 || target_h <= 0 {
            return;
        }

        let start_x = (w - target_w as f64) / 2.0 + state_ref.offset_x;
        let start_y = (h - target_h as f64) / 2.0 + state_ref.offset_y;

        let mut cache_borrow = scaled_cache_draw.borrow_mut();
        let needs_rescale = match *cache_borrow {
            Some((cw, ch, _)) => cw != target_w || ch != target_h,
            None => true,
        };

        if needs_rescale {
            if (state_ref.scale - 1.0).abs() < 0.001 {
                *cache_borrow = Some((target_w, target_h, state_ref.pixbuf.clone()));
            } else if let Some(scaled) =
                state_ref
                    .pixbuf
                    .scale_simple(target_w, target_h, gdk_pixbuf::InterpType::Hyper)
            {
                *cache_borrow = Some((target_w, target_h, scaled));
            }
        }

        if let Some((_, _, ref pb)) = *cache_borrow {
            cr.save().unwrap();
            cr.set_source_pixbuf(pb, start_x, start_y);
            cr.source().set_filter(cairo::Filter::Best);
            let _ = cr.paint();
            cr.restore().unwrap();
        } else {
            cr.save().unwrap();
            cr.translate(start_x, start_y);
            cr.scale(state_ref.scale, state_ref.scale);
            cr.set_source_pixbuf(&state_ref.pixbuf, 0.0, 0.0);
            cr.source().set_filter(cairo::Filter::Best);
            let _ = cr.paint();
            cr.restore().unwrap();
        }
    });
}
