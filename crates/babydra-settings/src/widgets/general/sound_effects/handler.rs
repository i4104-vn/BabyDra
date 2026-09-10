//! System Sound Effects event wiring.

use super::render::SoundEffectsWidgets;
use babydra_core::services::system::default_apps::{
    play_test_alert_sound, set_event_sounds_enabled, set_input_feedback_sounds_enabled,
};
use gtk4::prelude::*;

/// Connects signals for Sound Effects widgets.
pub fn wire_events(widgets: &SoundEffectsWidgets) {
    widgets.event_sounds_switch.connect_state_set(move |active| {
        set_event_sounds_enabled(active);
    });

    widgets
        .feedback_sounds_switch
        .connect_state_set(move |active| {
            set_input_feedback_sounds_enabled(active);
        });

    widgets.test_sound_btn.connect_clicked(move |_| {
        play_test_alert_sound();
    });
}
