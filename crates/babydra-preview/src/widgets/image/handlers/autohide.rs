//! Autohide controller for image controls bar and top-right info card.

use crate::widgets::image::render::ImageViewerUi;
use gtk4::prelude::*;
use gtk4::EventControllerMotion;
use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

/// Sets up autohide timer and hover-reveal controllers for controls_box and info_box in image preview.
pub fn setup_autohide_controls(ui: &ImageViewerUi) {
    let last_controls_activity = Rc::new(Cell::new(Instant::now()));
    let last_info_activity = Rc::new(Cell::new(Instant::now()));
    let last_motion_anywhere = Rc::new(Cell::new(Instant::now()));

    let is_in_bottom_zone = Rc::new(Cell::new(false));
    let is_in_top_right_zone = Rc::new(Cell::new(false));
    let is_controls_hovered = Rc::new(Cell::new(false));
    let is_info_hovered = Rc::new(Cell::new(false));

    // 1. Motion controller on main window for coordinate zones and cursor unhiding
    let motion_window = EventControllerMotion::new();
    let act_ctrl_win = last_controls_activity.clone();
    let act_info_win = last_info_activity.clone();
    let act_anywhere_win = last_motion_anywhere.clone();
    let in_bottom_win = is_in_bottom_zone.clone();
    let in_tr_win = is_in_top_right_zone.clone();
    let controls_rev_win = ui.controls_revealer.clone();
    let info_rev_win = ui.info_revealer.clone();
    let window_win = ui.window.clone();

    motion_window.connect_motion(move |_, x, y| {
        act_anywhere_win.set(Instant::now());
        window_win.set_cursor_from_name(None);

        let win_w = window_win.allocated_width().max(window_win.width()).max(1) as f64;
        let win_h = window_win.allocated_height().max(window_win.height()).max(1) as f64;

        // Bottom zone: bottom 90px (controls position)
        let in_bottom = y >= (win_h - 90.0).max(0.0);
        if in_bottom {
            in_bottom_win.set(true);
            act_ctrl_win.set(Instant::now());
            controls_rev_win.set_reveal_child(true);
        } else if in_bottom_win.get() {
            in_bottom_win.set(false);
            act_ctrl_win.set(Instant::now());
        }

        // Top-right zone: right 280px, top 90px (info position)
        let in_tr = x >= (win_w - 280.0).max(0.0) && y <= 90.0;
        if in_tr {
            in_tr_win.set(true);
            act_info_win.set(Instant::now());
            info_rev_win.set_reveal_child(true);
        } else if in_tr_win.get() {
            in_tr_win.set(false);
            act_info_win.set(Instant::now());
        }
    });

    let in_bottom_leave = is_in_bottom_zone.clone();
    let in_tr_leave = is_in_top_right_zone.clone();
    let act_ctrl_leave_win = last_controls_activity.clone();
    let act_info_leave_win = last_info_activity.clone();
    motion_window.connect_leave(move |_| {
        if in_bottom_leave.get() {
            in_bottom_leave.set(false);
            act_ctrl_leave_win.set(Instant::now());
        }
        if in_tr_leave.get() {
            in_tr_leave.set(false);
            act_info_leave_win.set(Instant::now());
        }
    });
    ui.window.add_controller(motion_window);

    // 2. Direct hover tracking on bottom controls bar
    let motion_controls = EventControllerMotion::new();
    let act_ctrl = last_controls_activity.clone();
    let hover_ctrl = is_controls_hovered.clone();
    let rev_ctrl = ui.controls_revealer.clone();
    let window_ctrl = ui.window.clone();

    motion_controls.connect_enter(move |_, _, _| {
        hover_ctrl.set(true);
        act_ctrl.set(Instant::now());
        rev_ctrl.set_reveal_child(true);
        window_ctrl.set_cursor_from_name(None);
    });

    let hover_ctrl_leave = is_controls_hovered.clone();
    let act_ctrl_leave = last_controls_activity.clone();
    motion_controls.connect_leave(move |_| {
        hover_ctrl_leave.set(false);
        act_ctrl_leave.set(Instant::now());
    });
    ui.controls_box.add_controller(motion_controls);

    // 3. Direct hover tracking on top-right info card
    let motion_info = EventControllerMotion::new();
    let act_inf = last_info_activity.clone();
    let hover_inf = is_info_hovered.clone();
    let rev_inf = ui.info_revealer.clone();

    motion_info.connect_enter(move |_, _, _| {
        hover_inf.set(true);
        act_inf.set(Instant::now());
        rev_inf.set_reveal_child(true);
    });

    let hover_inf_leave = is_info_hovered.clone();
    let act_inf_leave = last_info_activity.clone();
    motion_info.connect_leave(move |_| {
        hover_inf_leave.set(false);
        act_inf_leave.set(Instant::now());
    });
    ui.info_box.add_controller(motion_info);

    // 4. Periodic check timer (every 100ms)
    let act_ctrl_check = last_controls_activity.clone();
    let act_info_check = last_info_activity.clone();
    let act_any_check = last_motion_anywhere.clone();
    let in_bottom_check = is_in_bottom_zone.clone();
    let in_tr_check = is_in_top_right_zone.clone();
    let hover_c_check = is_controls_hovered.clone();
    let hover_i_check = is_info_hovered.clone();
    let controls_rev_check = ui.controls_revealer.clone();
    let info_rev_check = ui.info_revealer.clone();
    let exif_check = ui.exif_box.clone();
    let window_check = ui.window.clone();

    glib::timeout_add_local(Duration::from_millis(100), move || {
        let is_exif = exif_check.is_visible();

        if is_exif {
            act_ctrl_check.set(Instant::now());
            act_info_check.set(Instant::now());
            controls_rev_check.set_reveal_child(true);
            info_rev_check.set_reveal_child(true);
        }

        // Auto-hide controls bar after 5s of no focus/hover
        let controls_focused = in_bottom_check.get() || hover_c_check.get() || is_exif;
        if controls_focused {
            act_ctrl_check.set(Instant::now());
        } else if act_ctrl_check.get().elapsed() >= Duration::from_secs(5) {
            controls_rev_check.set_reveal_child(false);
        }

        // Auto-hide info card after 5s of no focus/hover
        let info_focused = in_tr_check.get() || hover_i_check.get() || is_exif;
        if info_focused {
            act_info_check.set(Instant::now());
        } else if act_info_check.get().elapsed() >= Duration::from_secs(5) {
            info_rev_check.set_reveal_child(false);
        }

        // Hide cursor if everything is hidden and no mouse movement for 5s
        if !controls_focused
            && !info_focused
            && !controls_rev_check.reveals_child()
            && !info_rev_check.reveals_child()
            && act_any_check.get().elapsed() >= Duration::from_secs(5)
        {
            window_check.set_cursor_from_name(Some("none"));
        }

        glib::ControlFlow::Continue
    });
}
