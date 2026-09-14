//! Glassmorphic recording control popover anchored below the Dynamic Island.

use std::ops::Deref;

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Label, Orientation};

use super::button::RecordingButtonWidget;
use crate::features::recording::service::{
    is_audio_muted, is_mic_muted, stop_recording_via_dbus_or_signal, toggle_audio_mute,
    toggle_mic_mute, toggle_pause_via_dbus_or_signal, IslandRecordingState,
};
use crate::island::ui::IslandPopover;

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
            380,
        );

        // 1. Header (Title + Live status badge)
        let header = GtkBox::new(Orientation::Horizontal, 8);
        header.add_css_class("recording-popover-header");
        header.set_valign(Align::Center);

        let title_lbl = Label::new(Some(&trans("recorder.session_header")));
        title_lbl.add_css_class("recording-header-title");
        title_lbl.set_valign(Align::Center);
        header.append(&title_lbl);

        let status_badge = Label::new(Some(&trans("recorder.status_recording")));
        status_badge.add_css_class("recording-status-badge");
        status_badge.add_css_class("badge-recording");
        status_badge.set_halign(Align::End);
        status_badge.set_hexpand(true);
        status_badge.set_valign(Align::Center);
        header.append(&status_badge);

        base.popover_box.append(&header);

        // 2. Hero timer section
        let timer_box = GtkBox::new(Orientation::Vertical, 2);
        timer_box.add_css_class("recording-timer-container");
        timer_box.set_halign(Align::Center);

        let timer_label = Label::new(Some("00:00:00"));
        timer_label.add_css_class("recording-hero-timer");
        timer_label.set_halign(Align::Center);
        timer_box.append(&timer_label);

        base.popover_box.append(&timer_box);

        // 3. Metadata card (Mode & Quality details)
        let meta_card = GtkBox::new(Orientation::Horizontal, 12);
        meta_card.add_css_class("recording-meta-card");
        meta_card.set_homogeneous(true);

        let mode_vbox = GtkBox::new(Orientation::Vertical, 2);
        let mode_hdr = Label::new(Some(&trans("recorder.target")));
        mode_hdr.add_css_class("recording-meta-label");
        let mode_label = Label::new(Some(&trans("recorder.mode_fullscreen")));
        mode_label.add_css_class("recording-meta-value");
        mode_vbox.append(&mode_hdr);
        mode_vbox.append(&mode_label);
        meta_card.append(&mode_vbox);

        let quality_vbox = GtkBox::new(Orientation::Vertical, 2);
        let quality_hdr = Label::new(Some(&trans("recorder.quality")));
        quality_hdr.add_css_class("recording-meta-label");
        let quality_label = Label::new(Some("60 FPS • MP4"));
        quality_label.add_css_class("recording-meta-value");
        quality_vbox.append(&quality_hdr);
        quality_vbox.append(&quality_label);
        meta_card.append(&quality_vbox);

        base.popover_box.append(&meta_card);

        // 4. Action buttons: 4 cards in a row
        let buttons_box = GtkBox::new(Orientation::Horizontal, 8);
        buttons_box.add_css_class("recording-popover-buttons");
        buttons_box.set_halign(Align::Center);
        buttons_box.set_homogeneous(true);
        buttons_box.set_margin_top(10);

        let btn_pause = RecordingButtonWidget::new(
            "media-playback-pause",
            &trans("recorder.pause"),
            "recording-action-pause",
        );

        let btn_stop = RecordingButtonWidget::new(
            "media-playback-stop",
            &trans("recorder.stop"),
            "recording-action-stop",
        );

        let audio_title = if is_audio_muted() {
            trans("recorder.audio_unmute")
        } else {
            trans("recorder.audio_mute")
        };
        let btn_mute_audio = RecordingButtonWidget::new(
            if is_audio_muted() {
                "audio-volume-muted"
            } else {
                "audio-volume-high"
            },
            &audio_title,
            "recording-action-audio",
        );

        let mic_title = if is_mic_muted() {
            trans("recorder.mic_unmute")
        } else {
            trans("recorder.mic_mute")
        };
        let btn_mute_mic = RecordingButtonWidget::new(
            if is_mic_muted() {
                "microphone-disabled"
            } else {
                "audio-input-microphone"
            },
            &mic_title,
            "recording-action-mic",
        );

        buttons_box.append(&btn_pause.container);
        buttons_box.append(&btn_stop.container);
        buttons_box.append(&btn_mute_audio.container);
        buttons_box.append(&btn_mute_mic.container);

        base.popover_box.append(&buttons_box);

        // Connect click gestures
        {
            btn_pause.click_gesture.connect_pressed(move |_, _, _, _| {
                toggle_pause_via_dbus_or_signal();
            });
        }

        {
            let base_pop = base.popover.clone();
            btn_stop.click_gesture.connect_pressed(move |_, _, _, _| {
                stop_recording_via_dbus_or_signal();
                base_pop.popdown();
            });
        }

        {
            let btn_a = btn_mute_audio.clone();
            btn_mute_audio
                .click_gesture
                .connect_pressed(move |_, _, _, _| {
                    let now_muted = toggle_audio_mute();
                    if now_muted {
                        btn_a.set_icon_and_title(
                            "audio-volume-muted",
                            &trans("recorder.audio_unmute"),
                        );
                        btn_a.set_alert(true);
                    } else {
                        btn_a.set_icon_and_title(
                            "audio-volume-high",
                            &trans("recorder.audio_mute"),
                        );
                        btn_a.set_alert(false);
                    }
                });
        }

        {
            let btn_m = btn_mute_mic.clone();
            btn_mute_mic
                .click_gesture
                .connect_pressed(move |_, _, _, _| {
                    let now_muted = toggle_mic_mute();
                    if now_muted {
                        btn_m.set_icon_and_title(
                            "microphone-disabled",
                            &trans("recorder.mic_unmute"),
                        );
                        btn_m.set_alert(true);
                    } else {
                        btn_m.set_icon_and_title(
                            "audio-input-microphone",
                            &trans("recorder.mic_mute"),
                        );
                        btn_m.set_alert(false);
                    }
                });
        }

        Self {
            base,
            timer_label,
            status_badge,
            mode_label,
            quality_label,
            btn_pause,
            btn_stop,
            btn_mute_audio,
            btn_mute_mic,
        }
    }

    /// Synchronizes popover data and button states with the latest recording state.
    pub fn update_data(&self, state: &IslandRecordingState) {
        // 1. Timer
        let elapsed = state.elapsed_secs;
        let hours = elapsed / 3600;
        let minutes = (elapsed % 3600) / 60;
        let seconds = elapsed % 60;
        self.timer_label
            .set_text(&format!("{:02}:{:02}:{:02}", hours, minutes, seconds));

        // 2. Status badge & Pause button
        if state.is_paused {
            self.status_badge.set_text(&trans("recorder.status_paused"));
            self.status_badge.remove_css_class("badge-recording");
            self.status_badge.add_css_class("badge-paused");

            self.btn_pause
                .set_icon_and_title("media-playback-start", &trans("recorder.resume"));
            self.btn_pause.set_alert(true);
        } else {
            self.status_badge
                .set_text(&trans("recorder.status_recording"));
            self.status_badge.remove_css_class("badge-paused");
            self.status_badge.add_css_class("badge-recording");

            self.btn_pause
                .set_icon_and_title("media-playback-pause", &trans("recorder.pause"));
            self.btn_pause.set_alert(false);
        }

        // 3. Metadata
        if !state.mode_name.is_empty() {
            self.mode_label.set_text(&state.mode_name);
        }
        let quality_str = format!("{} FPS • {}", state.framerate, state.format);
        self.quality_label.set_text(&quality_str);

        // 4. Sync Audio and Mic mute states
        let audio_muted = is_audio_muted();
        if audio_muted {
            self.btn_mute_audio.set_icon_and_title(
                "audio-volume-muted",
                &trans("recorder.audio_unmute"),
            );
            self.btn_mute_audio.set_alert(true);
        } else {
            self.btn_mute_audio
                .set_icon_and_title("audio-volume-high", &trans("recorder.audio_mute"));
            self.btn_mute_audio.set_alert(false);
        }

        let mic_muted = is_mic_muted();
        if mic_muted {
            self.btn_mute_mic.set_icon_and_title(
                "microphone-disabled",
                &trans("recorder.mic_unmute"),
            );
            self.btn_mute_mic.set_alert(true);
        } else {
            self.btn_mute_mic.set_icon_and_title(
                "audio-input-microphone",
                &trans("recorder.mic_mute"),
            );
            self.btn_mute_mic.set_alert(false);
        }
    }
}
