//! Integration tests for screen recording service and configuration.

use babydra_core::services::recording::{
    get_new_recording_path, get_recordings_dir, start_recording, stop_recording,
    RecordingConfig, RecordingMode, RecordingStatus,
};

#[test]
fn test_default_recording_config() {
    let config = RecordingConfig::default();
    assert_eq!(config.mode, RecordingMode::Fullscreen);
    assert_eq!(config.framerate, 60);
    assert!(!config.audio);
    assert_eq!(config.format, "mp4");
    assert!(config.resolution.is_none());
    assert!(config.audio_device.is_none());
    assert!(config.codec.is_none());
}

#[test]
fn test_recording_config_serde_roundtrip() {
    let original = RecordingConfig {
        mode: RecordingMode::Area {
            x: 100,
            y: 200,
            width: 1280,
            height: 720,
        },
        resolution: Some((1280, 720)),
        framerate: 30,
        audio: true,
        audio_device: Some("default_sink.monitor".to_string()),
        format: "mkv".to_string(),
        codec: Some("libx264".to_string()),
    };

    let serialized = serde_json::to_string(&original).expect("Serialization failed");
    let deserialized: RecordingConfig =
        serde_json::from_str(&serialized).expect("Deserialization failed");

    assert_eq!(original, deserialized);
}

#[test]
fn test_recording_modes() {
    let full = RecordingMode::Fullscreen;
    let out = RecordingMode::SingleOutput("HDMI-A-1".to_string());
    let area = RecordingMode::Area {
        x: 0,
        y: 0,
        width: 1920,
        height: 1080,
    };
    let win = RecordingMode::Window("100,100 800x600".to_string());

    assert_ne!(full, out);
    assert_ne!(area, win);
}

#[test]
fn test_recordings_path_resolution() {
    let dir = get_recordings_dir();
    assert!(dir.to_string_lossy().contains("Recordings"));

    let path_mp4 = get_new_recording_path("mp4");
    assert!(path_mp4.starts_with(&dir));
    assert!(path_mp4.to_string_lossy().ends_with(".mp4"));

    let path_mkv = get_new_recording_path("mkv");
    assert!(path_mkv.to_string_lossy().ends_with(".mkv"));
}

#[test]
fn test_recording_status_logic() {
    let idle = RecordingStatus::Idle;
    assert!(!idle.is_recording());
    assert_eq!(idle.elapsed_secs(), 0);

    let active = RecordingStatus::Recording {
        pid: 12345,
        output_path: std::path::PathBuf::from("/tmp/test.mp4"),
        elapsed_secs: 42,
        config: RecordingConfig::default(),
        is_paused: false,
    };
    assert!(active.is_recording());
    assert!(!active.is_paused());
    assert_eq!(active.elapsed_secs(), 42);
}

#[test]
fn test_live_recording_lifecycle_and_output() {
    let config = RecordingConfig {
        mode: RecordingMode::Area {
            x: 0,
            y: 0,
            width: 320,
            height: 240,
        },
        resolution: None,
        framerate: 24,
        audio: false,
        audio_device: None,
        format: "mp4".to_string(),
        codec: None,
    };

    if let Ok(output_path) = start_recording(&config) {
        std::thread::sleep(std::time::Duration::from_millis(1500));
        let res = stop_recording();
        assert!(res.is_ok());
        if output_path.exists() {
            let _ = std::fs::remove_file(&output_path);
        }
    }
}
