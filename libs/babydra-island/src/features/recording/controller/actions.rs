//! Action handlers and event wiring for the recording feature.

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::Label;

use babydra_core::i18n::trans;
use babydra_core::models::recording::{RecordingConfig, RecordingMode};
use babydra_core::services::recording::{
    get_recordings_dir, select_geometry_str_with_slurp, start_recording, stop_recording,
    toggle_pause,
};

use crate::features::recording::models::PopoverActionsContext;
use crate::features::recording::service::{toggle_audio_mute, toggle_mic_mute};
use crate::features::recording::ui::RecordingButtonWidget;

/// Dispatches starting screen recording.
pub fn start_recording_action(config: &RecordingConfig) {
    let _ = start_recording(config);
}

/// Dispatches stopping screen recording cleanly on a background worker thread.
pub fn stop_recording_action() {
    std::thread::spawn(|| {
        let _ = stop_recording();
    });
}

/// Dispatches pause / resume toggling.
pub fn toggle_pause_action() {
    let _ = toggle_pause();
}

/// Toggles audio mute state and updates the button visual.
pub fn toggle_audio_mute_action(button: &RecordingButtonWidget) {
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
}

/// Toggles microphone mute state and updates the button visual.
pub fn toggle_mic_mute_action(button: &RecordingButtonWidget) {
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
}

/// Opens the recordings directory in the system file manager.
pub fn open_recordings_dir_action() {
    let _ = std::process::Command::new("xdg-open")
        .arg(get_recordings_dir())
        .spawn();
}

/// Triggers slurp area selection and updates configuration.
pub fn select_recording_area_action(area_label: &Label, config: &Rc<RefCell<RecordingConfig>>) {
    if let Some(geometry) = select_geometry_str_with_slurp() {
        area_label.set_text(&geometry);
        config.borrow_mut().mode = RecordingMode::Window(geometry);
    }
}

/// Connects all UI events, dropdowns, and button actions to the recording controller.
pub fn connect_popover_actions(ctx: PopoverActionsContext) {
    // Mode selection
    {
        let config = ctx.config.clone();
        let display_output_row = ctx.display_output_row.clone();
        let area_row = ctx.area_row.clone();
        let output_dropdown = ctx.output_dropdown.clone();
        let output_names = ctx.output_names.clone();
        ctx.mode_combo.connect_selected_notify(move |dropdown| {
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

    // Display output selection
    {
        let config = ctx.config.clone();
        let output_names = ctx.output_names.clone();
        ctx.output_dropdown.connect_selected_notify(move |dropdown| {
            if let Some(name) = output_names.get(dropdown.selected() as usize) {
                config.borrow_mut().mode = RecordingMode::SingleOutput(name.clone());
            }
        });
    }

    // Area selection button
    {
        let config = ctx.config.clone();
        let area_label = ctx.area_label.clone();
        ctx.area_button.connect_clicked(move |_| {
            select_recording_area_action(&area_label, &config);
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
    ctx.btn_pause.click_gesture.connect_pressed(|_, _, _, _| {
        toggle_pause_action();
    });

    // Stop recording button
    {
        let timer_label = ctx.timer_label.clone();
        let settings = ctx.settings.clone();
        let action_row = ctx.action_row.clone();
        let meta_card = ctx.meta_card.clone();
        let buttons_box = ctx.buttons_box.clone();
        let status_badge = ctx.status_badge.clone();
        ctx.btn_stop.click_gesture.connect_pressed(move |_, _, _, _| {
            stop_recording_action();
            timer_label.set_text("00:00:00");
            settings.set_visible(true);
            action_row.set_visible(true);
            meta_card.set_visible(false);
            buttons_box.set_visible(false);

            status_badge.remove_css_class("badge-recording");
            status_badge.remove_css_class("badge-paused");
            status_badge.add_css_class("badge-ready");
            status_badge.set_text(&trans("recorder.status_idle"));
        });
    }

    // Mute system audio button
    {
        let button = ctx.btn_mute_audio.clone();
        ctx.btn_mute_audio
            .click_gesture
            .connect_pressed(move |_, _, _, _| {
                toggle_audio_mute_action(&button);
            });
    }

    // Mute microphone button
    {
        let button = ctx.btn_mute_mic.clone();
        ctx.btn_mute_mic
            .click_gesture
            .connect_pressed(move |_, _, _, _| {
                toggle_mic_mute_action(&button);
            });
    }
}
