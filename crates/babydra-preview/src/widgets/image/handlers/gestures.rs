//! Mouse gestures, viewport drag/scroll, and resize handlers for image viewer.

use crate::widgets::image::handlers::zoom::{clamp_position, do_zoom, fit_to_screen, update_zoom_display};
use crate::widgets::image::render::ImageViewerUi;
use babydra_core::models::preview::ImageState;
use gtk4::prelude::*;
use gtk4::{EventControllerScroll, EventControllerScrollFlags, EventSequenceState, GestureDrag};
use std::cell::RefCell;
use std::rc::Rc;

/// Sets up mouse gestures (scroll zoom, drag pan) and button zoom controls.
pub fn setup_gestures(state: &Rc<RefCell<ImageState>>, ui: &ImageViewerUi) {
    let state_resize = state.clone();
    let area_clone = ui.drawing_area.clone();
    let scale_lbl_clone = ui.scale_lbl.clone();
    ui.drawing_area.connect_resize(move |_, w, h| {
        let mut state_ref = state_resize.borrow_mut();
        fit_to_screen(&mut state_ref, w as f64, h as f64);
        update_zoom_display(&state_ref, &scale_lbl_clone);
        area_clone.queue_draw();
    });

    let state_out = state.clone();
    let area_out = ui.drawing_area.clone();
    let lbl_out = ui.scale_lbl.clone();
    ui.zoom_out_btn.connect_clicked(move |_| {
        do_zoom(&state_out, &area_out, &lbl_out, -0.1);
    });

    let state_in = state.clone();
    let area_in = ui.drawing_area.clone();
    let lbl_in = ui.scale_lbl.clone();
    ui.zoom_in_btn.connect_clicked(move |_| {
        do_zoom(&state_in, &area_in, &lbl_in, 0.1);
    });

    let state_reset = state.clone();
    let area_reset = ui.drawing_area.clone();
    let lbl_reset = ui.scale_lbl.clone();
    ui.reset_btn.connect_clicked(move |_| {
        let mut state_ref = state_reset.borrow_mut();
        fit_to_screen(
            &mut state_ref,
            area_reset.width() as f64,
            area_reset.height() as f64,
        );
        update_zoom_display(&state_ref, &lbl_reset);
        area_reset.queue_draw();
    });

    // Mouse Scroll Wheel Zoom
    let scroll_controller = EventControllerScroll::new(EventControllerScrollFlags::VERTICAL);
    let state_scroll = state.clone();
    let area_scroll = ui.drawing_area.clone();
    let lbl_scroll = ui.scale_lbl.clone();
    scroll_controller.connect_scroll(move |_, _, dy| {
        let delta = if dy > 0.0 { -0.1 } else { 0.1 };
        do_zoom(&state_scroll, &area_scroll, &lbl_scroll, delta);
        gtk4::glib::Propagation::Proceed
    });
    ui.drawing_area.add_controller(scroll_controller);

    // Mouse Drag Panning
    let drag_gesture = GestureDrag::new();
    let state_drag = state.clone();
    drag_gesture.connect_drag_begin(move |gesture, _x, _y| {
        let mut state_ref = state_drag.borrow_mut();
        state_ref.drag_start_x = state_ref.offset_x;
        state_ref.drag_start_y = state_ref.offset_y;
        gesture.set_state(EventSequenceState::Claimed);
    });

    let state_drag_update = state.clone();
    let area_drag_update = ui.drawing_area.clone();
    drag_gesture.connect_drag_update(move |_, offset_x, offset_y| {
        let mut state_ref = state_drag_update.borrow_mut();
        let area_w = area_drag_update.width() as f64;
        let area_h = area_drag_update.height() as f64;

        state_ref.offset_x = state_ref.drag_start_x + offset_x;
        state_ref.offset_y = state_ref.drag_start_y + offset_y;

        clamp_position(&mut state_ref, area_w, area_h);
        area_drag_update.queue_draw();
    });
    ui.drawing_area.add_controller(drag_gesture);
}
