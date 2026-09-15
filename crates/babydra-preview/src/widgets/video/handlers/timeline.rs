//! Video timeline tracking and scrubbing seeking handlers.

use babydra_core::models::preview::VideoState;
use crate::widgets::video::render::{format_duration, VideoViewerUi};
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Sets up timeline synchronization and scrubber seek callbacks.
pub fn setup_timeline_controls(
    state: &Rc<RefCell<VideoState>>,
    ui: &VideoViewerUi,
    total_secs: f64,
) {
    let total_str = format_duration(total_secs);
    ui.total_time_lbl.set_text(&total_str);

    // Sync timeline scale and time label with media progress
    let time_lbl_clone = ui.time_lbl.clone();
    let timeline_clone = ui.timeline_scale.clone();
    let state_ts = state.clone();

    ui.media_file.connect_timestamp_notify(move |mf| {
        if state_ts.borrow().is_seeking {
            return;
        }

        let ts_us = mf.timestamp();
        state_ts.borrow_mut().position_us = ts_us;
        let cur_secs = (ts_us as f64) / 1_000_000.0;
        timeline_clone.set_value(cur_secs);

        let cur_str = format_duration(cur_secs);
        time_lbl_clone.set_text(&cur_str);
    });

    // Timeline Drag / Scrub Seeking
    let mf_seek = ui.media_file.clone();
    let time_lbl_seek = ui.time_lbl.clone();
    let state_seek = state.clone();

    ui.timeline_scale.connect_change_value(move |_, _, val| {
        state_seek.borrow_mut().is_seeking = true;

        let seek_us = (val * 1_000_000.0).round() as i64;
        mf_seek.seek(seek_us);

        let cur_str = format_duration(val);
        time_lbl_seek.set_text(&cur_str);

        let state_reset = state_seek.clone();
        glib::timeout_add_local_once(std::time::Duration::from_millis(80), move || {
            state_reset.borrow_mut().is_seeking = false;
        });

        gtk4::glib::Propagation::Proceed
    });
}
