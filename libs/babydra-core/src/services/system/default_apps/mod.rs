//! System default applications and sound effects configuration.

pub mod apps;
pub mod sound;

pub use apps::{
    get_available_browsers, get_available_file_managers, get_available_terminals,
    get_default_browser, get_default_file_manager, get_default_terminal, set_default_browser,
    set_default_file_manager, set_default_terminal,
};
pub use sound::{
    get_event_sounds_enabled, get_input_feedback_sounds_enabled, play_test_alert_sound,
    set_event_sounds_enabled, set_input_feedback_sounds_enabled,
};
pub use crate::models::desktop::app::AppChoice;
