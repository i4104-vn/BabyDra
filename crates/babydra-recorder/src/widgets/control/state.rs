//! Local UI state for the recorder control window.

use babydra_core::models::recording::RecordingMode;

#[derive(Clone)]
pub struct ControlWindowState {
    pub mode: RecordingMode,
    pub selected_output: String,
    pub area_geometry: Option<String>,
    pub resolution_idx: usize,
    pub framerate: u32,
    pub format: String,
    pub audio: bool,
}

impl Default for ControlWindowState {
    fn default() -> Self {
        Self {
            mode: RecordingMode::Fullscreen,
            selected_output: String::new(),
            area_geometry: None,
            resolution_idx: 0,
            framerate: 60,
            format: "mp4".to_string(),
            audio: false,
        }
    }
}
