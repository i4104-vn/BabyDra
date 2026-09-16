//! Video audio volume scale and mute button handlers.

use crate::widgets::video::render::VideoViewerUi;
use babydra_core::models::preview::VideoState;
use gtk4::prelude::*;
use gtk4::Button;
use std::cell::RefCell;
use std::rc::Rc;

/// Updates the volume button icon according to the active volume and mute state.
pub fn update_volume_icon(btn: &Button, volume: f64, is_muted: bool) {
    let icon_name = if is_muted || volume <= 0.001 {
        "volume-mute"
    } else if volume < 0.5 {
        "volume-low"
    } else {
        "volume"
    };
    let icon = babydra_ui_kit::ui::icon::get_icon(icon_name, 16);
    btn.set_child(Some(&icon));
}

/// Sets up volume slider adjustment and mute toggling.
pub fn setup_volume_controls(state: &Rc<RefCell<VideoState>>, ui: &VideoViewerUi) {
    let mf_vol = ui.media_file.clone();
    let mute_btn_vol = ui.mute_btn.clone();
    let state_vol = state.clone();

    ui.volume_scale.connect_value_changed(move |scale| {
        let val = scale.value();
        mf_vol.set_volume(val);
        let mut st = state_vol.borrow_mut();
        st.volume = val;
        let is_muted = mf_vol.is_muted();
        update_volume_icon(&mute_btn_vol, val, is_muted);
    });

    let mf_mute = ui.media_file.clone();
    let mute_btn_clone = ui.mute_btn.clone();
    let state_mute = state.clone();
    ui.mute_btn.connect_clicked(move |_| {
        let new_muted = !mf_mute.is_muted();
        mf_mute.set_muted(new_muted);
        let mut st = state_mute.borrow_mut();
        st.is_muted = new_muted;
        let vol = mf_mute.volume();
        update_volume_icon(&mute_btn_clone, vol, new_muted);
    });
}
