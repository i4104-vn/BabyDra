//! Audio Output event wiring.

use super::render::OutputWidgets;
use babydra_core::models::volume::AudioDevice;
use babydra_core::services::system::volume::{is_muted, select_audio_device, set_muted, set_volume};
use gtk4::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

/// Connects signals for Audio Output widgets.
pub fn wire_events(widgets: &OutputWidgets, devices: Vec<AudioDevice>) {
    // 1. Device selection
    let devs_c = devices;
    widgets.dropdown.connect_selected_notify(move |dd| {
        let idx = dd.selected() as usize;
        if let Some(dev) = devs_c.get(idx) {
            select_audio_device(&dev.name);
        }
    });

    // 2. Mute icon updater
    let icon_box = widgets.mute_icon_box.clone();
    let update_icon = Rc::new(move |muted: bool| {
        if let Some(child) = icon_box.first_child() {
            icon_box.remove(&child);
        }
        let icon_name = if muted { "volume-mute" } else { "volume" };
        let img = babydra_ui_kit::ui::icon::get_icon_colored(icon_name, 16, "#ffffff");
        img.add_css_class("slider-icon");
        icon_box.append(&img);
    });

    let muted_state = Rc::new(Cell::new(is_muted()));

    // 3. Mute button on slider overlay
    let mute_switch_c1 = widgets.mute_switch.clone();
    let muted_state_c1 = muted_state.clone();
    let update_icon_c1 = update_icon.clone();
    widgets.mute_btn.connect_clicked(move |_| {
        let new_mute = !muted_state_c1.get();
        muted_state_c1.set(new_mute);
        set_muted(new_mute);
        update_icon_c1(new_mute);
        mute_switch_c1.set_active(new_mute);
    });

    // 4. Volume slider with debounce
    let muted_state_c2 = muted_state.clone();
    let update_icon_c2 = update_icon.clone();
    let mute_switch_c2 = widgets.mute_switch.clone();
    widgets.slider.connect_debounced(80, move |val| {
        set_volume(val);
        if val > 0.0 {
            muted_state_c2.set(false);
            update_icon_c2(false);
            mute_switch_c2.set_active(false);
        }
    });

    // 5. Mute switch
    let muted_state_c3 = muted_state;
    let update_icon_c3 = update_icon;
    widgets.mute_switch.connect_state_set(move |active| {
        muted_state_c3.set(active);
        set_muted(active);
        update_icon_c3(active);
    });
}
