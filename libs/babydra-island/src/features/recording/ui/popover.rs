use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use babydra_core::i18n::trans;
use babydra_core::models::recording::RecordingConfig;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, DropDown, Label, Orientation, Switch};

use super::button::RecordingButtonWidget;
use crate::features::recording::controller::{connect_popover_actions, PopoverActionsContext};
use crate::features::recording::service::IslandRecordingState;
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

fn setting_row(icon_name: &str, label: &str, widget: &impl IsA<gtk4::Widget>) -> GtkBox {
    let row = GtkBox::new(Orientation::Horizontal, 10);
    row.add_css_class("recording-setting-row");

    let left_box = GtkBox::new(Orientation::Horizontal, 8);
    left_box.set_hexpand(true);
    left_box.set_halign(Align::Start);
    left_box.set_valign(Align::Center);

    let icon = babydra_ui_kit::ui::icon::get_icon(icon_name, 14);
    icon.set_valign(Align::Center);
    icon.add_css_class("recording-setting-icon");
    left_box.append(&icon);

    let label = Label::new(Some(label));
    label.add_css_class("recording-setting-label");
    label.set_valign(Align::Center);
    left_box.append(&label);

    row.append(&left_box);
    widget.as_ref().set_valign(Align::Center);
    row.append(widget);
    row
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

        let header = GtkBox::new(Orientation::Horizontal, 8);
        header.add_css_class("recording-popover-header");

        let header_left = GtkBox::new(Orientation::Horizontal, 6);
        header_left.set_valign(Align::Center);
        let header_icon = babydra_ui_kit::ui::icon::get_icon("camera", 14);
        header_icon.set_valign(Align::Center);
        header_icon.add_css_class("recording-header-icon");
        header_left.append(&header_icon);
        let title = Label::new(Some(&trans("recorder.session_header")));
        title.add_css_class("recording-header-title");
        title.set_valign(Align::Center);
        header_left.append(&title);
        header.append(&header_left);

        let status_badge = Label::new(Some(&trans("recorder.status_idle")));
        status_badge.add_css_class("recording-status-badge");
        status_badge.add_css_class("badge-ready");
        status_badge.set_hexpand(true);
        status_badge.set_halign(Align::End);
        header.append(&status_badge);
        base.popover_box.append(&header);

        let settings = GtkBox::new(Orientation::Vertical, 4);
        settings.add_css_class("recording-settings-card");
        let mode_dropdown = DropDown::from_strings(&[
            &trans("recorder.mode_fullscreen"),
            &trans("recorder.mode_display"),
            &trans("recorder.mode_area"),
        ]);
        let outputs = babydra_core::services::system::display::get_displays();
        let output_names: Vec<String> =
            outputs.iter().map(|display| display.name.clone()).collect();
        let output_refs: Vec<&str> = output_names.iter().map(String::as_str).collect();
        let output_dropdown = DropDown::from_strings(&output_refs);

        let area_btn_box = GtkBox::new(Orientation::Horizontal, 6);
        area_btn_box.set_valign(Align::Center);
        let area_btn_icon = babydra_ui_kit::ui::icon::get_icon("rect", 13);
        area_btn_icon.set_valign(Align::Center);
        let area_btn_label = Label::new(Some(&trans("recorder.select_area_btn")));
        area_btn_label.set_valign(Align::Center);
        area_btn_box.append(&area_btn_icon);
        area_btn_box.append(&area_btn_label);

        let area_button = Button::new();
        area_button.set_child(Some(&area_btn_box));
        area_button.add_css_class("recording-area-btn");
        let area_label = Label::new(Some(&trans("recorder.no_area_selected")));
        area_label.add_css_class("recording-area-label");
        let area_box = GtkBox::new(Orientation::Horizontal, 8);
        area_box.set_valign(Align::Center);
        area_box.append(&area_button);
        area_box.append(&area_label);

        let mode_row = setting_row("th-large", &trans("recorder.mode_header"), &mode_dropdown);
        let display_output_row = setting_row(
            "display",
            &trans("recorder.display_output"),
            &output_dropdown,
        );
        display_output_row.set_visible(false);
        let area_row = setting_row("rect", &trans("recorder.mode_area"), &area_box);
        area_row.set_visible(false);

        settings.append(&mode_row);
        settings.append(&display_output_row);
        settings.append(&area_row);

        let resolution = DropDown::from_strings(&[
            &trans("recorder.resolution_native"),
            "1080p (1920x1080)",
            "720p (1280x720)",
            "480p (854x480)",
        ]);
        let framerate = DropDown::from_strings(&["90 FPS", "60 FPS", "30 FPS", "24 FPS"]);
        let format = DropDown::from_strings(&["MP4 (H.264)", "MKV (H.264)", "WebM (VP9)"]);
        let codec =
            DropDown::from_strings(&["Default", "H.264 (libx264)", "H.264 VA-API (h264_vaapi)"]);
        let audio = Switch::new();
        // `get_audio_devices(true)` returns profile/route identifiers for
        // the volume mixer.  They are not valid `wf-recorder -a` source
        // names.  Recording uses the PipeWire default source and links both
        // the microphone and the desktop monitor automatically.
        let audio_device_names = vec!["default".to_string()];
        let audio_device_refs: Vec<&str> = audio_device_names.iter().map(String::as_str).collect();
        let audio_device = DropDown::from_strings(&audio_device_refs);
        audio_device.set_sensitive(false);

        let audio_device_row =
            setting_row("volume", &trans("recorder.audio_device"), &audio_device);
        audio_device_row.set_visible(false);

        settings.append(&setting_row(
            "display",
            &trans("recorder.resolution"),
            &resolution,
        ));
        settings.append(&setting_row(
            "activity",
            &trans("recorder.framerate"),
            &framerate,
        ));
        settings.append(&setting_row("sliders", &trans("recorder.format"), &format));
        settings.append(&setting_row("cog", &trans("recorder.codec"), &codec));
        settings.append(&setting_row(
            "microphone",
            &trans("recorder.audio_toggle"),
            &audio,
        ));
        settings.append(&audio_device_row);
        base.popover_box.append(&settings);

        let action_row = GtkBox::new(Orientation::Horizontal, 8);
        action_row.add_css_class("recording-action-row");
        action_row.set_halign(Align::End);
        action_row.set_margin_top(8);

        let open_folder = babydra_ui_kit::components::create_icon_btn(
            "folder",
            &trans("recorder.open_folder"),
            "connect-pill-btn",
        );

        let start_button = babydra_ui_kit::components::create_icon_btn(
            "camera",
            &trans("recorder.start"),
            "suggested-action",
        );

        action_row.append(&open_folder);
        action_row.append(&start_button);
        base.popover_box.append(&action_row);

        // ── Live recording view: meta info (left) + vertical sidebar buttons (right) ──
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
        base.popover_box.append(&live_row);

        // Connect all interactions and logic through the recording controller
        connect_popover_actions(PopoverActionsContext {
            config,
            mode_combo: mode_dropdown,
            output_dropdown,
            display_output_row,
            output_names,
            area_row,
            area_label,
            area_button,
            resolution,
            framerate,
            format,
            audio,
            audio_device_row,
            audio_device,
            audio_device_names,
            codec,
            start_button,
            open_folder,
            btn_pause: btn_pause.clone(),
            btn_stop: btn_stop.clone(),
            btn_mute_audio: btn_mute_audio.clone(),
            btn_mute_mic: btn_mute_mic.clone(),
            timer_label: timer_label.clone(),
            settings: settings.clone(),
            action_row: action_row.clone(),
            live_row: live_row.clone(),
            status_badge: status_badge.clone(),
        });

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
            settings,
            action_row,
            live_row,
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
        self.quality_label
            .set_text(&format!("{} FPS • {}", state.framerate, state.format));

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
