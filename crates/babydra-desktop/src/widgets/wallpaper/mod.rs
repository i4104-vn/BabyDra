//! Wallpaper rendering widget for desktop background with water-drop ripple transition animation.
//! Supports all dynamic screen resolutions (1080p, 2K, 4K, ultrawide) at 120Hz+.

pub mod animation;
pub mod player;
pub mod renderer;
pub mod state;
pub mod surface;
pub mod transition;
pub mod watcher;

pub use animation::*;
pub use player::*;
pub use renderer::*;
pub use state::*;
pub use surface::*;
pub use transition::*;
pub use watcher::*;

use gtk4::prelude::*;
use std::rc::Rc;

/// Creates a fullscreen wallpaper widget with GPU acceleration for live wallpaper (Video & GIF)
/// and water-drop ripple transition for all transitions (static <-> video <-> static).
pub fn create_wallpaper_w() -> (gtk4::Overlay, Rc<dyn Fn(bool)>) {
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

    let state = WallpaperState::new();

    // Initial wallpaper load with dynamic monitor resolution detection
    let (init_w, init_h) = get_monitor_res(&drawing_area);
    let init_path = babydra_core::wallpaper::get_wallpaper();
    let init_mode = babydra_core::wallpaper::get_wallpaper_mode();
    *state.current_wp_mode.borrow_mut() = init_mode;

    if let Some(ref path) = init_path {
        *state.current_wp_path.borrow_mut() = Some(path.clone());
        if let Some(surf) = load_first_frame_surface(path, init_w, init_h) {
            *state.current_surface.borrow_mut() = Some(surf);
        }

        if init_mode == babydra_core::wallpaper::WallpaperMode::Live {
            start_live_media(
                path,
                &init_mode,
                &live_picture,
                &drawing_area,
                &state.active_media_file,
                &state.is_frozen,
                &state.gif_source_id,
                &state.recycle_source_id,
                None,
            );
        }
    }

    // High-performance paint function with expanding water ripple circle from randomized origin
    let old_surf_draw = state.old_surface.clone();
    let cur_surf_draw = state.current_surface.clone();
    let progress_draw = state.transition_progress.clone();
    let origin_draw = state.ripple_origin.clone();

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

    // Wire transition trigger and filesystem watchers
    let trigger_transition = create_transition_trigger(&drawing_area, &live_picture, &state);
    start_wallpaper_watcher(trigger_transition.clone());

    let frozen = state.is_frozen.clone();
    let active_media = state.active_media_file.clone();
    let pending_live_play = state.pending_live_play.clone();
    let current_path = state.current_wp_path.clone();
    let current_mode = state.current_wp_mode.clone();
    let live_picture_for_freeze = live_picture.clone();
    let drawing_area_for_freeze = drawing_area.clone();
    let gif_source = state.gif_source_id.clone();
    let recycle_source = state.recycle_source_id.clone();
    let frozen_for_media = state.is_frozen.clone();
    let refresh_after_resume = trigger_transition;
    let set_frozen = Rc::new(move |should_freeze| {
        if frozen.replace(should_freeze) == should_freeze {
            return;
        }

        if let Some(media) = active_media.borrow().as_ref() {
            if should_freeze {
                safely_pause_media_file(media);
            } else if media.error().is_none() {
                media.play();
            }
        }

        if !should_freeze {
            let has_pending_play = pending_live_play.borrow().is_some();
            let video_path = current_path
                .borrow()
                .clone()
                .filter(|path| babydra_core::wallpaper::is_video_file(path));
            let media_needs_restart = match active_media.borrow().as_ref() {
                Some(media) => media.error().is_some(),
                None => true,
            };
            if *current_mode.borrow() == babydra_core::wallpaper::WallpaperMode::Live
                && !has_pending_play
                && media_needs_restart
            {
                if let Some(path) = video_path {
                    start_live_media(
                        &path,
                        &current_mode.borrow(),
                        &live_picture_for_freeze,
                        &drawing_area_for_freeze,
                        &active_media,
                        &frozen_for_media,
                        &gif_source,
                        &recycle_source,
                        None,
                    );
                }
            }
            refresh_after_resume();
        }
    });

    (container, set_frozen)
}
