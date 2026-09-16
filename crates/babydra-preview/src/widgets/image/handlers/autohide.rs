//! Autohide controller for image controls bar and top-right info card.

use crate::widgets::image::render::ImageViewerUi;
use gtk4::prelude::*;
use gtk4::EventControllerMotion;
use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

/// Sets up autohide timer and hover-reveal controllers for controls_box and info_box in image preview.
pub fn setup_autohide_controls(ui: &ImageViewerUi) {
    let last_activity = Rc::new(Cell::new(Instant::now()));
    let is_hovered = Rc::new(Cell::new(false));

    // 1. Motion controller on main window: any movement shows both immediately
    let motion_window = EventControllerMotion::new();
    let act_win = last_activity.clone();
    let controls_rev_win = ui.controls_revealer.clone();
    let info_rev_win = ui.info_revealer.clone();
    let window_win = ui.window.clone();

    motion_window.connect_motion(move |_, _, _| {
        act_win.set(Instant::now());
        window_win.set_cursor_from_name(None);
        controls_rev_win.set_reveal_child(true);
        info_rev_win.set_reveal_child(true);
    });
    ui.window.add_controller(motion_window);

    // 2. Direct hover tracking on controls_box
    let motion_controls = EventControllerMotion::new();
    let act_ctrl = last_activity.clone();
    let hover_ctrl = is_hovered.clone();
    let rev_ctrl = ui.controls_revealer.clone();
    let rev_info_ctrl = ui.info_revealer.clone();
    let window_ctrl = ui.window.clone();

    motion_controls.connect_enter(move |_, _, _| {
        hover_ctrl.set(true);
        act_ctrl.set(Instant::now());
        rev_ctrl.set_reveal_child(true);
        rev_info_ctrl.set_reveal_child(true);
        window_ctrl.set_cursor_from_name(None);
    });

    let hover_ctrl_leave = is_hovered.clone();
    let act_ctrl_leave = last_activity.clone();
    motion_controls.connect_leave(move |_| {
        hover_ctrl_leave.set(false);
        act_ctrl_leave.set(Instant::now());
    });
    ui.controls_box.add_controller(motion_controls);

    // 3. Direct hover tracking on top-right info card
    let motion_info = EventControllerMotion::new();
    let act_inf = last_activity.clone();
    let hover_inf = is_hovered.clone();
    let rev_inf_ctrl = ui.controls_revealer.clone();
    let rev_inf = ui.info_revealer.clone();

    motion_info.connect_enter(move |_, _, _| {
        hover_inf.set(true);
        act_inf.set(Instant::now());
        rev_inf_ctrl.set_reveal_child(true);
        rev_inf.set_reveal_child(true);
    });

    let hover_inf_leave = is_hovered.clone();
    let act_inf_leave = last_activity.clone();
    motion_info.connect_leave(move |_| {
        hover_inf_leave.set(false);
        act_inf_leave.set(Instant::now());
    });
    ui.info_box.add_controller(motion_info);

    // 4. Periodic check timer (every 100ms)
    let act_check = last_activity.clone();
    let hover_check = is_hovered.clone();
    let controls_rev_check = ui.controls_revealer.clone();
    let info_rev_check = ui.info_revealer.clone();
    let exif_check = ui.exif_box.clone();
    let window_check = ui.window.clone();

    glib::timeout_add_local(Duration::from_millis(100), move || {
        let is_exif = exif_check.is_visible();
        let is_user_interacting = hover_check.get() || is_exif;

        if is_user_interacting {
            act_check.set(Instant::now());
            controls_rev_check.set_reveal_child(true);
            info_rev_check.set_reveal_child(true);
            window_check.set_cursor_from_name(None);
            return glib::ControlFlow::Continue;
        }

        // Hide both simultaneously after 5s of inactivity
        if act_check.get().elapsed() >= Duration::from_secs(5) {
            controls_rev_check.set_reveal_child(false);
            info_rev_check.set_reveal_child(false);
            window_check.set_cursor_from_name(Some("none"));
        }

        glib::ControlFlow::Continue
    });
}
