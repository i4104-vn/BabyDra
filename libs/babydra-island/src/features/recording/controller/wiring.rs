//! UI event binding and signal wiring for recording popover.

use babydra_core::i18n::trans;
use babydra_core::models::recording::RecordingMode;
use gtk4::prelude::*;

use super::actions::{
    open_recordings_dir_action, select_recording_area_action, start_recording_action,
    stop_recording_action, toggle_audio_mute_action, toggle_mic_mute_action, toggle_pause_action,
};
use crate::features::recording::models::PopoverActionsContext;

/// Connects all UI events, dropdowns, and button actions to the recording controller.
pub fn connect_popover_actions(ctx: PopoverActionsContext) {
    // Mode selection
    {
        let config = ctx.config.clone();
        let display_output_row = ctx.display_output_row.clone();
        let area_row = ctx.area_row.clone();
        let output_dropdown = ctx.output_dropdown.clone();
        let output_names = ctx.output_names.clone();
        let popover = ctx.popover.clone();
        let area_label = ctx.area_label.clone();
        ctx.mode_combo.connect_selected_notify(move |dropdown| {
            let mut cfg = config.borrow_mut();
            match dropdown.selected() {
                1 => {
                    let output = output_names
                        .get(output_dropdown.selected() as usize)
                        .cloned()
                        .unwrap_or_default();
                    cfg.mode = RecordingMode::SingleOutput(output);
                    display_output_row.set_visible(true);
                    area_row.set_visible(false);
                }
                2 => {
                    cfg.mode = RecordingMode::Window(String::new());
                    display_output_row.set_visible(false);
                    area_row.set_visible(true);
                    drop(cfg);
                    select_recording_area_action(&popover, &area_label, &config);
                }
                _ => {
                    cfg.mode = RecordingMode::Fullscreen;
                    display_output_row.set_visible(false);
                    area_row.set_visible(false);
                }
            }
        });
    }

    // Display output selection
    {
        let config = ctx.config.clone();
        let output_names = ctx.output_names.clone();
        ctx.output_dropdown
            .connect_selected_notify(move |dropdown| {
                if let Some(name) = output_names.get(dropdown.selected() as usize) {
                    config.borrow_mut().mode = RecordingMode::SingleOutput(name.clone());
                }
            });
    }

    // Area selection button
    {
        let config = ctx.config.clone();
        let area_label = ctx.area_label.clone();
        let popover = ctx.popover.clone();
        ctx.area_button.connect_clicked(move |_| {
            select_recording_area_action(&popover, &area_label, &config);
        });
    }

    // Resolution selection
    {
        let config = ctx.config.clone();
        ctx.resolution.connect_selected_notify(move |dropdown| {
            config.borrow_mut().resolution = match dropdown.selected() {
                1 => Some((1920, 1080)),
                2 => Some((1280, 720)),
                3 => Some((854, 480)),
                _ => None,
            };
        });
    }

    // Framerate selection
    {
        let config = ctx.config.clone();
        ctx.framerate.connect_selected_notify(move |dropdown| {
            config.borrow_mut().framerate = match dropdown.selected() {
                0 => 90,
                2 => 30,
                3 => 24,
                _ => 60,
            };
        });
    }

    // Format selection
    {
        let config = ctx.config.clone();
        ctx.format.connect_selected_notify(move |dropdown| {
            config.borrow_mut().format = match dropdown.selected() {
                1 => "mkv",
                2 => "webm",
                _ => "mp4",
            }
            .to_string();
        });
    }

    // HDR switch
    {
        let config = ctx.config.clone();
        ctx.hdr.connect_active_notify(move |switch| {
            config.borrow_mut().hdr = switch.is_active();
        });
    }

    // Audio switch
    {
        let config = ctx.config.clone();
        let audio_device_names = ctx.audio_device_names.clone();
        let audio_device = ctx.audio_device.clone();
        let audio_device_row = ctx.audio_device_row.clone();
        ctx.audio.connect_active_notify(move |switch| {
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

    // Audio device dropdown
    {
        let config = ctx.config.clone();
        let audio_device_names = ctx.audio_device_names.clone();
        ctx.audio_device.connect_selected_notify(move |dropdown| {
            config.borrow_mut().audio_device = audio_device_names
                .get(dropdown.selected() as usize)
                .cloned();
        });
    }

    // Codec dropdown
    {
        let config = ctx.config.clone();
        ctx.codec.connect_selected_notify(move |dropdown| {
            config.borrow_mut().codec = match dropdown.selected() {
                1 => Some("libx264".to_string()),
                2 => Some("h264_vaapi".to_string()),
                _ => None,
            };
        });
    }

    // Start recording button
    {
        let config = ctx.config.clone();
        ctx.start_button.connect_clicked(move |_| {
            start_recording_action(&config.borrow());
        });
    }

    // Open recordings folder button
    ctx.open_folder.connect_clicked(|_| {
        open_recordings_dir_action();
    });

    // Pause recording button
    ctx.btn_pause.connect_clicked(|_| {
        toggle_pause_action();
    });

    // Stop recording button
    {
        let timer_label = ctx.timer_label.clone();
        let settings = ctx.settings.clone();
        let action_row = ctx.action_row.clone();
        let live_row = ctx.live_row.clone();
        let status_badge = ctx.status_badge.clone();
        ctx.btn_stop.connect_clicked(move |_| {
            stop_recording_action();
            timer_label.set_text("00:00:00");
            settings.set_visible(true);
            action_row.set_visible(true);
            live_row.set_visible(false);

            status_badge.remove_css_class("badge-recording");
            status_badge.remove_css_class("badge-paused");
            status_badge.add_css_class("badge-ready");
            status_badge.set_text(&trans("recorder.status_idle"));
        });
    }

    // Mute system audio button
    {
        let button = ctx.btn_mute_audio.clone();
        ctx.btn_mute_audio.connect_clicked(move |_| {
            toggle_audio_mute_action(&button);
        });
    }

    // Mute microphone button
    {
        let button = ctx.btn_mute_mic.clone();
        ctx.btn_mute_mic.connect_clicked(move |_| {
            toggle_mic_mute_action(&button);
        });
    }
}
