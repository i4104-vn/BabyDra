//! Directional slide-in / slide-out widget animations.

use gtk4::prelude::*;
use super::easing;

/// Direction axis for a slide transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlideDirection {
    Down,
    Up,
    Left,
    Right,
}

/// Slides a widget into view from the given direction using frame-synced FrameClock timing.
pub fn slide_in(widget: &gtk4::Widget, direction: SlideDirection, distance_px: i32, duration_ms: u64) {
    widget.set_opacity(0.0);
    widget.set_visible(true);

    let original_margin_top = widget.margin_top();
    let original_margin_bottom = widget.margin_bottom();
    let original_margin_start = widget.margin_start();
    let original_margin_end = widget.margin_end();

    match direction {
        SlideDirection::Down => widget.set_margin_top(original_margin_top - distance_px),
        SlideDirection::Up => widget.set_margin_bottom(original_margin_bottom - distance_px),
        SlideDirection::Right => widget.set_margin_start(original_margin_start - distance_px),
        SlideDirection::Left => widget.set_margin_end(original_margin_end - distance_px),
    }

    let start_time = std::cell::Cell::new(0i64);
    let dur_us = duration_ms as i64 * 1000;

    widget.add_tick_callback(move |w, clock| {
        let now = clock.frame_time();
        if start_time.get() == 0 {
            start_time.set(now);
        }
        let elapsed_us = now - start_time.get();
        if elapsed_us >= dur_us {
            w.set_opacity(1.0);
            match direction {
                SlideDirection::Down => w.set_margin_top(original_margin_top),
                SlideDirection::Up => w.set_margin_bottom(original_margin_bottom),
                SlideDirection::Right => w.set_margin_start(original_margin_start),
                SlideDirection::Left => w.set_margin_end(original_margin_end),
            }
            return glib::ControlFlow::Break;
        }

        let t = elapsed_us as f64 / dur_us as f64;
        let eased = easing::ease_out_cubic(t);

        w.set_opacity(eased.min(1.0).max(0.0));
        match direction {
            SlideDirection::Down => {
                let offset = original_margin_top - distance_px;
                let current = offset + (distance_px as f64 * eased) as i32;
                w.set_margin_top(current);
            }
            SlideDirection::Up => {
                let offset = original_margin_bottom - distance_px;
                let current = offset + (distance_px as f64 * eased) as i32;
                w.set_margin_bottom(current);
            }
            SlideDirection::Right => {
                let offset = original_margin_start - distance_px;
                let current = offset + (distance_px as f64 * eased) as i32;
                w.set_margin_start(current);
            }
            SlideDirection::Left => {
                let offset = original_margin_end - distance_px;
                let current = offset + (distance_px as f64 * eased) as i32;
                w.set_margin_end(current);
            }
        }
        glib::ControlFlow::Continue
    });
}

/// Slides a widget out of view using frame-synced FrameClock timing.
pub fn slide_out(
    widget: &gtk4::Widget,
    direction: SlideDirection,
    distance_px: i32,
    duration_ms: u64,
    hide_after: bool,
) {
    let start_opacity = widget.opacity();
    let original_margin_top = widget.margin_top();
    let original_margin_bottom = widget.margin_bottom();
    let original_margin_start = widget.margin_start();
    let original_margin_end = widget.margin_end();

    let start_time = std::cell::Cell::new(0i64);
    let dur_us = duration_ms as i64 * 1000;

    widget.add_tick_callback(move |w, clock| {
        let now = clock.frame_time();
        if start_time.get() == 0 {
            start_time.set(now);
        }
        let elapsed_us = now - start_time.get();
        if elapsed_us >= dur_us {
            w.set_opacity(0.0);
            match direction {
                SlideDirection::Down => w.set_margin_bottom(original_margin_bottom),
                SlideDirection::Up => w.set_margin_top(original_margin_top),
                SlideDirection::Right => w.set_margin_end(original_margin_end),
                SlideDirection::Left => w.set_margin_start(original_margin_start),
            }
            if hide_after {
                w.set_visible(false);
            }
            return glib::ControlFlow::Break;
        }

        let t = elapsed_us as f64 / dur_us as f64;
        let eased = easing::ease_out_cubic(t);

        w.set_opacity((start_opacity * (1.0 - eased)).min(1.0).max(0.0));
        match direction {
            SlideDirection::Down => {
                let current = original_margin_bottom - (distance_px as f64 * eased) as i32;
                w.set_margin_bottom(current);
            }
            SlideDirection::Up => {
                let current = original_margin_top - (distance_px as f64 * eased) as i32;
                w.set_margin_top(current);
            }
            SlideDirection::Right => {
                let current = original_margin_end - (distance_px as f64 * eased) as i32;
                w.set_margin_end(current);
            }
            SlideDirection::Left => {
                let current = original_margin_start - (distance_px as f64 * eased) as i32;
                w.set_margin_start(current);
            }
        }
        glib::ControlFlow::Continue
    });
}

/// Slides a widget out of view with a completion callback using frame-synced timing.
pub fn slide_out_cb<F>(
    widget: &gtk4::Widget,
    direction: SlideDirection,
    distance_px: i32,
    duration_ms: u64,
    hide_after: bool,
    on_complete: F,
) where
    F: FnOnce() + 'static,
{
    let start_opacity = widget.opacity();
    let original_margin_top = widget.margin_top();
    let original_margin_bottom = widget.margin_bottom();
    let original_margin_start = widget.margin_start();
    let original_margin_end = widget.margin_end();

    let start_time = std::cell::Cell::new(0i64);
    let dur_us = duration_ms as i64 * 1000;

    let on_complete_opt = std::cell::RefCell::new(Some(on_complete));

    widget.add_tick_callback(move |w, clock| {
        let now = clock.frame_time();
        if start_time.get() == 0 {
            start_time.set(now);
        }
        let elapsed_us = now - start_time.get();
        if elapsed_us >= dur_us {
            w.set_opacity(0.0);
            match direction {
                SlideDirection::Down => w.set_margin_bottom(original_margin_bottom),
                SlideDirection::Up => w.set_margin_top(original_margin_top),
                SlideDirection::Right => w.set_margin_end(original_margin_end),
                SlideDirection::Left => w.set_margin_start(original_margin_start),
            }
            if hide_after {
                w.set_visible(false);
            }
            if let Some(cb) = on_complete_opt.borrow_mut().take() {
                cb();
            }
            return glib::ControlFlow::Break;
        }

        let t = elapsed_us as f64 / dur_us as f64;
        let eased = easing::ease_out_cubic(t);

        w.set_opacity((start_opacity * (1.0 - eased)).min(1.0).max(0.0));
        match direction {
            SlideDirection::Down => {
                let current = original_margin_bottom - (distance_px as f64 * eased) as i32;
                w.set_margin_bottom(current);
            }
            SlideDirection::Up => {
                let current = original_margin_top - (distance_px as f64 * eased) as i32;
                w.set_margin_top(current);
            }
            SlideDirection::Right => {
                let current = original_margin_end - (distance_px as f64 * eased) as i32;
                w.set_margin_end(current);
            }
            SlideDirection::Left => {
                let current = original_margin_start - (distance_px as f64 * eased) as i32;
                w.set_margin_start(current);
            }
        }
        glib::ControlFlow::Continue
    });
}
