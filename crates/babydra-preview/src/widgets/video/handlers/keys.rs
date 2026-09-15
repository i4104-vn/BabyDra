//! Keyboard shortcut controller and metadata details inspection for video viewer.

use crate::widgets::video::render::VideoViewerUi;
use gtk4::prelude::*;
use gtk4::EventControllerKey;

/// Sets up keyboard navigation and 'i' key handling for video preview.
pub fn setup_key_controller(ui: &VideoViewerUi) {
    let key_controller = EventControllerKey::new();
    let play_btn_key = ui.play_pause_btn.clone();
    let mf_key = ui.media_file.clone();
    let vol_scale_key = ui.volume_scale.clone();
    let details_box_clone = ui.details_box.clone();
    let info_box_clone = ui.info_box.clone();
    let controls_box_clone = ui.controls_box.clone();
    let mute_key = ui.mute_btn.clone();

    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        match keyval.name().as_deref() {
            Some("space") => {
                play_btn_key.emit_clicked();
            }
            Some("i") | Some("I") => {
                details_box_clone.set_visible(true);
                info_box_clone.set_visible(false);
                controls_box_clone.set_visible(false);
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
