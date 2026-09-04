//! Wallpaper rendering widget for desktop background with water-drop ripple transition animation.
//! Supports all dynamic screen resolutions (1080p, 2K, 4K, ultrawide) at 120Hz+.

pub mod animation;
pub mod player;
pub mod renderer;
pub mod surface;

pub use animation::*;
pub use player::*;
pub use renderer::*;
pub use surface::*;

use gtk4::cairo;
use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

/// Creates a fullscreen wallpaper widget with GPU acceleration for live wallpaper (Video & GIF)
/// and water-drop ripple transition for all transitions (static <-> video <-> static).
pub fn create_wallpaper_w() -> gtk4::Overlay {
    let container = gtk4::Overlay::new();
    container.set_hexpand(true);
    container.set_vexpand(true);
    container.add_css_class("desktop-wallpaper-container");

    let drawing_area = gtk4::DrawingArea::new();
    drawing_area.set_hexpand(true);
    drawing_area.set_vexpand(true);

    let live_picture = gtk4::Picture::new();
    live_picture.set_hexpand(true);
    live_picture.set_vexpand(true);
    live_picture.set_can_shrink(true);
    live_picture.set_content_fit(gtk4::ContentFit::Cover);
    live_picture.set_visible(false);

    container.set_child(Some(&drawing_area));
    container.add_overlay(&live_picture);

    let current_wp_path: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));
    let current_wp_mode: Rc<RefCell<babydra_core::wallpaper::WallpaperMode>> =
        Rc::new(RefCell::new(babydra_core::wallpaper::WallpaperMode::Static));
    let active_media_file: Rc<RefCell<Option<gtk4::MediaFile>>> = Rc::new(RefCell::new(None));
    let gif_source_id: Rc<RefCell<Option<glib::SourceId>>> = Rc::new(RefCell::new(None));

    let old_surface: Rc<RefCell<Option<cairo::ImageSurface>>> = Rc::new(RefCell::new(None));
    let current_surface: Rc<RefCell<Option<cairo::ImageSurface>>> = Rc::new(RefCell::new(None));
    let transition_progress: Rc<Cell<f64>> = Rc::new(Cell::new(1.0));
    let is_animating: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let active_start_time: Rc<Cell<Option<i64>>> = Rc::new(Cell::new(None));
    let ripple_origin: Rc<Cell<(f64, f64)>> = Rc::new(Cell::new((1.0, 0.0)));
    let pending_live_play: Rc<RefCell<Option<(PathBuf, babydra_core::wallpaper::WallpaperMode)>>> =
        Rc::new(RefCell::new(None));

    // Initial wallpaper load with dynamic monitor resolution detection
    let (init_w, init_h) = get_monitor_res(&drawing_area);
    let init_path = babydra_core::wallpaper::get_wallpaper();
    let init_mode = babydra_core::wallpaper::get_wallpaper_mode();
    *current_wp_mode.borrow_mut() = init_mode;

    if let Some(ref path) = init_path {
        *current_wp_path.borrow_mut() = Some(path.clone());
        // Pre-populate current_surface with the first frame (for both static and live mode)
        if let Some(surf) = load_first_frame_surface(path, init_w, init_h) {
            *current_surface.borrow_mut() = Some(surf);
        }

        if init_mode == babydra_core::wallpaper::WallpaperMode::Live {
            start_live_media(
                path,
                &init_mode,
                &live_picture,
                &drawing_area,
                &active_media_file,
                &gif_source_id,
            );
        }
    }

    // High-performance paint function with expanding water ripple circle from randomized origin
    let old_surf_draw = old_surface.clone();
    let cur_surf_draw = current_surface.clone();
    let progress_draw = transition_progress.clone();
    let origin_draw = ripple_origin.clone();

    drawing_area.set_draw_func(move |_, cr, width, height| {
        render_wallpaper(
            cr,
            width as f64,
            height as f64,
            progress_draw.get(),
            origin_draw.get(),
            old_surf_draw.borrow().as_ref(),
            cur_surf_draw.borrow().as_ref(),
        );
    });

    // Helper closure to trigger smooth transition to a new wallpaper
    let trigger_transition = {
        let da_c = drawing_area.clone();
        let live_pic_c = live_picture.clone();
        let cur_path_c = current_wp_path.clone();
        let cur_mode_c = current_wp_mode.clone();
        let old_surf_c = old_surface.clone();
        let cur_surf_c = current_surface.clone();
        let prog_c = transition_progress.clone();
        let anim_c = is_animating.clone();
        let start_time_c = active_start_time.clone();
        let origin_c = ripple_origin.clone();
        let active_media_c = active_media_file.clone();
        let gif_source_c = gif_source_id.clone();
        let pending_live_c = pending_live_play.clone();

        Rc::new(move || {
            let new_path = babydra_core::wallpaper::get_wallpaper();
            let new_mode = babydra_core::wallpaper::get_wallpaper_mode();
            let path_changed = match (&new_path, &*cur_path_c.borrow()) {
                (Some(p1), Some(p2)) => p1 != p2,
                (Some(_), None) => true,
                (None, Some(_)) => true,
                (None, None) => false,
            };
            let mode_changed = new_mode != *cur_mode_c.borrow();

            if path_changed || mode_changed {
                let prev_path = cur_path_c.borrow().clone();
                let prev_mode = *cur_mode_c.borrow();
                *cur_mode_c.borrow_mut() = new_mode;

                let (mon_w, mon_h) = get_monitor_res(&da_c);

                // 1. Determine what the previous visible surface (old_surface) was
                let prev_surf = if prev_mode == babydra_core::wallpaper::WallpaperMode::Live {
                    // Video or animated GIF was active: capture current playing frame
                    capture_live_picture_surface(&live_pic_c)
                        .or_else(|| {
                            prev_path
                                .as_ref()
                                .and_then(|p| load_first_frame_surface(p, mon_w, mon_h))
                        })
                        .or_else(|| cur_surf_c.borrow().clone())
                } else {
                    // Static wallpaper was active: use current_surface
                    cur_surf_c.borrow().clone()
                };

                // 2. Stop and clear any active live playback
                if let Some(id) = gif_source_c.borrow_mut().take() {
                    id.remove();
                }
                if let Some(mf) = active_media_c.borrow_mut().take() {
                    mf.pause();
                }
                live_pic_c.set_paintable(None::<&gtk4::gdk::Paintable>);
                live_pic_c.set_visible(false);
                da_c.set_visible(true);

                // 3. Pick random ripple origin
                origin_c.set(pick_random_ripple_origin());

                if let Some(ref path) = new_path {
                    *cur_path_c.borrow_mut() = Some(path.clone());

                    // Helper to launch or restart the water-drop ripple animation
                    let trigger_tick_animation = || {
                        prog_c.set(0.0);
                        start_time_c.set(None);
                        if !anim_c.get() {
                            anim_c.set(true);
                            let da_tick = da_c.clone();
                            let prog_tick = prog_c.clone();
                            let anim_tick = anim_c.clone();
                            let old_surf_tick = old_surf_c.clone();
                            let start_time_tick = start_time_c.clone();
                            let pending_live_tick = pending_live_c.clone();
                            let live_pic_tick = live_pic_c.clone();
                            let active_media_tick = active_media_c.clone();
                            let gif_source_tick = gif_source_c.clone();

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

                                    // After transition completes: start live playback seamlessly
                                    if let Some((live_path, live_mode)) =
                                        pending_live_tick.borrow_mut().take()
                                    {
                                        start_live_media(
                                            &live_path,
                                            &live_mode,
                                            &live_pic_tick,
                                            &da_tick,
                                            &active_media_tick,
                                            &gif_source_tick,
                                        );
                                    }

                                    glib::ControlFlow::Break
                                } else {
                                    glib::ControlFlow::Continue
                                }
                            });
                        }
                    };

                    if new_mode == babydra_core::wallpaper::WallpaperMode::Live {
                        // LIVE WALLPAPER TARGET:
                        // Extract first frame of video or load frame 0 of GIF
                        let target_surf = load_first_frame_surface(path, mon_w, mon_h);

                        if let Some(new_surf) = target_surf {
                            *cur_surf_c.borrow_mut() = Some(new_surf);
                            *pending_live_c.borrow_mut() = Some((path.clone(), new_mode));

                            if let Some(prev) = prev_surf {
                                *old_surf_c.borrow_mut() = Some(prev);
                                trigger_tick_animation();
                            } else {
                                // No previous surface: immediately start playing live wallpaper
                                prog_c.set(1.0);
                                da_c.queue_draw();
                                if let Some((live_path, live_mode)) =
                                    pending_live_c.borrow_mut().take()
                                {
                                    start_live_media(
                                        &live_path,
                                        &live_mode,
                                        &live_pic_c,
                                        &da_c,
                                        &active_media_c,
                                        &gif_source_c,
                                    );
                                }
                            }
                        }
                    } else {
                        // STATIC WALLPAPER TARGET:
                        *pending_live_c.borrow_mut() = None;
                        if let Some(new_surf) = load_and_prescale(path, mon_w, mon_h) {
                            *cur_surf_c.borrow_mut() = Some(new_surf);

                            if let Some(prev) = prev_surf {
                                *old_surf_c.borrow_mut() = Some(prev);
                                trigger_tick_animation();
                            } else {
                                prog_c.set(1.0);
                                da_c.queue_draw();
                            }
                        }
                    }
                } else {
                    *old_surf_c.borrow_mut() = None;
                    *cur_surf_c.borrow_mut() = None;
                    *cur_path_c.borrow_mut() = None;
                    *pending_live_c.borrow_mut() = None;
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

    container
}
