//! Microphone / Audio Input event wiring.

use super::render::InputWidgets;
use babydra_core::models::volume::AudioDevice;
use babydra_core::services::system::volume::{
    is_microphone_muted, select_audio_source, set_microphone_muted, set_microphone_volume,
};
use gtk4::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

/// Connects signals for Microphone / Audio Input widgets.
pub fn wire_events(widgets: &InputWidgets, devices: Vec<AudioDevice>) {
    // 1. Source selection
    let devs_c = devices;
    widgets.dropdown.connect_selected_notify(move |dd| {
        let idx = dd.selected() as usize;
        if let Some(dev) = devs_c.get(idx) {
            select_audio_source(&dev.name);
        }
    });

    // 2. Microphone mute state & overlay button
    let mic_muted = Rc::new(Cell::new(is_microphone_muted()));

    let mic_muted_c1 = mic_muted.clone();
    let mute_switch_c1 = widgets.mute_switch.clone();
    widgets.mute_btn.connect_clicked(move |_| {
        let new_mute = !mic_muted_c1.get();
        mic_muted_c1.set(new_mute);
        set_microphone_muted(new_mute);
        mute_switch_c1.set_active(new_mute);
    });

    // 3. Microphone slider with debounce
    widgets.slider.connect_debounced(80, move |val| {
        set_microphone_volume(val);
    });

    // 4. Mute switch
    let mic_muted_c2 = mic_muted;
    widgets.mute_switch.connect_state_set(move |active| {
        mic_muted_c2.set(active);
        set_microphone_muted(active);
    });
}
