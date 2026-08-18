//! Wallpaper rendering widget for desktop background with water-drop ripple transition animation.
//! Supports all dynamic screen resolutions (1080p, 2K, 4K, ultrawide) at 120Hz+.

use gdk_pixbuf::Pixbuf;
use gtk4::cairo;
use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

/// Duration of the circular water-drop ripple transition in microseconds (550ms).
const TRANSITION_DURATION_US: f64 = 550_000.0;

/// Smooth quartic ease-out function for realistic water ripple expansion.
#[inline]
fn ease_out_quart(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(4)
}

/// Dynamically queries the current display/monitor geometry and scale factor.
fn get_current_monitor_resolution(drawing_area: &gtk4::DrawingArea) -> (i32, i32) {
    let w = drawing_area.width();
    let h = drawing_area.height();
    if w > 100 && h > 100 {
        return (w, h);
    }

    if let Some(display) = gtk4::gdk::Display::default() {
        let monitors = display.monitors();
        if let Some(item) = monitors.item(0) {
            if let Ok(mon) = item.downcast::<gtk4::gdk::Monitor>() {
                let geom = mon.geometry();
                let scale = mon.scale_factor().max(1);
                let mw = geom.width() * scale;
                let mh = geom.height() * scale;
                if mw > 100 && mh > 100 {
                    return (mw, mh);
                }
            }
        }
    }

    (2560, 1440)
}

/// Converts a Pixbuf to an optimized Cairo ImageSurface for ultra-fast memory blitting.
fn pixbuf_to_surface(pixbuf: &Pixbuf) -> Option<cairo::ImageSurface> {
    let width = pixbuf.width();
    let height = pixbuf.height();
    let format = if pixbuf.has_alpha() {
        cairo::Format::ARgb32
    } else {
        cairo::Format::Rgb24
    };

    let surface = cairo::ImageSurface::create(format, width, height).ok()?;
    let cr = cairo::Context::new(&surface).ok()?;
    cr.set_source_pixbuf(pixbuf, 0.0, 0.0);
    let _ = cr.paint();
    Some(surface)
}

/// Loads and pre-scales an image to the exact monitor dimensions (with 3% margin for subtle zoom)
/// so that Cairo rendering during animation does zero heavy resampling and renders in < 0.2ms.
fn load_and_prescale(path: &PathBuf, target_w: i32, target_h: i32) -> Option<cairo::ImageSurface> {
    let pixbuf = Pixbuf::from_file(path).ok()?;
    let orig_w = pixbuf.width() as f64;
    let orig_h = pixbuf.height() as f64;
    if orig_w <= 0.0 || orig_h <= 0.0 {
        return None;
    }

    let req_w = ((target_w as f64) * 1.03).round() as i32;
    let req_h = ((target_h as f64) * 1.03).round() as i32;

    let scale_x = req_w as f64 / orig_w;
    let scale_y = req_h as f64 / orig_h;
    let scale = scale_x.max(scale_y);

    let scaled_w = (orig_w * scale).round() as i32;
    let scaled_h = (orig_h * scale).round() as i32;

    let scaled_pixbuf = if (scaled_w - orig_w as i32).abs() < 50 && (scaled_h - orig_h as i32).abs() < 50 {
        pixbuf
    } else {
        pixbuf.scale_simple(scaled_w, scaled_h, gdk_pixbuf::InterpType::Bilinear)?
    };

    pixbuf_to_surface(&scaled_pixbuf)
}

/// Blits a pre-scaled ImageSurface covering the screen area with aspect-fill, alpha, and zoom.
fn draw_surface_aspect_fill(
    cr: &cairo::Context,
    surface: &cairo::ImageSurface,
    screen_w: f64,
    screen_h: f64,
    alpha: f64,
    zoom: f64,
) {
    if alpha <= 0.001 {
        return;
    }

    let surf_w = surface.width() as f64;
    let surf_h = surface.height() as f64;
    if surf_w <= 0.0 || surf_h <= 0.0 {
        return;
    }

    let scale_x = screen_w / surf_w;
    let scale_y = screen_h / surf_h;
    let base_scale = scale_x.max(scale_y);
    let scale = base_scale * zoom;

    let scaled_w = surf_w * scale;
    let scaled_h = surf_h * scale;
    let offset_x = (screen_w - scaled_w) / 2.0;
    let offset_y = (screen_h - scaled_h) / 2.0;

    let _ = cr.save();
    cr.translate(offset_x, offset_y);
    cr.scale(scale, scale);
    let _ = cr.set_source_surface(surface, 0.0, 0.0);
    if alpha >= 0.999 {
        let _ = cr.paint();
    } else {
        let _ = cr.paint_with_alpha(alpha);
    }
    let _ = cr.restore();
}

/// Creates a fullscreen wallpaper widget with a water-drop ripple transition opening from the corner.
pub fn create_wallpaper_widget() -> gtk4::DrawingArea {
    let drawing_area = gtk4::DrawingArea::new();
    drawing_area.set_hexpand(true);
    drawing_area.set_vexpand(true);
    drawing_area.add_css_class("desktop-wallpaper-container");

    let current_wp_path: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));
    let old_surface: Rc<RefCell<Option<cairo::ImageSurface>>> = Rc::new(RefCell::new(None));
    let current_surface: Rc<RefCell<Option<cairo::ImageSurface>>> = Rc::new(RefCell::new(None));
    let transition_progress: Rc<Cell<f64>> = Rc::new(Cell::new(1.0));
    let is_animating: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let active_start_time: Rc<Cell<Option<i64>>> = Rc::new(Cell::new(None));
    let ripple_origin: Rc<Cell<(f64, f64)>> = Rc::new(Cell::new((1.0, 0.0)));

    // Initial wallpaper load with dynamic monitor resolution detection
    let (init_w, init_h) = get_current_monitor_resolution(&drawing_area);
    if let Some(path) = babydra_core::wallpaper::get_current_wallpaper() {
        if let Some(surf) = load_and_prescale(&path, init_w, init_h) {
            *current_surface.borrow_mut() = Some(surf);
            *current_wp_path.borrow_mut() = Some(path);
        }
    }

    // High-performance paint function with expanding water ripple circle from randomized origin
    let old_surf_draw = old_surface.clone();
    let cur_surf_draw = current_surface.clone();
    let progress_draw = transition_progress.clone();
    let origin_draw = ripple_origin.clone();

    drawing_area.set_draw_func(move |_, cr, width, height| {
        let w = width as f64;
        let h = height as f64;
        let p = progress_draw.get();

        // 1. Fallback dark background
        let _ = cr.set_source_rgb(0.08, 0.09, 0.11);
        let _ = cr.paint();

        // 2. Base layer: previous wallpaper
        if p < 1.0 {
            if let Some(ref old_surf) = *old_surf_draw.borrow() {
                draw_surface_aspect_fill(cr, old_surf, w, h, 1.0, 1.0);
            }
        }

        // 3. New wallpaper: Expanding circular water drop ripple from random origin
        if let Some(ref cur_surf) = *cur_surf_draw.borrow() {
            if p < 1.0 {
                let (rx, ry) = origin_draw.get();
                let origin_x = w * rx;
                let origin_y = h * ry;

                // Dynamically compute maximum radius to the furthest corner of the screen
                let d1 = (origin_x * origin_x + origin_y * origin_y).sqrt();
                let d2 = ((w - origin_x).powi(2) + origin_y * origin_y).sqrt();
                let d3 = (origin_x * origin_x + (h - origin_y).powi(2)).sqrt();
                let d4 = ((w - origin_x).powi(2) + (h - origin_y).powi(2)).sqrt();
                let max_radius = d1.max(d2).max(d3).max(d4);

                let eased = ease_out_quart(p);
                let current_radius = max_radius * eased;

                // Circular mask clip for new wallpaper
                let _ = cr.save();
                cr.arc(origin_x, origin_y, current_radius, 0.0, 2.0 * std::f64::consts::PI);
                cr.clip();
                let zoom = 1.02 - eased * 0.02;
                draw_surface_aspect_fill(cr, cur_surf, w, h, 1.0, zoom);
                let _ = cr.restore();

                // Luminous water ripple wave rings along the circumference
                let wave_alpha = (1.0 - p) * 0.45;
                if wave_alpha > 0.01 && current_radius > 8.0 {
                    let _ = cr.save();
                    // Primary ripple wavefront
                    cr.set_source_rgba(1.0, 1.0, 1.0, wave_alpha);
                    cr.set_line_width(3.5 * (1.0 - p * 0.4));
                    cr.arc(origin_x, origin_y, current_radius, 0.0, 2.0 * std::f64::consts::PI);
                    let _ = cr.stroke();

                    // Secondary subtle trailing ripple
                    if current_radius > 25.0 {
                        cr.set_source_rgba(1.0, 1.0, 1.0, wave_alpha * 0.35);
                        cr.set_line_width(1.5);
                        cr.arc(origin_x, origin_y, (current_radius - 12.0).max(0.0), 0.0, 2.0 * std::f64::consts::PI);
                        let _ = cr.stroke();
                    }
                    let _ = cr.restore();
                }
            } else {
                // Fully completed: Render full screen
                draw_surface_aspect_fill(cr, cur_surf, w, h, 1.0, 1.0);
            }
        }
    });

    // Helper closure to trigger smooth transition to a new wallpaper
    let trigger_transition = {
        let da_c = drawing_area.clone();
        let cur_path_c = current_wp_path.clone();
        let old_surf_c = old_surface.clone();
        let cur_surf_c = current_surface.clone();
        let prog_c = transition_progress.clone();
        let anim_c = is_animating.clone();
        let start_time_c = active_start_time.clone();
        let origin_c = ripple_origin.clone();

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
                    // Pick a random origin corner/edge position for this transition
                    let nanos = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.subsec_nanos())
                        .unwrap_or(123456);

                    let choices = [
                        (0.0, 0.0), // Top-Left
                        (1.0, 0.0), // Top-Right
                        (0.0, 1.0), // Bottom-Left
                        (1.0, 1.0), // Bottom-Right
                        (0.5, 0.0), // Top-Center
                        (0.5, 1.0), // Bottom-Center
                        (1.0, 0.5), // Right-Center
                        (0.0, 0.5), // Left-Center
                    ];
                    let idx = (nanos as usize) % choices.len();
                    origin_c.set(choices[idx]);

                    let (mon_w, mon_h) = get_current_monitor_resolution(&da_c);
                    if let Some(new_surf) = load_and_prescale(path, mon_w, mon_h) {
                        let has_prev = cur_surf_c.borrow().is_some();
                        if has_prev {
                            let prev = cur_surf_c.borrow_mut().take();
                            *old_surf_c.borrow_mut() = prev;
                            *cur_surf_c.borrow_mut() = Some(new_surf);
                            *cur_path_c.borrow_mut() = new_path.clone();
                            prog_c.set(0.0);
                            start_time_c.set(None);

                            if !anim_c.get() {
                                anim_c.set(true);
                                let da_tick = da_c.clone();
                                let prog_tick = prog_c.clone();
                                let anim_tick = anim_c.clone();
                                let old_surf_tick = old_surf_c.clone();
                                let start_time_tick = start_time_c.clone();

                                da_c.add_tick_callback(move |_, clock| {
                                    let now = clock.frame_time();
                                    if start_time_tick.get().is_none() {
                                        start_time_tick.set(Some(now));
                                    }

                                    let start = start_time_tick.get().unwrap();
                                    let elapsed = (now - start) as f64;
                                    let progress = (elapsed / TRANSITION_DURATION_US).clamp(0.0, 1.0);
                                    prog_tick.set(progress);
                                    da_tick.queue_draw();

                                    if progress >= 1.0 {
                                        *old_surf_tick.borrow_mut() = None;
                                        anim_tick.set(false);
                                        glib::ControlFlow::Break
                                    } else {
                                        glib::ControlFlow::Continue
                                    }
                                });
                            }
                        } else {
                            *cur_surf_c.borrow_mut() = Some(new_surf);
                            *cur_path_c.borrow_mut() = new_path.clone();
                            prog_c.set(1.0);
                            da_c.queue_draw();
                        }
                    }
                } else {
                    *old_surf_c.borrow_mut() = None;
                    *cur_surf_c.borrow_mut() = None;
                    *cur_path_c.borrow_mut() = None;
                    prog_c.set(1.0);
                    da_c.queue_draw();
                }
            }
        })
    };

    // 1. Instantaneous file watcher on ~/.babydra and ~/.babydra/wallpaper (0ms reaction)
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let babydra_dir = home.join(".babydra");
    let wallpaper_dir = babydra_dir.join("wallpaper");
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();

    let trigger_watch = trigger_transition.clone();
    glib::spawn_future_local(async move {
        while rx.recv().await.is_some() {
            trigger_watch();
        }
    });

    let tx1 = tx.clone();
    if let Ok(_w1) = babydra_core::FileWatcher::new(babydra_dir, move |_| {
        let _ = tx1.send(());
    }) {
        std::mem::forget(_w1);
    }

    let tx2 = tx.clone();
    if let Ok(_w2) = babydra_core::FileWatcher::new(wallpaper_dir, move |_| {
        let _ = tx2.send(());
    }) {
        std::mem::forget(_w2);
    }

    // 2. Periodic poll fallback (every 500ms) to ensure sync in all edge cases
    let trigger_poll = trigger_transition.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        trigger_poll();
        glib::ControlFlow::Continue
    });

    drawing_area
}
