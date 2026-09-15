//! Playback speed popover options and rate application handlers.

use babydra_core::services::preview::SPEED_PRESETS;
use crate::widgets::playback::set_playback_speed;
use babydra_core::models::preview::VideoState;
use crate::widgets::video::render::VideoViewerUi;
use gtk4::prelude::*;
use gtk4::Button;
use std::cell::RefCell;
use std::rc::Rc;

/// Sets up speed popover presentation and speed selection button listeners.
pub fn setup_speed_controls(state: &Rc<RefCell<VideoState>>, ui: &VideoViewerUi) {
    let speed_btn_clone = ui.speed_btn.clone();
    let popover_clone = ui.speed_popover.clone();
    ui.speed_btn.connect_clicked(move |_| {
        popover_clone.popup();
    });

    if let Some(child) = ui.speed_popover.child() {
        if let Some(box_w) = child.downcast_ref::<gtk4::Box>() {
            let mut curr = box_w.first_child();
            let mut idx = 0;
            while let Some(w) = curr {
                if let Some(btn) = w.downcast_ref::<Button>() {
                    if idx < SPEED_PRESETS.len() {
                        let target_speed = SPEED_PRESETS[idx];
                        let mf_spd = ui.media_file.clone();
                        let spd_btn_label = speed_btn_clone.clone();
                        let pop_dismiss = ui.speed_popover.clone();
                        let state_spd = state.clone();

                        btn.connect_clicked(move |_| {
                            set_playback_speed(&mf_spd, target_speed);
                            state_spd.borrow_mut().speed = target_speed;
                            spd_btn_label.set_label(&format!("{:.2}x", target_speed).replace(".00", ".0").replace(".50", ".5").replace(".75", ".75").replace(".25", ".25"));
                            pop_dismiss.popdown();
                        });
                    }
                }
                idx += 1;
                curr = w.next_sibling();
            }
        }
    }
}
