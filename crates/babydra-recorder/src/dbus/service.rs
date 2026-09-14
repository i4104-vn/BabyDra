//! DBus IPC server hosting `org.babydra.Recorder` on the session bus.

use babydra_core::models::recording::{RecordingConfig, RecordingMode};
use babydra_core::services::recording::{
    get_active_config, get_elapsed_secs, is_paused, is_recording, pause_recording,
    resume_recording, select_geometry_str_with_slurp, start_recording, stop_recording,
    toggle_pause, toggle_recording,
};
use tokio::sync::mpsc::UnboundedSender;
use zbus::interface;

#[derive(Debug, Clone)]
pub enum RecorderCommand {
    ShowUI,
    Start,
    Stop,
    Toggle,
    Pause,
    Resume,
    TogglePause,
    RecordOutput(String),
    RecordArea,
    RecordWindow(String),
}

pub struct RecorderDbusService {
    pub tx: UnboundedSender<RecorderCommand>,
}

#[interface(name = "org.babydra.Recorder")]
impl RecorderDbusService {
    /// Starts recording with default configuration.
    async fn start_recording(&self) -> bool {
        let _ = self.tx.send(RecorderCommand::Start);
        start_recording(&RecordingConfig::default()).is_ok()
    }

    /// Stops current recording and returns the path to the saved file.
    async fn stop_recording(&self) -> String {
        let _ = self.tx.send(RecorderCommand::Stop);
        match stop_recording() {
            Ok(Some(path)) => path.to_string_lossy().to_string(),
            _ => String::new(),
        }
    }

    /// Toggles recording state on or off.
    async fn toggle_recording(&self) -> bool {
        let _ = self.tx.send(RecorderCommand::Toggle);
        toggle_recording(None).unwrap_or(false)
    }

    /// Pauses active recording.
    async fn pause_recording(&self) -> bool {
        let _ = self.tx.send(RecorderCommand::Pause);
        pause_recording().unwrap_or(false)
    }

    /// Resumes paused recording.
    async fn resume_recording(&self) -> bool {
        let _ = self.tx.send(RecorderCommand::Resume);
        resume_recording().unwrap_or(false)
    }

    /// Toggles between pause and resume.
    async fn toggle_pause(&self) -> bool {
        let _ = self.tx.send(RecorderCommand::TogglePause);
        toggle_pause().unwrap_or(false)
    }

    /// Returns whether the recording is paused.
    async fn is_paused(&self) -> bool {
        is_paused()
    }

    /// Records a specific monitor output.
    async fn record_output(&self, output_name: &str) -> bool {
        let _ = self.tx.send(RecorderCommand::RecordOutput(output_name.to_string()));
        let config = RecordingConfig {
            mode: RecordingMode::SingleOutput(output_name.to_string()),
            ..Default::default()
        };
        start_recording(&config).is_ok()
    }

    /// Prompts the user to select an area via slurp, then records it.
    async fn record_area(&self) -> bool {
        let _ = self.tx.send(RecorderCommand::RecordArea);
        if let Some(geom) = select_geometry_str_with_slurp() {
            let config = RecordingConfig {
                mode: RecordingMode::Window(geom),
                ..Default::default()
            };
            start_recording(&config).is_ok()
        } else {
            false
        }
    }

    /// Records a window or specific geometry.
    async fn record_window(&self, geometry: &str) -> bool {
        let _ = self.tx.send(RecorderCommand::RecordWindow(geometry.to_string()));
        let config = RecordingConfig {
            mode: RecordingMode::Window(geometry.to_string()),
            ..Default::default()
        };
        start_recording(&config).is_ok()
    }

    /// Returns current state: "recording" or "idle".
    async fn get_status(&self) -> String {
        if is_recording() {
            "recording".to_string()
        } else {
            "idle".to_string()
        }
    }

    /// Returns elapsed seconds of active recording.
    async fn get_elapsed(&self) -> u64 {
        get_elapsed_secs()
    }

    /// Returns the active recording mode name.
    async fn get_mode(&self) -> String {
        if let Some(cfg) = get_active_config() {
            match cfg.mode {
                RecordingMode::Fullscreen => "Fullscreen".to_string(),
                RecordingMode::SingleOutput(ref name) => format!("Display ({name})"),
                RecordingMode::Area { .. } => "Area".to_string(),
                RecordingMode::Window(_) => "Window / Region".to_string(),
            }
        } else {
            "Idle".to_string()
        }
    }

    /// Returns the active format.
    async fn get_format(&self) -> String {
        get_active_config()
            .map(|c| c.format.to_uppercase())
            .unwrap_or_else(|| "MP4".to_string())
    }

    /// Returns the active framerate.
    async fn get_framerate(&self) -> u32 {
        get_active_config().map(|c| c.framerate).unwrap_or(60)
    }

    /// Brings up the recorder control window.
    async fn show_ui(&self) {
        let _ = self.tx.send(RecorderCommand::ShowUI);
    }
}
