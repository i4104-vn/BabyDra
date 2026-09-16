//! Keyboard shortcut controller and EXIF visibility toggling for image viewer.

use crate::widgets::image::handlers::zoom::{do_zoom, fit_to_screen, update_zoom_display};
use crate::widgets::image::render::{populate_exif_dialog, show_exif_loading, ImageViewerUi};
use babydra_core::models::preview::ImageState;
use babydra_core::models::shell::exif::ExifData;
use gtk4::prelude::*;
use gtk4::EventControllerKey;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// Sets up keyboard shortcuts for zoom manipulation and EXIF dialog inspection.
pub fn setup_key_controller(state: &Rc<RefCell<ImageState>>, ui: &ImageViewerUi, path: PathBuf) {
    let key_controller = EventControllerKey::new();
    let state_key = state.clone();
    let area_key = ui.drawing_area.clone();
    let lbl_key = ui.scale_lbl.clone();
    let exif_box_clone = ui.exif_box.clone();
    let info_box_clone = ui.info_box.clone();
    let controls_box_clone = ui.controls_box.clone();

    // Cache for EXIF data: None = not yet loaded, Some(data) = cached
    let exif_cache: Rc<RefCell<Option<Option<ExifData>>>> = Rc::new(RefCell::new(None));
    let is_loading = Rc::new(RefCell::new(false));

    let path_clone = path;
    let exif_cache_pressed = exif_cache.clone();
    let is_loading_pressed = is_loading.clone();
    let exif_box_press = exif_box_clone.clone();

    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        match keyval.name().as_deref() {
            Some("i") | Some("I") => {
                exif_box_press.set_visible(true);
                info_box_clone.set_visible(false);
                controls_box_clone.set_visible(false);

                // Lazy load EXIF in background thread if not already loaded or loading
                let is_loaded = exif_cache_pressed.borrow().is_some();
                let already_loading = *is_loading_pressed.borrow();

                if !is_loaded && !already_loading {
                    *is_loading_pressed.borrow_mut() = true;
                    show_exif_loading(&exif_box_press);

                    let (tx, rx) = std::sync::mpsc::channel::<Option<ExifData>>();
                    let p = path_clone.clone();
                    std::thread::spawn(move || {
                        let data = babydra_core::read_exif(&p);
                        let _ = tx.send(data);
                    });

                    let cache_res = exif_cache_pressed.clone();
                    let loading_res = is_loading_pressed.clone();
                    let box_res = exif_box_press.clone();

                    let mut rx_opt = Some(rx);
                    glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
                        if let Some(ref rx_chan) = rx_opt {
                            if let Ok(data) = rx_chan.try_recv() {
                                rx_opt = None;
                                *loading_res.borrow_mut() = false;
                                populate_exif_dialog(&box_res, data.as_ref());
                                *cache_res.borrow_mut() = Some(data);
                                return glib::ControlFlow::Break;
                            }
                        }
                        glib::ControlFlow::Continue
                    });
                }
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
