//! Autohide controller for bottom controls bar and top-right info card.

use crate::widgets::video::render::VideoViewerUi;
use babydra_core::models::preview::VideoState;
use gtk4::prelude::*;
use gtk4::EventControllerMotion;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

/// Sets up autohide timer and motion-reveal controllers for controls_box and info_box.
pub fn setup_autohide_controls(
    state: &Rc<RefCell<VideoState>>,
    ui: &VideoViewerUi,
    is_popover_open: Rc<Cell<bool>>,
) {
    let last_mouse_pos = Rc::new(Cell::new(None::<(f64, f64)>));
    let last_activity = Rc::new(Cell::new(Instant::now()));
    let are_controls_visible = Rc::new(Cell::new(true));

    // 1. Motion controller on main window: physical movement reveals both simultaneously
    let motion_window = EventControllerMotion::new();
    let act_win = last_activity.clone();
    let pos_win = last_mouse_pos.clone();
    let visible_win = are_controls_visible.clone();
    let controls_rev_win = ui.controls_revealer.clone();
    let info_rev_win = ui.info_revealer.clone();
    let window_win = ui.window.clone();

    motion_window.connect_motion(move |_, x, y| {
        let (prev_x, prev_y) = pos_win.get().unwrap_or((-9999.0, -9999.0));
        let dx = (x - prev_x).abs();
        let dy = (y - prev_y).abs();

        // Filter out synthetic/redraw/cursor pointer events where mouse didn't physically move
        if dx < 2.0 && dy < 2.0 {
            return;
        }

        pos_win.set(Some((x, y)));
        act_win.set(Instant::now());

        if !visible_win.get() {
            visible_win.set(true);
            controls_rev_win.set_reveal_child(true);
            info_rev_win.set_reveal_child(true);
            window_win.set_cursor_from_name(None);
        }
    });
    ui.window.add_controller(motion_window);

    // 2. Periodic check timer (every 100ms)
    let act_check = last_activity.clone();
    let state_check = state.clone();
    let controls_rev_check = ui.controls_revealer.clone();
    let info_rev_check = ui.info_revealer.clone();
    let details_check = ui.details_box.clone();
    let popover_check = is_popover_open.clone();
    let window_check = ui.window.clone();
    let visible_check = are_controls_visible.clone();

    glib::timeout_add_local(Duration::from_millis(100), move || {
        let is_seeking = state_check.borrow().is_seeking;
        let is_details = details_check.is_visible();
        let is_popover = popover_check.get();

        // While actively seeking, inspecting details, or selecting speed: keep visible
        if is_seeking || is_details || is_popover {
            act_check.set(Instant::now());
            if !visible_check.get() {
                visible_check.set(true);
                controls_rev_check.set_reveal_child(true);
                info_rev_check.set_reveal_child(true);
                window_check.set_cursor_from_name(None);
            }
            return glib::ControlFlow::Continue;
        }

        // Hide both simultaneously after 5s without physical mouse motion
        if visible_check.get() && act_check.get().elapsed() >= Duration::from_secs(5) {
            visible_check.set(false);
            controls_rev_check.set_reveal_child(false);
            info_rev_check.set_reveal_child(false);
            window_check.set_cursor_from_name(Some("none"));
        }

        glib::ControlFlow::Continue
    });
}
