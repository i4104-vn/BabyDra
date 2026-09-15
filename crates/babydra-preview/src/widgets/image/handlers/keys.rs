//! Keyboard shortcut controller and EXIF visibility toggling for image viewer.

use crate::widgets::image::handlers::zoom::{do_zoom, fit_to_screen, update_zoom_display};
use crate::widgets::image::render::ImageViewerUi;
use babydra_core::models::preview::ImageState;
use gtk4::prelude::*;
use gtk4::EventControllerKey;
use std::cell::RefCell;
use std::rc::Rc;

/// Sets up keyboard shortcuts for zoom manipulation and EXIF dialog inspection.
pub fn setup_key_controller(state: &Rc<RefCell<ImageState>>, ui: &ImageViewerUi) {
    let key_controller = EventControllerKey::new();
    let state_key = state.clone();
    let area_key = ui.drawing_area.clone();
    let lbl_key = ui.scale_lbl.clone();
    let exif_box_clone = ui.exif_box.clone();
    let info_box_clone = ui.info_box.clone();
    let controls_box_clone = ui.controls_box.clone();

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

    let exif_box_rel = ui.exif_box.clone();
    let info_box_rel = ui.info_box.clone();
    let controls_box_rel = ui.controls_box.clone();
    key_controller.connect_key_released(move |_, keyval, _, _| {
        if let Some("i") | Some("I") = keyval.name().as_deref() {
            exif_box_rel.set_visible(false);
            info_box_rel.set_visible(true);
            controls_box_rel.set_visible(true);
        }
    });

    ui.window.add_controller(key_controller);
}
