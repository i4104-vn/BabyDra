//! Live session view layout and active control buttons.

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Label, Orientation};

use crate::features::recording::ui::RecordingButtonWidget;

/// UI elements for displaying the active recording status and controls.
pub struct LiveCardWidgets {
    pub card: GtkBox,
    pub timer_label: Label,
    pub mode_label: Label,
    pub quality_label: Label,
    pub btn_pause: RecordingButtonWidget,
    pub btn_stop: RecordingButtonWidget,
    pub btn_mute_audio: RecordingButtonWidget,
    pub btn_mute_mic: RecordingButtonWidget,
}

/// Builds the live recording row widget containing duration timer and action controls.
pub fn create_live_card() -> LiveCardWidgets {
    let live_row = GtkBox::new(Orientation::Horizontal, 12);
    live_row.add_css_class("recording-live-row");
    live_row.set_visible(false);
    live_row.set_margin_top(4);

    // Left: timer + mode & quality info
    let left_box = GtkBox::new(Orientation::Vertical, 8);
    left_box.set_hexpand(true);
    left_box.set_valign(Align::Center);

    let timer_label = Label::new(Some("00:00:00"));
    timer_label.add_css_class("recording-hero-timer");
    timer_label.set_halign(Align::Center);
    left_box.append(&timer_label);

    let meta_card = GtkBox::new(Orientation::Horizontal, 12);
    meta_card.add_css_class("recording-meta-card");
    meta_card.set_homogeneous(true);
    meta_card.set_valign(Align::Center);
    let mode_label = Label::new(Some(&trans("recorder.mode_fullscreen")));
    mode_label.add_css_class("recording-meta-chip");
    let quality_label = Label::new(Some("60 FPS • MP4"));
    quality_label.add_css_class("recording-meta-chip");
    meta_card.append(&mode_label);
    meta_card.append(&quality_label);
    left_box.append(&meta_card);

    live_row.append(&left_box);

    // Right: 2x2 grid of circular buttons
    let buttons_box = GtkBox::new(Orientation::Vertical, 6);
    buttons_box.set_valign(Align::Center);
    buttons_box.set_halign(Align::End);
    let btn_pause = RecordingButtonWidget::new(
        "media-playback-pause",
        &trans("recorder.pause"),
        "",
    );
    let btn_stop = RecordingButtonWidget::new(
        "media-playback-stop",
        &trans("recorder.stop"),
        "delete-btn",
    );
    let btn_mute_audio = RecordingButtonWidget::new(
        "audio-volume-high",
        &trans("recorder.audio_mute"),
        "",
    );
    let btn_mute_mic = RecordingButtonWidget::new(
        "audio-input-microphone",
        &trans("recorder.mic_mute"),
        "",
    );

    let row1 = GtkBox::new(Orientation::Horizontal, 6);
    row1.append(&btn_pause.button);
    row1.append(&btn_stop.button);

    let row2 = GtkBox::new(Orientation::Horizontal, 6);
    row2.append(&btn_mute_audio.button);
    row2.append(&btn_mute_mic.button);

    buttons_box.append(&row1);
    buttons_box.append(&row2);

    live_row.append(&buttons_box);

    LiveCardWidgets {
        card: live_row,
        timer_label,
        mode_label,
        quality_label,
        btn_pause,
        btn_stop,
        btn_mute_audio,
        btn_mute_mic,
    }
}
