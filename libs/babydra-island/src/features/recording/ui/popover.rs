//! Recording configuration and controls rendered inside the island popover.

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use babydra_core::i18n::trans;
use babydra_core::models::recording::{RecordingConfig, RecordingMode};
use babydra_core::services::recording::{
    get_recordings_dir, select_geometry_str_with_slurp, start_recording, stop_recording,
    toggle_pause,
};
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, DropDown, Label, Orientation, Switch};

use super::button::RecordingButtonWidget;
use crate::features::recording::service::{
    toggle_audio_mute, toggle_mic_mute, IslandRecordingState,
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
    settings: GtkBox,
    action_row: GtkBox,
    meta_card: GtkBox,
    buttons_box: GtkBox,
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
        let config = Rc::new(RefCell::new(RecordingConfig::default()));

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

        let timer_box = GtkBox::new(Orientation::Vertical, 2);
        timer_box.add_css_class("recording-timer-container");
        let timer_label = Label::new(Some("00:00:00"));
        timer_label.add_css_class("recording-hero-timer");
        timer_label.set_halign(Align::Center);
        timer_box.append(&timer_label);
        base.popover_box.append(&timer_box);

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
        let display_output_row = setting_row("display", &trans("recorder.display_output"), &output_dropdown);
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
        let framerate = DropDown::from_strings(&["60 FPS", "30 FPS", "24 FPS"]);
        let format = DropDown::from_strings(&["MP4 (H.264)", "MKV (H.264)", "WebM (VP9)"]);
        let codec =
            DropDown::from_strings(&["Default", "H.264 (libx264)", "H.264 VA-API (h264_vaapi)"]);
        let audio = Switch::new();
        let audio_devices = babydra_core::services::system::volume::get_audio_devices(true);
        let audio_device_names: Vec<String> = audio_devices
            .iter()
            .map(|device| device.name.clone())
            .collect();
        let audio_device_refs: Vec<&str> = audio_device_names.iter().map(String::as_str).collect();
        let audio_device = DropDown::from_strings(&audio_device_refs);
        audio_device.set_sensitive(false);

        let audio_device_row = setting_row("volume", &trans("recorder.audio_device"), &audio_device);
        audio_device_row.set_visible(false);

        settings.append(&setting_row("display", &trans("recorder.resolution"), &resolution));
        settings.append(&setting_row("activity", &trans("recorder.framerate"), &framerate));
        settings.append(&setting_row("sliders", &trans("recorder.format"), &format));
        settings.append(&setting_row("cog", &trans("recorder.codec"), &codec));
        settings.append(&setting_row("microphone", &trans("recorder.audio_toggle"), &audio));
        settings.append(&audio_device_row);
        base.popover_box.append(&settings);

        {
            let config = config.clone();
            let display_output_row = display_output_row.clone();
            let area_row = area_row.clone();
            let output_dropdown = output_dropdown.clone();
            let output_names = output_names.clone();
            mode_dropdown.connect_selected_notify(move |dropdown| {
                let mut config = config.borrow_mut();
                match dropdown.selected() {
                    1 => {
                        let output = output_names
                            .get(output_dropdown.selected() as usize)
                            .cloned()
                            .unwrap_or_default();
                        config.mode = RecordingMode::SingleOutput(output);
                        display_output_row.set_visible(true);
                        area_row.set_visible(false);
                    }
                    2 => {
                        config.mode = RecordingMode::Window(String::new());
                        display_output_row.set_visible(false);
                        area_row.set_visible(true);
                    }
                    _ => {
                        config.mode = RecordingMode::Fullscreen;
                        display_output_row.set_visible(false);
                        area_row.set_visible(false);
                    }
                }
            });
        }
        {
            let config = config.clone();
            let output_names = output_names.clone();
            output_dropdown.connect_selected_notify(move |dropdown| {
                if let Some(name) = output_names.get(dropdown.selected() as usize) {
                    config.borrow_mut().mode = RecordingMode::SingleOutput(name.clone());
                }
            });
        }
        {
            let config = config.clone();
            let area_label = area_label.clone();
            area_button.connect_clicked(move |_| {
                if let Some(geometry) = select_geometry_str_with_slurp() {
                    area_label.set_text(&geometry);
                    config.borrow_mut().mode = RecordingMode::Window(geometry);
                }
            });
        }
        {
            let config = config.clone();
            resolution.connect_selected_notify(move |dropdown| {
                config.borrow_mut().resolution = match dropdown.selected() {
                    1 => Some((1920, 1080)),
                    2 => Some((1280, 720)),
                    3 => Some((854, 480)),
                    _ => None,
                };
            });
        }
        {
            let config = config.clone();
            framerate.connect_selected_notify(move |dropdown| {
                config.borrow_mut().framerate = match dropdown.selected() {
                    1 => 30,
                    2 => 24,
                    _ => 60,
                };
            });
        }
        {
            let config = config.clone();
            format.connect_selected_notify(move |dropdown| {
                config.borrow_mut().format = match dropdown.selected() {
                    1 => "mkv",
                    2 => "webm",
                    _ => "mp4",
                }
                .to_string();
            });
        }
        {
            let config = config.clone();
            let audio_device_names = audio_device_names.clone();
            let audio_device = audio_device.clone();
            let audio_device_row = audio_device_row.clone();
            audio.connect_active_notify(move |switch| {
                let active = switch.is_active();
                audio_device_row.set_visible(active);
                audio_device.set_sensitive(active);
                let mut config = config.borrow_mut();
                config.audio = active;
                config.audio_device = if active {
                    audio_device_names.first().cloned()
                } else {
                    None
                };
            });
        }
        {
            let config = config.clone();
            let audio_device_names = audio_device_names.clone();
            audio_device.connect_selected_notify(move |dropdown| {
                config.borrow_mut().audio_device = audio_device_names
                    .get(dropdown.selected() as usize)
                    .cloned();
            });
        }
        {
            let config = config.clone();
            codec.connect_selected_notify(move |dropdown| {
                config.borrow_mut().codec = match dropdown.selected() {
                    1 => Some("libx264".to_string()),
                    2 => Some("h264_vaapi".to_string()),
                    _ => None,
                };
            });
        }

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
        {
            let config = config.clone();
            start_button.connect_clicked(move |_| {
                let _ = start_recording(&config.borrow());
            });
        }
        open_folder.connect_clicked(|_| {
            let _ = std::process::Command::new("xdg-open")
                .arg(get_recordings_dir())
                .spawn();
        });

        let meta_card = GtkBox::new(Orientation::Horizontal, 12);
        meta_card.add_css_class("recording-meta-card");
        meta_card.set_homogeneous(true);
        let mode_label = Label::new(Some(&trans("recorder.mode_fullscreen")));
        mode_label.add_css_class("recording-meta-chip");
        let quality_label = Label::new(Some("60 FPS • MP4"));
        quality_label.add_css_class("recording-meta-chip");
        meta_card.append(&mode_label);
        meta_card.append(&quality_label);
        meta_card.set_visible(false);
        base.popover_box.append(&meta_card);

        let buttons_box = GtkBox::new(Orientation::Horizontal, 8);
        buttons_box.set_homogeneous(true);
        buttons_box.set_visible(false);
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
        let btn_mute_audio = RecordingButtonWidget::new(
            "audio-volume-high",
            &trans("recorder.audio_mute"),
            "recording-action-audio",
        );
        let btn_mute_mic = RecordingButtonWidget::new(
            "audio-input-microphone",
            &trans("recorder.mic_mute"),
            "recording-action-mic",
        );
        for button in [&btn_pause, &btn_stop, &btn_mute_audio, &btn_mute_mic] {
            buttons_box.append(&button.container);
        }
        base.popover_box.append(&buttons_box);

        btn_pause.click_gesture.connect_pressed(|_, _, _, _| {
            let _ = toggle_pause();
        });
        {
            let popover = base.popover.clone();
            btn_stop.click_gesture.connect_pressed(move |_, _, _, _| {
                let _ = stop_recording();
                popover.popdown();
            });
        }
        {
            let button = btn_mute_audio.clone();
            btn_mute_audio
                .click_gesture
                .connect_pressed(move |_, _, _, _| {
                    let muted = toggle_audio_mute();
                    button.set_icon_and_title(
                        if muted {
                            "audio-volume-muted"
                        } else {
                            "audio-volume-high"
                        },
                        &trans(if muted {
                            "recorder.audio_unmute"
                        } else {
                            "recorder.audio_mute"
                        }),
                    );
                    button.set_alert(muted);
                });
        }
        {
            let button = btn_mute_mic.clone();
            btn_mute_mic
                .click_gesture
                .connect_pressed(move |_, _, _, _| {
                    let muted = toggle_mic_mute();
                    button.set_icon_and_title(
                        if muted {
                            "microphone-disabled"
                        } else {
                            "audio-input-microphone"
                        },
                        &trans(if muted {
                            "recorder.mic_unmute"
                        } else {
                            "recorder.mic_mute"
                        }),
                    );
                    button.set_alert(muted);
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
            settings,
            action_row,
            meta_card,
            buttons_box,
        }
    }

    pub fn update_data(&self, state: &IslandRecordingState) {
        let elapsed = state.elapsed_secs;
        self.timer_label.set_text(&format!(
            "{:02}:{:02}:{:02}",
            elapsed / 3600,
            (elapsed % 3600) / 60,
            elapsed % 60
        ));
        let recording = state.is_recording;

        self.settings.set_visible(!recording);
        self.action_row.set_visible(!recording);
        self.meta_card.set_visible(recording);
        self.buttons_box.set_visible(recording);

        self.status_badge.remove_css_class("badge-ready");
        self.status_badge.remove_css_class("badge-recording");
        self.status_badge.remove_css_class("badge-paused");
        if !recording {
            self.status_badge.add_css_class("badge-ready");
            self.status_badge.set_text(&trans("recorder.status_idle"));
        } else if state.is_paused {
            self.status_badge.add_css_class("badge-paused");
            self.status_badge.set_text(&trans("recorder.status_paused"));
        } else {
            self.status_badge.add_css_class("badge-recording");
            self.status_badge.set_text(&trans("recorder.status_recording"));
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
        self.btn_mute_audio
            .set_alert(babydra_core::services::system::volume::is_muted());
        self.btn_mute_mic
            .set_alert(babydra_core::services::system::volume::is_microphone_muted());
    }
}
