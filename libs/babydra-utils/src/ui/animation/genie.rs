//! Fixed-size smooth fade animations for popup dialog windows.

use gtk4::prelude::*;
use super::easing;

/// Animates a widget fading in smoothly while maintaining a fixed window size.
pub fn genie_in(widget: &gtk4::Widget, _target_width: i32, _target_height: i32, duration_ms: u64) {
    widget.set_opacity(0.0);
    widget.set_visible(true);

    let start_time = std::cell::Cell::new(0i64);
    let dur_us = (duration_ms.max(150) as i64) * 1000;

    widget.add_tick_callback(move |w, clock| {
        let now = clock.frame_time();
        if start_time.get() == 0 {
            start_time.set(now);
        }
        let elapsed_us = now - start_time.get();
        if elapsed_us >= dur_us {
            w.set_opacity(1.0);
            return glib::ControlFlow::Break;
        }

        let t = elapsed_us as f64 / dur_us as f64;
        let eased = easing::ease_out_cubic(t);
        w.set_opacity(eased);

        glib::ControlFlow::Continue
    });
}

/// Animates a widget fading out smoothly while maintaining a fixed window size, then runs the completion callback.
pub fn genie_out<F>(widget: &gtk4::Widget, _target_width: i32, _target_height: i32, duration_ms: u64, on_complete: F)
where
    F: FnOnce() + 'static,
{
    let start_time = std::cell::Cell::new(0i64);
    let dur_us = (duration_ms.max(150) as i64) * 1000;
    let on_complete_opt = std::cell::RefCell::new(Some(on_complete));

    widget.add_tick_callback(move |w, clock| {
        let now = clock.frame_time();
        if start_time.get() == 0 {
            start_time.set(now);
        }
        let elapsed_us = now - start_time.get();
        if elapsed_us >= dur_us {
            w.set_opacity(0.0);
            if let Some(cb) = on_complete_opt.borrow_mut().take() {
                cb();
            }
            return glib::ControlFlow::Break;
        }

        let t = elapsed_us as f64 / dur_us as f64;
        let eased = easing::ease_out_cubic(t);
        w.set_opacity(1.0 - eased);

        glib::ControlFlow::Continue
    });
}
