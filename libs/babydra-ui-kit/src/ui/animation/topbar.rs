//! Topbar startup cascading animation.
//! Stacks all elements at the top-left as a compact circle, then reveals each capsule
//! in sequence from left to right across the top bar with wide, fluid slide travel.

use std::cell::Cell;
use std::rc::Rc;

use super::easing;
use gtk4::prelude::*;

/// Plays the startup cascading reveal animation for the topbar with clear staggered phases
/// and sweeping sliding travel distances.
pub fn topbar_startup_cascade(
    window: &gtk4::ApplicationWindow,
    workspace_box: &gtk4::Box,
    logo_btn: &gtk4::Button,
    notch_capsule: &gtk4::Widget,
    tray_widget: &gtk4::Widget,
    system_monitor: &gtk4::Widget,
    status_indicators: &gtk4::Widget,
) {
    // Disable CSS transitions during frame-clock tick animation to prevent stuttering
    window.add_css_class("startup-animating");

    // Measure natural target width for the workspace box
    let (_, nat_w, _, _) = workspace_box.measure(gtk4::Orientation::Horizontal, -1);
    let target_ws_w = nat_w.max(120);

    // Initial state: workspace box starts as circle (36x36) around logo
    workspace_box.set_size_request(36, 36);
    logo_btn.set_opacity(1.0);

    let logo_widget = logo_btn.clone().upcast::<gtk4::Widget>();
    let mut child = workspace_box.first_child();
    while let Some(w) = child {
        if w != logo_widget {
            w.set_opacity(0.0);
        }
        child = w.next_sibling();
    }

    // Wide sweeping start offsets
    notch_capsule.set_opacity(0.0);
    notch_capsule.set_margin_start(-400);

    tray_widget.set_opacity(0.0);
    tray_widget.set_margin_start(-300);

    system_monitor.set_opacity(0.0);
    system_monitor.set_margin_start(-400);

    status_indicators.set_opacity(0.0);
    status_indicators.set_margin_start(-600);

    let start_time = Rc::new(Cell::new(0i64));
    let total_duration_us = 1_600_000i64; // 1.6s total cinematic duration

    let win_weak = window.downgrade();
    let ws_clone = workspace_box.clone();
    let logo_clone = logo_btn.clone();
    let notch_clone = notch_capsule.clone();
    let tray_clone = tray_widget.clone();
    let sysmon_clone = system_monitor.clone();
    let status_clone = status_indicators.clone();

    window.add_tick_callback(move |_win, clock| {
        let now = clock.frame_time();
        if start_time.get() == 0 {
            start_time.set(now);
        }
        let elapsed_us = now - start_time.get();
        let elapsed_ms = (elapsed_us as f64) / 1000.0;

        if elapsed_us >= total_duration_us {
            // Restore normal transitions and clean properties
            if let Some(target_win) = win_weak.upgrade() {
                target_win.remove_css_class("startup-animating");
            }

            ws_clone.set_size_request(-1, 36);
            let mut c = ws_clone.first_child();
            while let Some(w) = c {
                w.set_opacity(1.0);
                c = w.next_sibling();
            }

            notch_clone.set_opacity(1.0);
            notch_clone.set_margin_start(0);

            tray_clone.set_opacity(1.0);
            tray_clone.set_margin_start(0);

            sysmon_clone.set_opacity(1.0);
            sysmon_clone.set_margin_start(0);

            status_clone.set_opacity(1.0);
            status_clone.set_margin_start(0);

            return glib::ControlFlow::Break;
        }

        // --- Phase 1: Left capsule expands from circle to workspace pill (0ms -> 450ms) ---
        if elapsed_ms <= 480.0 {
            let t1 = (elapsed_ms / 450.0).clamp(0.0, 1.0);
            let eased1 = easing::ease_out_cubic(t1);
            let current_w = 36.0 + (target_ws_w as f64 - 36.0) * eased1;
            ws_clone.set_size_request(current_w.max(36.0) as i32, 36);

            let child_opacity = ((elapsed_ms - 80.0) / 260.0).clamp(0.0, 1.0);
            let logo_w = logo_clone.clone().upcast::<gtk4::Widget>();
            let mut c = ws_clone.first_child();
            while let Some(w) = c {
                if w != logo_w {
                    w.set_opacity(child_opacity);
                }
                c = w.next_sibling();
            }
        }

        // --- Phase 2: Dynamic Island glides across from left to center (350ms -> 950ms) ---
        if elapsed_ms >= 350.0 {
            let t2 = ((elapsed_ms - 350.0) / 600.0).clamp(0.0, 1.0);
            let eased2 = easing::ease_out_quart(t2);
            notch_clone.set_opacity(eased2.min(1.0));
            let offset = (-400.0 * (1.0 - eased2)) as i32;
            notch_clone.set_margin_start(offset);
        }

        // --- Phase 3: Tray widget glides across (650ms -> 1200ms) ---
        if elapsed_ms >= 650.0 {
            let t3 = ((elapsed_ms - 650.0) / 550.0).clamp(0.0, 1.0);
            let eased3 = easing::ease_out_quart(t3);
            tray_clone.set_opacity(eased3.min(1.0));
            let offset = (-300.0 * (1.0 - eased3)) as i32;
            tray_clone.set_margin_start(offset);
        }

        // --- Phase 4: System monitor glides across (750ms -> 1300ms) ---
        if elapsed_ms >= 750.0 {
            let t4 = ((elapsed_ms - 750.0) / 550.0).clamp(0.0, 1.0);
            let eased4 = easing::ease_out_quart(t4);
            sysmon_clone.set_opacity(eased4.min(1.0));
            let offset = (-400.0 * (1.0 - eased4)) as i32;
            sysmon_clone.set_margin_start(offset);
        }

        // --- Phase 5: Status indicators sweep across and dock to far right (950ms -> 1600ms) ---
        if elapsed_ms >= 950.0 {
            let t5 = ((elapsed_ms - 950.0) / 650.0).clamp(0.0, 1.0);
            let eased5 = easing::ease_out_quart(t5);
            status_clone.set_opacity(eased5.min(1.0));
            let offset = (-600.0 * (1.0 - eased5)) as i32;
            status_clone.set_margin_start(offset);
        }

        glib::ControlFlow::Continue
    });
}
