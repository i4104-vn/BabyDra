//! Settings card layout and controls construction for recording configuration.

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, DropDown, Label, Orientation, Switch};

use super::row::setting_row;

/// Collection of UI controls created for recording settings and primary action buttons.
pub struct SettingsCardWidgets {
    pub card: GtkBox,
    pub action_row: GtkBox,
    pub mode_dropdown: DropDown,
    pub output_dropdown: DropDown,
    pub display_output_row: GtkBox,
    pub output_names: Vec<String>,
    pub area_row: GtkBox,
    pub area_label: Label,
    pub area_button: Button,
    pub resolution: DropDown,
    pub framerate: DropDown,
    pub format: DropDown,
    pub hdr: Switch,
    pub audio: Switch,
    pub audio_device_row: GtkBox,
    pub audio_device: DropDown,
    pub audio_device_names: Vec<String>,
    pub codec: DropDown,
    pub start_button: Button,
    pub open_folder: Button,
}

/// Builds the recording settings card and bottom action button row.
pub fn create_settings_card() -> SettingsCardWidgets {
    let settings = GtkBox::new(Orientation::Vertical, 4);
    settings.add_css_class("recording-settings-card");

    let mode_dropdown = DropDown::from_strings(&[
        &trans("recorder.mode_fullscreen"),
        &trans("recorder.mode_display"),
        &trans("recorder.mode_area"),
    ]);

    let outputs = babydra_core::services::system::display::get_displays();
    let output_names: Vec<String> = outputs.iter().map(|display| display.name.clone()).collect();
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
    let framerate = DropDown::from_strings(&["90 FPS", "60 FPS", "30 FPS", "24 FPS"]);
    let format = DropDown::from_strings(&["MP4 (H.264)", "MKV (H.264)", "WebM (VP9)"]);
    let codec = DropDown::from_strings(&["Default", "H.264 (libx264)", "H.264 VA-API (h264_vaapi)"]);
    let hdr = Switch::new();
    let audio = Switch::new();

    let audio_device_names = vec!["default".to_string()];
    let audio_device_refs: Vec<&str> = audio_device_names.iter().map(String::as_str).collect();
    let audio_device = DropDown::from_strings(&audio_device_refs);
    audio_device.set_sensitive(false);

    let audio_device_row = setting_row("volume", &trans("recorder.audio_device"), &audio_device);
    audio_device_row.set_visible(false);

    settings.append(&setting_row("display", &trans("recorder.resolution"), &resolution));
    settings.append(&setting_row("activity", &trans("recorder.framerate"), &framerate));
    settings.append(&setting_row("sliders", &trans("recorder.format"), &format));
    settings.append(&setting_row("cog", &trans("recorder.codec"), &codec));
    settings.append(&setting_row("sun", &trans("recorder.hdr_toggle"), &hdr));
    settings.append(&setting_row("microphone", &trans("recorder.audio_toggle"), &audio));
    settings.append(&audio_device_row);

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

    SettingsCardWidgets {
        card: settings,
        action_row,
        mode_dropdown,
        output_dropdown,
        display_output_row,
        output_names,
        area_row,
        area_label,
        area_button,
        resolution,
        framerate,
        format,
        hdr,
        audio,
        audio_device_row,
        audio_device,
        audio_device_names,
        codec,
        start_button,
        open_folder,
    }
}
