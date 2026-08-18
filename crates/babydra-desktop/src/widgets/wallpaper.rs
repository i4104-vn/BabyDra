//! Wallpaper rendering widget for desktop background with smooth crossfade transition animation.

use gdk_pixbuf::Pixbuf;
use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

/// Duration of the wallpaper crossfade transition in microseconds (450ms).
const TRANSITION_DURATION_US: f64 = 450_000.0;

/// Smooth cubic easing function for natural crossfade.
fn ease_in_out(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

/// Renders a pixbuf covering the area with aspect-fill and custom alpha transparency.
fn draw_aspect_fill(
    cr: &gtk4::cairo::Context,
    pixbuf: &Pixbuf,
    width: f64,
    height: f64,
    alpha: f64,
) {
    if alpha <= 0.001 {
        return;
    }

    let img_w = pixbuf.width() as f64;
    let img_h = pixbuf.height() as f64;
    if img_w <= 0.0 || img_h <= 0.0 {
        return;
    }

    let scale_x = width / img_w;
    let scale_y = height / img_h;
    let scale = scale_x.max(scale_y);

    let scaled_w = img_w * scale;
    let scaled_h = img_h * scale;
    let offset_x = (width - scaled_w) / 2.0;
    let offset_y = (height - scaled_h) / 2.0;

    let _ = cr.save();
    cr.translate(offset_x, offset_y);
    cr.scale(scale, scale);
    cr.set_source_pixbuf(pixbuf, 0.0, 0.0);
    if alpha >= 0.999 {
        let _ = cr.paint();
    } else {
        let _ = cr.paint_with_alpha(alpha);
    }
    let _ = cr.restore();
}

/// Creates a fullscreen wallpaper widget that scales, covers, and animates transitions smoothly.
pub fn create_wallpaper_widget() -> gtk4::DrawingArea {
    let drawing_area = gtk4::DrawingArea::new();
    drawing_area.set_hexpand(true);
    drawing_area.set_vexpand(true);
    drawing_area.add_css_class("desktop-wallpaper-container");

    let current_wp_path: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));
    let old_pixbuf: Rc<RefCell<Option<Pixbuf>>> = Rc::new(RefCell::new(None));
    let current_pixbuf: Rc<RefCell<Option<Pixbuf>>> = Rc::new(RefCell::new(None));
    let transition_progress: Rc<Cell<f64>> = Rc::new(Cell::new(1.0));
    let is_animating: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    // Initial wallpaper load
    if let Some(path) = babydra_core::wallpaper::get_current_wallpaper() {
        if let Ok(pixbuf) = Pixbuf::from_file(&path) {
            *current_pixbuf.borrow_mut() = Some(pixbuf);
            *current_wp_path.borrow_mut() = Some(path);
        }
    }

    // Paint function with crossfade animation
    let old_pb_draw = old_pixbuf.clone();
    let cur_pb_draw = current_pixbuf.clone();
    let progress_draw = transition_progress.clone();

    drawing_area.set_draw_func(move |_, cr, width, height| {
        let w = width as f64;
        let h = height as f64;
        let p = progress_draw.get();

        // 1. Draw fallback dark background
        let _ = cr.set_source_rgb(0.08, 0.09, 0.11);
        let _ = cr.paint();

        // 2. Draw previous wallpaper (if transition is active)
        if p < 1.0 {
            if let Some(ref old_pb) = *old_pb_draw.borrow() {
                draw_aspect_fill(cr, old_pb, w, h, 1.0);
            }
        }

        // 3. Draw new wallpaper with crossfade alpha
        if let Some(ref cur_pb) = *cur_pb_draw.borrow() {
            let alpha = if p < 1.0 { ease_in_out(p) } else { 1.0 };
            draw_aspect_fill(cr, cur_pb, w, h, alpha);
        }
    });

    // Helper closure to trigger smooth transition to a new wallpaper
    let trigger_transition = {
        let da_c = drawing_area.clone();
        let cur_path_c = current_wp_path.clone();
        let old_pb_c = old_pixbuf.clone();
        let cur_pb_c = current_pixbuf.clone();
        let prog_c = transition_progress.clone();
        let anim_c = is_animating.clone();

        Rc::new(move || {
            let new_path = babydra_core::wallpaper::get_current_wallpaper();
            let changed = match (&new_path, &*cur_path_c.borrow()) {
                (Some(p1), Some(p2)) => p1 != p2,
                (Some(_), None) => true,
                (None, Some(_)) => true,
                (None, None) => false,
            };

            if changed {
                if let Some(ref path) = new_path {
                    if let Ok(new_pb) = Pixbuf::from_file(path) {
                        let has_prev = cur_pb_c.borrow().is_some();
                        if has_prev {
                            // Move current to old, install new, start animation
                            let prev = cur_pb_c.borrow_mut().take();
                            *old_pb_c.borrow_mut() = prev;
                            *cur_pb_c.borrow_mut() = Some(new_pb);
                            *cur_path_c.borrow_mut() = new_path.clone();
                            prog_c.set(0.0);

                            if !anim_c.get() {
                                anim_c.set(true);
                                let start_time: Rc<Cell<Option<i64>>> = Rc::new(Cell::new(None));
                                let da_tick = da_c.clone();
                                let prog_tick = prog_c.clone();
                                let anim_tick = anim_c.clone();
                                let old_pb_tick = old_pb_c.clone();

                                da_c.add_tick_callback(move |_, clock| {
                                    let now = clock.frame_time();
                                    if start_time.get().is_none() {
                                        start_time.set(Some(now));
                                    }

                                    let start = start_time.get().unwrap();
                                    let elapsed = (now - start) as f64;
                                    let progress = (elapsed / TRANSITION_DURATION_US).clamp(0.0, 1.0);
                                    prog_tick.set(progress);
                                    da_tick.queue_draw();

                                    if progress >= 1.0 {
                                        *old_pb_tick.borrow_mut() = None;
                                        anim_tick.set(false);
                                        glib::ControlFlow::Break
                                    } else {
                                        glib::ControlFlow::Continue
                                    }
                                });
                            }
                        } else {
                            *cur_pb_c.borrow_mut() = Some(new_pb);
                            *cur_path_c.borrow_mut() = new_path.clone();
                            prog_c.set(1.0);
                            da_c.queue_draw();
                        }
                    }
                } else {
                    *old_pb_c.borrow_mut() = None;
                    *cur_pb_c.borrow_mut() = None;
                    *cur_path_c.borrow_mut() = None;
                    prog_c.set(1.0);
                    da_c.queue_draw();
                }
            }
        })
    };

    // 1. File watcher on ~/.babydra for instantaneous 0ms response when settings changes
    let config_dir = babydra_core::config::get_babydra_config_dir();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();
    let trigger_watch = trigger_transition.clone();

    glib::spawn_future_local(async move {
        while rx.recv().await.is_some() {
            trigger_watch();
        }
    });

    if let Ok(_watcher) = babydra_core::FileWatcher::new(config_dir, move |_| {
        let _ = tx.send(());
    }) {
        std::mem::forget(_watcher);
    }

    // 2. Periodic poll fallback (every 800ms) to ensure sync in all edge cases
    let trigger_poll = trigger_transition.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(800), move || {
        trigger_poll();
        glib::ControlFlow::Continue
    });

    drawing_area
}
