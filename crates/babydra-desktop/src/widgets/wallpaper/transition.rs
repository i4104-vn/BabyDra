//! Wallpaper transition coordination and water-drop ripple animation loop.

use super::animation::{pick_random_ripple_origin, TRANSITION_DURATION_US};
use super::player::{prepare_live_video, start_live_media};
use super::state::WallpaperState;
use super::surface::{
    capture_live_picture_surface, get_monitor_res, load_and_prescale, load_first_frame_surface,
};
use gtk4::prelude::*;
use std::rc::Rc;

/// Creates a transition trigger callback that detects wallpaper changes and starts the animation.
pub fn create_transition_trigger(
    drawing_area: &gtk4::DrawingArea,
    live_picture: &gtk4::Picture,
    state: &WallpaperState,
) -> Rc<dyn Fn()> {
    let da_c = drawing_area.clone();
    let live_pic_c = live_picture.clone();
    let state_c = state.clone();

    Rc::new(move || {
        let new_path = babydra_core::wallpaper::get_wallpaper();
        let new_mode = babydra_core::wallpaper::get_wallpaper_mode();
        let path_changed = match (&new_path, &*state_c.current_wp_path.borrow()) {
            (Some(p1), Some(p2)) => p1 != p2,
            (Some(_), None) => true,
            (None, Some(_)) => true,
            (None, None) => false,
        };
        let mode_changed = new_mode != *state_c.current_wp_mode.borrow();

        if path_changed || mode_changed {
            execute_transition(&da_c, &live_pic_c, &state_c, new_path, new_mode);
        }
    })
}

/// Executes a transition to the new wallpaper path and mode.
fn execute_transition(
    da: &gtk4::DrawingArea,
    live_pic: &gtk4::Picture,
    state: &WallpaperState,
    new_path: Option<std::path::PathBuf>,
    new_mode: babydra_core::wallpaper::WallpaperMode,
) {
    let prev_path = state.current_wp_path.borrow().clone();
    let prev_mode = *state.current_wp_mode.borrow();
    *state.current_wp_mode.borrow_mut() = new_mode;

    let (mon_w, mon_h) = get_monitor_res(da);

    // 1. Determine the previous visible surface (old_surface) for the transition effect
    let prev_surf = if prev_mode == babydra_core::wallpaper::WallpaperMode::Live {
        capture_live_picture_surface(live_pic)
            .or_else(|| {
                prev_path
                    .as_ref()
                    .and_then(|p| load_first_frame_surface(p, mon_w, mon_h))
            })
            .or_else(|| state.current_surface.borrow().clone())
    } else {
        state.current_surface.borrow().clone()
    };

    // 2. Stop and clear any active live playback
    state.stop_active_media();
    live_pic.set_paintable(None::<&gtk4::gdk::Paintable>);
    live_pic.set_visible(false);
    da.set_visible(true);

    // 3. Pick a random origin for the expanding circular water-drop ripple
    state.ripple_origin.set(pick_random_ripple_origin());

    if let Some(ref path) = new_path {
        *state.current_wp_path.borrow_mut() = Some(path.clone());

        if new_mode == babydra_core::wallpaper::WallpaperMode::Live {
            // Live wallpaper target: pre-warm video in background
            let pre_rolled = prepare_live_video(path);
            let target_surf = load_first_frame_surface(path, mon_w, mon_h);

            if let Some(new_surf) = target_surf {
                *state.current_surface.borrow_mut() = Some(new_surf);
                *state.pending_live_play.borrow_mut() =
                    Some((path.clone(), new_mode, pre_rolled));

                if let Some(prev) = prev_surf {
                    *state.old_surface.borrow_mut() = Some(prev);
                    start_transition_animation(da, live_pic, state);
                } else {
                    state.transition_progress.set(1.0);
                    da.queue_draw();
                    if let Some((live_path, live_mode, pre_rolled_mf)) =
                        state.pending_live_play.borrow_mut().take()
                    {
                        start_live_media(
                            &live_path,
                            &live_mode,
                            live_pic,
                            da,
                            &state.active_media_file,
                            &state.gif_source_id,
                            &state.recycle_source_id,
                            pre_rolled_mf,
                        );
                    }
                }
            }
        } else {
            // Static wallpaper target
            *state.pending_live_play.borrow_mut() = None;
            if let Some(new_surf) = load_and_prescale(path, mon_w, mon_h) {
                *state.current_surface.borrow_mut() = Some(new_surf);

                if let Some(prev) = prev_surf {
                    *state.old_surface.borrow_mut() = Some(prev);
                    start_transition_animation(da, live_pic, state);
                } else {
                    state.transition_progress.set(1.0);
                    da.queue_draw();
                }
            }
        }
    } else {
        *state.old_surface.borrow_mut() = None;
        *state.current_surface.borrow_mut() = None;
        *state.current_wp_path.borrow_mut() = None;
        *state.pending_live_play.borrow_mut() = None;
        state.transition_progress.set(1.0);
        da.queue_draw();
    }
}

/// Runs the 60/120Hz tick callback animating the circular water ripple transition.
fn start_transition_animation(
    da: &gtk4::DrawingArea,
    live_pic: &gtk4::Picture,
    state: &WallpaperState,
) {
    state.transition_progress.set(0.0);
    state.active_start_time.set(None);

    if state.is_animating.get() {
        return;
    }
    state.is_animating.set(true);

    let da_tick = da.clone();
    let live_pic_tick = live_pic.clone();
    let state_tick = state.clone();

    da.add_tick_callback(move |_, clock| {
        let now = clock.frame_time();
        if state_tick.active_start_time.get().is_none() {
            state_tick.active_start_time.set(Some(now));
        }

        let start = state_tick.active_start_time.get().unwrap();
        let elapsed = (now - start) as f64;
        let progress = (elapsed / TRANSITION_DURATION_US).clamp(0.0, 1.0);
        state_tick.transition_progress.set(progress);
        da_tick.queue_draw();

        if progress >= 1.0 {
            *state_tick.old_surface.borrow_mut() = None;
            state_tick.is_animating.set(false);

            // After ripple completes: seamlessly start live video/GIF playback
            if let Some((live_path, live_mode, pre_rolled_mf)) =
                state_tick.pending_live_play.borrow_mut().take()
            {
                start_live_media(
                    &live_path,
                    &live_mode,
                    &live_pic_tick,
                    &da_tick,
                    &state_tick.active_media_file,
                    &state_tick.gif_source_id,
                    &state_tick.recycle_source_id,
                    pre_rolled_mf,
                );
            }

            glib::ControlFlow::Break
        } else {
            glib::ControlFlow::Continue
        }
    });
}
