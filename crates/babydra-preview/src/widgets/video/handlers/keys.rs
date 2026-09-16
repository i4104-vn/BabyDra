//! Keyboard shortcut controller and metadata details inspection for video viewer.

use crate::widgets::video::render::{
    format_duration, populate_video_details, show_video_details_loading, VideoViewerUi,
};
use crate::widgets::utils::format_dimensions;
use babydra_core::models::preview::VideoMetadata;
use gtk4::prelude::*;
use gtk4::EventControllerKey;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// Sets up keyboard navigation and lazy 'i' key handling for video preview.
pub fn setup_key_controller(ui: &VideoViewerUi, path: PathBuf) {
    let key_controller = EventControllerKey::new();
    let play_btn_key = ui.play_pause_btn.clone();
    let mf_key = ui.media_file.clone();
    let vol_scale_key = ui.volume_scale.clone();
    let details_box_clone = ui.details_box.clone();
    let info_box_clone = ui.info_box.clone();
    let controls_box_clone = ui.controls_box.clone();
    let mute_key = ui.mute_btn.clone();
    let meta_lbl_key = ui.meta_lbl.clone();

    // Cache for probe_video metadata: None = not yet loaded, Some(meta) = cached
    let meta_cache: Rc<RefCell<Option<VideoMetadata>>> = Rc::new(RefCell::new(None));
    let is_loading = Rc::new(RefCell::new(false));

    let path_clone = path;
    let meta_cache_pressed = meta_cache.clone();
    let is_loading_pressed = is_loading.clone();
    let details_box_press = details_box_clone.clone();

    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        match keyval.name().as_deref() {
            Some("space") => {
                play_btn_key.emit_clicked();
            }
            Some("i") | Some("I") => {
                details_box_press.set_visible(true);
                info_box_clone.set_visible(false);
                controls_box_clone.set_visible(false);

                // Lazy load video stream metadata via ffprobe in background thread if needed
                let is_loaded = meta_cache_pressed.borrow().is_some();
                let already_loading = *is_loading_pressed.borrow();

                if !is_loaded && !already_loading {
                    *is_loading_pressed.borrow_mut() = true;
                    show_video_details_loading(&details_box_press);

                    let (tx, rx) = std::sync::mpsc::channel::<VideoMetadata>();
                    let p = path_clone.clone();
                    std::thread::spawn(move || {
                        let meta = babydra_core::services::preview::probe_video(&p);
                        let _ = tx.send(meta);
                    });

                    let cache_res = meta_cache_pressed.clone();
                    let loading_res = is_loading_pressed.clone();
                    let box_res = details_box_press.clone();
                    let lbl_res = meta_lbl_key.clone();

                    let mut rx_opt = Some(rx);
                    glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
                        if let Some(ref rx_chan) = rx_opt {
                            if let Ok(meta) = rx_chan.try_recv() {
                                rx_opt = None;
                                *loading_res.borrow_mut() = false;
                                populate_video_details(&box_res, &meta);

                                // Update info overlay with exact resolution and container specs
                                let res_text = format_dimensions(meta.width, meta.height);
                                let meta_text = format!(
                                    "{} • {} • {}",
                                    res_text,
                                    format_duration(meta.duration_secs),
                                    babydra_ui_kit::components::explore::format_size(
                                        meta.file_size
                                    )
                                );
                                lbl_res.set_text(&meta_text);

                                *cache_res.borrow_mut() = Some(meta);
                                return glib::ControlFlow::Break;
                            }
                        }
                        glib::ControlFlow::Continue
                    });
                }
            }
            Some("Left") => {
                let cur = mf_key.timestamp();
                let target = (cur - 5_000_000).max(0);
                mf_key.seek(target);
            }
            Some("Right") => {
                let cur = mf_key.timestamp();
                let dur = mf_key.duration();
                let target = (cur + 5_000_000).min(dur);
                mf_key.seek(target);
            }
            Some("Up") => {
                let cur_vol = vol_scale_key.value();
                vol_scale_key.set_value((cur_vol + 0.05).min(1.0));
            }
            Some("Down") => {
                let cur_vol = vol_scale_key.value();
                vol_scale_key.set_value((cur_vol - 0.05).max(0.0));
            }
            Some("m") | Some("M") => {
                mute_key.emit_clicked();
            }
            _ => {}
        }
        gtk4::glib::Propagation::Proceed
    });

    let details_rel = ui.details_box.clone();
    let info_rel = ui.info_box.clone();
    let controls_rel = ui.controls_box.clone();

    key_controller.connect_key_released(move |_, keyval, _, _| {
        if let Some("i") | Some("I") = keyval.name().as_deref() {
            details_rel.set_visible(false);
            info_rel.set_visible(true);
            controls_rel.set_visible(true);
        }
    });

    ui.window.add_controller(key_controller);
}
