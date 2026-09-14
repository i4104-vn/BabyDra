//! Command-line argument handling and DBus dispatching.

use babydra_core::models::recording::{RecordingConfig, RecordingMode};
use babydra_core::services::recording::{
    is_recording, select_geometry_str_with_slurp, start_recording, stop_recording,
    toggle_recording,
};

/// Attempts to activate an existing running DBus recorder instance to show its window.
/// Returns true if an existing daemon handled the request.
pub fn try_activate_running_instance() -> bool {
    if let Ok(conn) = zbus::blocking::Connection::session() {
        if let Ok(proxy) = zbus::blocking::Proxy::new(
            &conn,
            "org.babydra.Recorder",
            "/org/babydra/Recorder",
            "org.babydra.Recorder",
        ) {
            let res: Result<(), _> = proxy.call("ShowUi", &());
            if res.is_ok() {
                return true;
            }
        }
    }
    false
}

/// Checks if the recorder DBus daemon is currently running on the session bus.
pub fn is_daemon_already_running() -> bool {
    if let Ok(conn) = zbus::blocking::Connection::session() {
        if let Ok(proxy) = zbus::blocking::Proxy::new(
            &conn,
            "org.babydra.Recorder",
            "/org/babydra/Recorder",
            "org.babydra.Recorder",
        ) {
            let res: Result<String, _> = proxy.call("GetStatus", &());
            return res.is_ok();
        }
    }
    false
}

/// Handles incoming CLI flags and returns `true` if the application should terminate.
pub fn handle_cli_flag(arg: &str) -> bool {
    // Attempt via DBus first
    if let Ok(conn) = zbus::blocking::Connection::session() {
        if let Ok(proxy) = zbus::blocking::Proxy::new(
            &conn,
            "org.babydra.Recorder",
            "/org/babydra/Recorder",
            "org.babydra.Recorder",
        ) {
            match arg {
                "--start" => {
                    let res: Result<bool, _> = proxy.call("StartRecording", &());
                    if let Ok(started) = res {
                        println!("Recording {}", if started { "started" } else { "failed" });
                        return true;
                    }
                }
                "--stop" => {
                    let res: Result<String, _> = proxy.call("StopRecording", &());
                    if let Ok(path) = res {
                        println!("Recording stopped: {path}");
                        return true;
                    }
                }
                "--toggle" => {
                    let res: Result<bool, _> = proxy.call("ToggleRecording", &());
                    if let Ok(active) = res {
                        println!(
                            "Recording toggled: {}",
                            if active { "active" } else { "idle" }
                        );
                        return true;
                    }
                }
                "--pause" => {
                    let res: Result<bool, _> = proxy.call("PauseRecording", &());
                    if let Ok(paused) = res {
                        println!("Recording {}", if paused { "paused" } else { "pause failed" });
                        return true;
                    }
                }
                "--resume" => {
                    let res: Result<bool, _> = proxy.call("ResumeRecording", &());
                    if let Ok(resumed) = res {
                        println!("Recording {}", if resumed { "resumed" } else { "resume failed" });
                        return true;
                    }
                }
                "--status" => {
                    let res: Result<String, _> = proxy.call("GetStatus", &());
                    if let Ok(status) = res {
                        println!("{status}");
                        return true;
                    }
                }
                "--area" => {
                    let res: Result<bool, _> = proxy.call("RecordArea", &());
                    if let Ok(started) = res {
                        println!(
                            "Area recording {}",
                            if started { "started" } else { "cancelled" }
                        );
                        return true;
                    }
                }
                "--ui" => {
                    let res: Result<(), _> = proxy.call("ShowUi", &());
                    if res.is_ok() {
                        return true;
                    }
                }
                _ => {}
            }
        }
    }

    // Direct fallback if daemon is not running
    match arg {
        "--start" => {
            match start_recording(&RecordingConfig::default()) {
                Ok(p) => println!("Recording started: {}", p.display()),
                Err(e) => eprintln!("Error starting recording: {e}"),
            }
            true
        }
        "--stop" => {
            match stop_recording() {
                Ok(Some(p)) => println!("Recording saved: {}", p.display()),
                Ok(None) => println!("No active recording found"),
                Err(e) => eprintln!("Error stopping recording: {e}"),
            }
            true
        }
        "--toggle" => {
            match toggle_recording(None) {
                Ok(active) => println!(
                    "Recording toggled: {}",
                    if active { "active" } else { "idle" }
                ),
                Err(e) => eprintln!("Error toggling recording: {e}"),
            }
            true
        }
        "--pause" => {
            match babydra_core::services::recording::pause_recording() {
                Ok(true) => println!("Recording paused"),
                Ok(false) => println!("Recording not paused"),
                Err(e) => eprintln!("Error pausing recording: {e}"),
            }
            true
        }
        "--resume" => {
            match babydra_core::services::recording::resume_recording() {
                Ok(true) => println!("Recording resumed"),
                Ok(false) => println!("Recording not resumed"),
                Err(e) => eprintln!("Error resuming recording: {e}"),
            }
            true
        }
        "--status" => {
            if is_recording() {
                println!("recording");
            } else {
                println!("idle");
            }
            true
        }
        "--area" => {
            if let Some(geom) = select_geometry_str_with_slurp() {
                let config = RecordingConfig {
                    mode: RecordingMode::Window(geom),
                    ..Default::default()
                };
                match start_recording(&config) {
                    Ok(p) => println!("Area recording started: {}", p.display()),
                    Err(e) => eprintln!("Error: {e}"),
                }
            }
            true
        }
        "--help" | "-h" => {
            println!("BabyDra Screen Recorder");
            println!("Usage: babydra-recorder [OPTIONS]");
            println!();
            println!("Options:");
            println!("  --start       Start screen recording");
            println!("  --stop        Stop current recording");
            println!("  --toggle      Toggle recording state");
            println!("  --pause       Pause active recording");
            println!("  --resume      Resume paused recording");
            println!("  --area        Select an area to record");
            println!("  --status      Print recording status");
            println!("  --ui          Open the recorder control window");
            println!("  --daemon      Run recorder background daemon & tray");
            println!("  --help, -h    Show this help message");
            true
        }
        _ => false,
    }
}
