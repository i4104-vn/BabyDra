//! Dynamic Island capsule size / zoom transition animations.

use gtk4::prelude::*;
use super::easing;

/// Animates the Dynamic Island capsule expanding from a small pill to full width. Uses FrameClock timing.
pub fn island_zoom_in(widget: &gtk4::Widget, target_width: i32, target_height: i32, duration_ms: u64) {
    widget.set_opacity(1.0);
    widget.set_visible(true);
    widget.set_size_request(target_height, target_height);

    if let Some(ref child) = widget.first_child() {
        child.set_opacity(0.0);
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
            w.set_size_request(target_width, target_height);
            w.set_opacity(1.0);
            if let Some(ref child) = w.first_child() {
                child.set_opacity(1.0);
            }
            return glib::ControlFlow::Break;
        }

        let t = elapsed_us as f64 / dur_us as f64;
        w.set_opacity(1.0);

        let eased_w = easing::ease_out_cubic(t);
        let current_w = target_height + ((target_width - target_height) as f64 * eased_w) as i32;
        w.set_size_request(current_w, target_height);

        if let Some(ref child) = w.first_child() {
            let child_t = (t - 0.5) * 2.0;
            let child_opacity = child_t.max(0.0).min(1.0);
            child.set_opacity(child_opacity);
        }

        glib::ControlFlow::Continue
    });
}

/// Animates the Dynamic Island capsule collapsing back to a pill. Uses FrameClock timing.
pub fn island_zoom_out(widget: &gtk4::Widget, target_width: i32, duration_ms: u64, hide_after: bool) {
    let start_h = widget.height().max(22);

    if let Some(ref child) = widget.first_child() {
        child.set_opacity(1.0);
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
            w.set_size_request(0, 0);
            w.set_opacity(0.0);
            if let Some(ref child) = w.first_child() {
                child.set_opacity(0.0);
            }
            if hide_after {
                w.set_visible(false);
            }
            return glib::ControlFlow::Break;
        }

        let t = elapsed_us as f64 / dur_us as f64;
        let capsule_opacity = (1.0 - (t - 0.5) * 2.0).max(0.0).min(1.0);
        w.set_opacity(capsule_opacity);

        let eased_w = easing::ease_out_cubic(t);
        let current_w = start_h + ((target_width - start_h) as f64 * (1.0 - eased_w)) as i32;
        w.set_size_request(current_w, start_h);

        if let Some(ref child) = w.first_child() {
            let child_opacity = (1.0 - t * 2.0).max(0.0).min(1.0);
            child.set_opacity(child_opacity);
        }

        glib::ControlFlow::Continue
    });
}

/// Animates the width of the Dynamic Island capsule from start_width to target_width.
pub fn island_animate_width<F>(
    widget: &gtk4::Widget,
    start_width: i32,
    target_width: i32,
    duration_ms: u64,
    on_complete: F,
) where
    F: FnOnce() + 'static,
{
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
            w.set_size_request(target_width, -1);
            if let Some(cb) = on_complete_opt.borrow_mut().take() {
                cb();
            }
            return glib::ControlFlow::Break;
        }

        let t = elapsed_us as f64 / dur_us as f64;
        let eased = easing::ease_out_cubic(t);
        let current_w = start_width + ((target_width - start_width) as f64 * eased) as i32;
        w.set_size_request(current_w, -1);

        glib::ControlFlow::Continue
    });
}

/// Animates both the width and height of the Dynamic Island capsule.
pub fn island_animate_size<F>(
    widget: &gtk4::Widget,
    start_width: i32,
    target_width: i32,
    start_height: i32,
    target_height: i32,
    duration_ms: u64,
    on_complete: F,
) where
    F: FnOnce() + 'static,
{
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
            w.set_size_request(target_width, target_height);
            if let Some(cb) = on_complete_opt.borrow_mut().take() {
                cb();
            }
            return glib::ControlFlow::Break;
        }

        let t = elapsed_us as f64 / dur_us as f64;
        let eased = easing::ease_out_cubic(t);
        let current_w = start_width + ((target_width - start_width) as f64 * eased) as i32;
        let current_h = start_height + ((target_height - start_height) as f64 * eased) as i32;
        w.set_size_request(current_w, current_h);

        glib::ControlFlow::Continue
    });
}
