//! Recording configuration and session control popover.

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use babydra_core::i18n::trans;
use babydra_core::models::recording::RecordingConfig;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label};

use crate::features::recording::controller::{connect_popover_actions, PopoverActionsContext};
use crate::features::recording::service::IslandRecordingState;
use crate::features::recording::ui::button::RecordingButtonWidget;
use crate::island::ui::IslandPopover;

pub mod header;
pub mod live;
pub mod row;
pub mod settings;

pub use header::create_popover_header;
pub use live::create_live_card;
pub use row::setting_row;
pub use settings::create_settings_card;

#[derive(Clone)]
pub struct RecordingPopover {
    pub base: IslandPopover,
    pub timer_label: Label,
    pub status_badge: Label,
    pub mode_label: Label,
    pub quality_label: Label,
    pub btn_pause: RecordingButtonWidget,
    pub btn_stop: RecordingButtonWidget,
    pub btn_mute_audio: RecordingButtonWidget,
    pub btn_mute_mic: RecordingButtonWidget,
    settings: GtkBox,
    action_row: GtkBox,
    live_row: GtkBox,
}

impl Deref for RecordingPopover {
    type Target = IslandPopover;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl RecordingPopover {
    pub fn new(capsule: &GtkBox) -> Self {
        let base = IslandPopover::new_modal(
            capsule,
            "recording-popover control-popover",
            "recording-popover-box",
            360,
        );
        let config = Rc::new(RefCell::new(RecordingConfig {
            framerate: 90,
            ..Default::default()
        }));

        let (header, status_badge) = create_popover_header();
        base.popover_box.append(&header);

        let settings = create_settings_card();
        base.popover_box.append(&settings.card);
        base.popover_box.append(&settings.action_row);

        let live = create_live_card();
        base.popover_box.append(&live.card);

        // Connect all interactions and logic through the recording controller
        connect_popover_actions(PopoverActionsContext {
            popover: base.clone(),
            config,
            mode_combo: settings.mode_dropdown,
            output_dropdown: settings.output_dropdown,
            display_output_row: settings.display_output_row,
            output_names: settings.output_names,
            area_row: settings.area_row,
            area_label: settings.area_label,
            area_button: settings.area_button,
            resolution: settings.resolution,
            framerate: settings.framerate,
            format: settings.format,
            hdr: settings.hdr,
            audio: settings.audio,
            audio_device_row: settings.audio_device_row,
            audio_device: settings.audio_device,
            audio_device_names: settings.audio_device_names,
            codec: settings.codec,
            start_button: settings.start_button,
            open_folder: settings.open_folder,
            btn_pause: live.btn_pause.clone(),
            btn_stop: live.btn_stop.clone(),
            btn_mute_audio: live.btn_mute_audio.clone(),
            btn_mute_mic: live.btn_mute_mic.clone(),
            timer_label: live.timer_label.clone(),
            settings: settings.card.clone(),
            action_row: settings.action_row.clone(),
            live_row: live.card.clone(),
            status_badge: status_badge.clone(),
        });

        Self {
            base,
            timer_label: live.timer_label,
            status_badge,
            mode_label: live.mode_label,
            quality_label: live.quality_label,
            btn_pause: live.btn_pause,
            btn_stop: live.btn_stop,
            btn_mute_audio: live.btn_mute_audio,
            btn_mute_mic: live.btn_mute_mic,
            settings: settings.card,
            action_row: settings.action_row,
            live_row: live.card,
        }
    }

    pub fn reset_to_config(&self) {
        self.timer_label.set_text("00:00:00");
        self.settings.set_visible(true);
        self.action_row.set_visible(true);
        self.live_row.set_visible(false);

        self.status_badge.remove_css_class("badge-recording");
        self.status_badge.remove_css_class("badge-paused");
        self.status_badge.add_css_class("badge-ready");
        self.status_badge.set_text(&trans("recorder.status_idle"));
    }

    pub fn update_data(&self, state: &IslandRecordingState) {
        if !state.is_recording {
            self.reset_to_config();
            return;
        }

        let elapsed = state.elapsed_secs;
        self.timer_label.set_text(&format!(
            "{:02}:{:02}:{:02}",
            elapsed / 3600,
            (elapsed % 3600) / 60,
            elapsed % 60
        ));

        self.settings.set_visible(false);
        self.action_row.set_visible(false);
        self.live_row.set_visible(true);

        self.status_badge.remove_css_class("badge-ready");
        self.status_badge.remove_css_class("badge-recording");
        self.status_badge.remove_css_class("badge-paused");
        if state.is_paused {
            self.status_badge.add_css_class("badge-paused");
            self.status_badge.set_text(&trans("recorder.status_paused"));
        } else {
            self.status_badge.add_css_class("badge-recording");
            self.status_badge
                .set_text(&trans("recorder.status_recording"));
        }

        if state.is_paused {
            self.btn_pause
                .set_icon_and_title("media-playback-start", &trans("recorder.resume"));
        } else {
            self.btn_pause
                .set_icon_and_title("media-playback-pause", &trans("recorder.pause"));
        }
        if !state.mode_name.is_empty() {
            self.mode_label.set_text(&state.mode_name);
        }
        let hdr_tag = if state.hdr { " • HDR10" } else { "" };
        self.quality_label
            .set_text(&format!("{} FPS • {}{}", state.framerate, state.format, hdr_tag));

        let audio_muted = state.is_audio_muted;
        self.btn_mute_audio
            .button
            .set_sensitive(state.audio_enabled);
        self.btn_mute_audio.set_icon_and_title(
            if audio_muted {
                "audio-volume-muted"
            } else {
                "audio-volume-high"
            },
            &trans(if audio_muted {
                "recorder.audio_unmute"
            } else {
                "recorder.audio_mute"
            }),
        );
        self.btn_mute_audio.set_alert(audio_muted && state.audio_enabled);

        let mic_muted = state.is_mic_muted;
        self.btn_mute_mic
            .button
            .set_sensitive(state.audio_enabled);
        self.btn_mute_mic.set_icon_and_title(
            if mic_muted {
                "microphone-disabled"
            } else {
                "audio-input-microphone"
            },
            &trans(if mic_muted {
                "recorder.mic_unmute"
            } else {
                "recorder.mic_mute"
            }),
        );
        self.btn_mute_mic.set_alert(mic_muted && state.audio_enabled);
    }
}
