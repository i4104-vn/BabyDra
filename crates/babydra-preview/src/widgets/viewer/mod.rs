//! Image viewer widget: zoom/pan state, math helpers and interaction wiring.

mod render;

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

struct ImageState {
    pixbuf: gdk_pixbuf::Pixbuf,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
    min_scale: f64,
    img_w: f64,
    img_h: f64,
    // Drag gestures tracking
    drag_start_x: f64,
    drag_start_y: f64,
}

/// Clamps the image position so it cannot be dragged away from the viewport.
fn clamp_position(state_ref: &mut ImageState, area_w: f64, area_h: f64) {
    let scaled_w = state_ref.img_w * state_ref.scale;
    let scaled_h = state_ref.img_h * state_ref.scale;

    let limit_x = ((scaled_w - area_w) / 2.0).max(0.0);
    let limit_y = ((scaled_h - area_h) / 2.0).max(0.0);

    state_ref.offset_x = state_ref.offset_x.clamp(-limit_x, limit_x);
    state_ref.offset_y = state_ref.offset_y.clamp(-limit_y, limit_y);
}

/// Updates the zoom percentage label.
fn update_zoom_display(state_ref: &ImageState, lbl: &gtk4::Label) {
    lbl.set_text(&format!("{:.0}%", state_ref.scale * 100.0));
}

/// Fits the image to the viewport and sets the minimum zoom scale.
fn fit_to_screen(state_ref: &mut ImageState, area_w: f64, area_h: f64) {
    let scale_x = area_w / state_ref.img_w;
    let scale_y = area_h / state_ref.img_h;
    state_ref.min_scale = scale_x.min(scale_y).min(1.0).max(0.05);
    state_ref.scale = state_ref.min_scale;
    state_ref.offset_x = 0.0;
    state_ref.offset_y = 0.0;
}

/// Applies a zoom delta around the current position, clamped to the allowed range.
fn do_zoom(
    state: &Rc<RefCell<ImageState>>,
    area: &gtk4::DrawingArea,
    lbl: &gtk4::Label,
    delta: f64,
) {
    let mut state_ref = state.borrow_mut();
    let area_w = area.width() as f64;
    let area_h = area.height() as f64;

    let next_scale = (state_ref.scale + delta).clamp(state_ref.min_scale, 5.0);
    state_ref.scale = next_scale;

    if state_ref.scale <= state_ref.min_scale + 0.001 {
        state_ref.offset_x = 0.0;
        state_ref.offset_y = 0.0;
    } else {
        clamp_position(&mut *state_ref, area_w, area_h);
    }

    update_zoom_display(&state_ref, lbl);
    area.queue_draw();
}

/// Builds and presents the image viewer window.
pub fn build_ui(app: &gtk4::Application, path: PathBuf) {
    let pixbuf = match gdk_pixbuf::Pixbuf::from_file(&path) {
        Ok(pb) => pb,
        Err(_) => {
            let err_window = gtk4::ApplicationWindow::new(app);
            err_window.set_default_size(600, 400);
            err_window.add_css_class("viewer-window");
            let err_label = gtk4::Label::new(Some(&trans("preview.failed_load")));
            err_label.add_css_class("brand-text");
            err_window.set_child(Some(&err_label));
            err_window.present();
            return;
        }
    };

    let img_w = pixbuf.width() as f64;
    let img_h = pixbuf.height() as f64;

    let state = Rc::new(RefCell::new(ImageState {
        pixbuf,
        scale: 1.0,
        offset_x: 0.0,
        offset_y: 0.0,
        min_scale: 0.1,
        img_w,
        img_h,
        drag_start_x: 0.0,
        drag_start_y: 0.0,
    }));

    let render::ViewerUi {
        window,
        drawing_area,
        scale_lbl,
        info_box,
        controls_box,
        exif_box,
        zoom_out_btn,
        reset_btn,
        zoom_in_btn,
    } = render::build_viewer_ui(app, &path, img_w as u32, img_h as u32);

    // --- Helpers / Closures ---
    let state_draw = state.clone();
    drawing_area.set_draw_func(move |_area, cr, width, height| {
        let state_ref = state_draw.borrow();
        let w = width as f64;
        let h = height as f64;

        // Draw Dark Background
        cr.set_source_rgb(15.0 / 255.0, 15.0 / 255.0, 15.0 / 255.0);
        cr.paint().unwrap();

        // Calculate layout coordinates
        let draw_w = state_ref.img_w * state_ref.scale;
        let draw_h = state_ref.img_h * state_ref.scale;
        let start_x = (w - draw_w) / 2.0 + state_ref.offset_x;
        let start_y = (h - draw_h) / 2.0 + state_ref.offset_y;

        cr.save().unwrap();
        cr.translate(start_x, start_y);
        cr.scale(state_ref.scale, state_ref.scale);
        cr.set_source_pixbuf(&state_ref.pixbuf, 0.0, 0.0);
        cr.paint().unwrap();
        cr.restore().unwrap();
    });

    // --- Event Handlers & Gestures ---
    let state_resize = state.clone();
    let area_clone = drawing_area.clone();
    let scale_lbl_clone = scale_lbl.clone();
    drawing_area.connect_resize(move |_, w, h| {
        let mut state_ref = state_resize.borrow_mut();
        fit_to_screen(&mut state_ref, w as f64, h as f64);
        update_zoom_display(&state_ref, &scale_lbl_clone);
        area_clone.queue_draw();
    });

    let state_out = state.clone();
    let area_out = drawing_area.clone();
    let lbl_out = scale_lbl.clone();
    zoom_out_btn.connect_clicked(move |_| {
        do_zoom(&state_out, &area_out, &lbl_out, -0.1);
    });

    let state_in = state.clone();
    let area_in = drawing_area.clone();
    let lbl_in = scale_lbl.clone();
    zoom_in_btn.connect_clicked(move |_| {
        do_zoom(&state_in, &area_in, &lbl_in, 0.1);
    });

    let state_reset = state.clone();
    let area_reset = drawing_area.clone();
    let lbl_reset = scale_lbl.clone();
    reset_btn.connect_clicked(move |_| {
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
    let scroll_controller =
        gtk4::EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);
    let state_scroll = state.clone();
    let area_scroll = drawing_area.clone();
    let lbl_scroll = scale_lbl.clone();
    scroll_controller.connect_scroll(move |_, _, dy| {
        let delta = if dy > 0.0 { -0.1 } else { 0.1 };
        do_zoom(&state_scroll, &area_scroll, &lbl_scroll, delta);
        gtk4::glib::Propagation::Proceed
    });
    drawing_area.add_controller(scroll_controller);

    // Mouse Drag Panning
    let drag_gesture = gtk4::GestureDrag::new();
    let state_drag = state.clone();
    drag_gesture.connect_drag_begin(move |gesture, _x, _y| {
        let mut state_ref = state_drag.borrow_mut();
        state_ref.drag_start_x = state_ref.offset_x;
        state_ref.drag_start_y = state_ref.offset_y;
        gesture.set_state(gtk4::EventSequenceState::Claimed);
    });

    let state_drag_update = state.clone();
    let area_drag_update = drawing_area.clone();
    drag_gesture.connect_drag_update(move |_, offset_x, offset_y| {
        let mut state_ref = state_drag_update.borrow_mut();
        let area_w = area_drag_update.width() as f64;
        let area_h = area_drag_update.height() as f64;

        state_ref.offset_x = state_ref.drag_start_x + offset_x;
        state_ref.offset_y = state_ref.drag_start_y + offset_y;

        clamp_position(&mut state_ref, area_w, area_h);
        area_drag_update.queue_draw();
    });
    drawing_area.add_controller(drag_gesture);

    // Key Events for Exif Box and Zoom Shortcuts
    let key_controller = gtk4::EventControllerKey::new();
    let state_key = state.clone();
    let area_key = drawing_area.clone();
    let lbl_key = scale_lbl.clone();
    let exif_box_clone = exif_box.clone();
    let info_box_clone = info_box.clone();
    let controls_box_clone = controls_box.clone();

    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        match keyval.name().as_deref() {
            Some("i") | Some("I") => {
                exif_box_clone.set_visible(true);
                info_box_clone.set_visible(false);
                controls_box_clone.set_visible(false);
            }
            Some("plus") | Some("equal") => {
                do_zoom(&state_key, &area_key, &lbl_key, 0.1);
            }
            Some("minus") => {
                do_zoom(&state_key, &area_key, &lbl_key, -0.1);
            }
            Some("0") => {
                let mut state_ref = state_key.borrow_mut();
                fit_to_screen(
                    &mut state_ref,
                    area_key.width() as f64,
                    area_key.height() as f64,
                );
                update_zoom_display(&state_ref, &lbl_key);
                area_key.queue_draw();
            }
            _ => {}
        }
        gtk4::glib::Propagation::Proceed
    });

    key_controller.connect_key_released(move |_, keyval, _, _| {
        if let Some("i") | Some("I") = keyval.name().as_deref() {
            exif_box.set_visible(false);
            info_box.set_visible(true);
            controls_box.set_visible(true);
        }
    });

    window.add_controller(key_controller);

    window.present();
}
