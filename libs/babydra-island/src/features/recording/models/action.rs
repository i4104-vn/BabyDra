//! Action context models for recording popover event connections.

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::{Box as GtkBox, Button, DropDown, Label, Switch};

use babydra_core::models::recording::RecordingConfig;

use crate::features::recording::ui::RecordingButtonWidget;

/// Widgets and configuration state needed to wire recording popover interactions.
pub struct PopoverActionsContext {
    pub config: Rc<RefCell<RecordingConfig>>,
    pub mode_combo: DropDown,
    pub output_dropdown: DropDown,
    pub display_output_row: GtkBox,
    pub output_names: Vec<String>,
    pub area_row: GtkBox,
    pub area_label: Label,
    pub area_button: Button,
    pub resolution: DropDown,
    pub framerate: DropDown,
    pub format: DropDown,
    pub audio: Switch,
    pub audio_device_row: GtkBox,
    pub audio_device: DropDown,
    pub audio_device_names: Vec<String>,
    pub codec: DropDown,
    pub start_button: Button,
    pub open_folder: Button,
    pub btn_pause: RecordingButtonWidget,
    pub btn_stop: RecordingButtonWidget,
    pub btn_mute_audio: RecordingButtonWidget,
    pub btn_mute_mic: RecordingButtonWidget,
    pub timer_label: Label,
    pub settings: GtkBox,
    pub action_row: GtkBox,
    pub meta_card: GtkBox,
    pub buttons_box: GtkBox,
    pub status_badge: Label,
}
