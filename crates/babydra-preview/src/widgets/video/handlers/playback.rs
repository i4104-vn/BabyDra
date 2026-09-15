//! Video playback start/pause toggles and ended state management.

use babydra_core::models::preview::VideoState;
use crate::widgets::video::render::VideoViewerUi;
use gtk4::prelude::*;
use gtk4::GestureClick;
use std::cell::RefCell;
use std::rc::Rc;

/// Sets up playback toggle controls (button, picture click, and ended callback).
pub fn setup_playback_controls(state: &Rc<RefCell<VideoState>>, ui: &VideoViewerUi) {
    let mf_toggle = ui.media_file.clone();
    let btn_toggle = ui.play_pause_btn.clone();
    let state_toggle = state.clone();

    let toggle_playback = move || {
        let is_playing = mf_toggle.is_playing();
        if is_playing {
            mf_toggle.pause();
            btn_toggle.set_icon_name("media-playback-start-symbolic");
            state_toggle.borrow_mut().is_playing = false;
        } else {
            if mf_toggle.is_ended() {
                mf_toggle.seek(0);
            }
            mf_toggle.play();
            btn_toggle.set_icon_name("media-playback-pause-symbolic");
            state_toggle.borrow_mut().is_playing = true;
        }
    };

    let toggle_btn_cb = toggle_playback.clone();
    ui.play_pause_btn.connect_clicked(move |_| {
        toggle_btn_cb();
    });

    let click_gesture = GestureClick::new();
    let toggle_click = toggle_playback.clone();
    click_gesture.connect_pressed(move |_, _, _, _| {
        toggle_click();
    });
    ui.picture.add_controller(click_gesture);

    let btn_ended = ui.play_pause_btn.clone();
    let state_ended = state.clone();
    ui.media_file.connect_ended_notify(move |mf| {
        if mf.is_ended() {
            btn_ended.set_icon_name("media-playback-start-symbolic");
            state_ended.borrow_mut().is_playing = false;
        }
    });
}
