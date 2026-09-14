//! Timer display and primary action buttons (Start/Stop, Open Folder).

use super::state::ControlWindowState;
use babydra_core::models::recording::{RecordingConfig, RecordingMode};
use babydra_core::services::recording::{
    get_elapsed_secs, get_recordings_dir, is_recording, start_recording, stop_recording,
};
use babydra_ui_kit::prelude::*;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Label, Orientation};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

/// Builds the actions section and sets up the live timer polling loop.
pub fn build_actions_section(state: Rc<RefCell<ControlWindowState>>) -> (GtkBox, Label, Button) {
    let container = GtkBox::new(Orientation::Vertical, 10);

    let timer_label = Label::new(Some("00:00:00"));
    timer_label.add_css_class("recorder-timer");
    container.append(&timer_label);

    let action_box = GtkBox::new(Orientation::Horizontal, 10);
    action_box.set_homogeneous(true);

    let record_btn = create_accent_button(&babydra_core::i18n::trans("recorder.start"));
    record_btn.add_css_class("recorder-action-btn");
    action_box.append(&record_btn);

    let open_folder_btn = create_button(&babydra_core::i18n::trans("recorder.open_folder"));
    open_folder_btn.connect_clicked(|_| {
        let dir = get_recordings_dir();
        let _ = std::process::Command::new("xdg-open").arg(dir).spawn();
    });
    action_box.append(&open_folder_btn);

    container.append(&action_box);

    // Connect record button
    {
        let state_c = state.clone();
        let record_btn_c = record_btn.clone();
        record_btn.connect_clicked(move |_| {
            if is_recording() {
                let _ = stop_recording();
                record_btn_c.set_label(&babydra_core::i18n::trans("recorder.start"));
                record_btn_c.remove_css_class("destructive-action");
                record_btn_c.add_css_class("suggested-action");
            } else {
                let st = state_c.borrow().clone();
                let resolution = match st.resolution_idx {
                    1 => Some((1920, 1080)),
                    2 => Some((1280, 720)),
                    3 => Some((854, 480)),
                    _ => None,
                };

                let mode = match &st.mode {
                    RecordingMode::SingleOutput(_) => {
                        RecordingMode::SingleOutput(st.selected_output.clone())
                    }
                    RecordingMode::Window(_) => {
                        let geom = st.area_geometry.clone().unwrap_or_default();
                        RecordingMode::Window(geom)
                    }
                    m => m.clone(),
                };

                let config = RecordingConfig {
                    mode,
                    resolution,
                    framerate: st.framerate,
                    audio: st.audio,
                    audio_device: None,
                    format: st.format.clone(),
                    codec: None,
                };

                if start_recording(&config).is_ok() {
                    record_btn_c.set_label(&babydra_core::i18n::trans("recorder.stop"));
                    record_btn_c.remove_css_class("suggested-action");
                    record_btn_c.add_css_class("destructive-action");
                }
            }
        });
    }

    // Timer polling loop
    {
        let timer_lbl_c = timer_label.clone();
        let record_btn_c = record_btn.clone();
        glib::timeout_add_local(Duration::from_millis(500), move || {
            if is_recording() {
                let elapsed = get_elapsed_secs();
                let hours = elapsed / 3600;
                let minutes = (elapsed % 3600) / 60;
                let seconds = elapsed % 60;
                timer_lbl_c.set_text(&format!("{:02}:{:02}:{:02}", hours, minutes, seconds));
                record_btn_c.set_label(&babydra_core::i18n::trans("recorder.stop"));
                record_btn_c.remove_css_class("suggested-action");
                record_btn_c.add_css_class("destructive-action");
            } else {
                timer_lbl_c.set_text("00:00:00");
                record_btn_c.set_label(&babydra_core::i18n::trans("recorder.start"));
                record_btn_c.remove_css_class("destructive-action");
                record_btn_c.add_css_class("suggested-action");
            }
            glib::ControlFlow::Continue
        });
    }

    (container, timer_label, record_btn)
}
