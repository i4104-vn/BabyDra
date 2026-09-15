//! Isolated audio and microphone stream management for screen recording sessions.
//!
//! Controls PipeWire / PulseAudio links and streams specifically attached to `wf-recorder`
//! without touching master system volume or master microphone input.

use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::Duration;

static ACTIVE_PID: AtomicU32 = AtomicU32::new(0);
static AUDIO_ENABLED: AtomicBool = AtomicBool::new(false);
static IS_AUDIO_MUTED: AtomicBool = AtomicBool::new(false);
static IS_MIC_MUTED: AtomicBool = AtomicBool::new(false);

/// Retrieves PipeWire input ports owned by `wf-recorder`.
fn get_pw_ports(direction: &str, predicate: impl Fn(&str) -> bool) -> Vec<String> {
    let Ok(output) = Command::new("pw-link").arg(direction).output() else {
        return Vec::new();
    };

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|port| predicate(port))
        .map(str::to_owned)
        .collect()
}

fn get_recorder_input_ports() -> Vec<String> {
    get_pw_ports("-i", |port| {
        port.contains("wf-recorder") && port.contains(":input_")
    })
}

/// Retrieves monitor ports (system desktop audio playback monitor).
fn get_monitor_output_ports() -> Vec<String> {
    get_pw_ports("-o", |port| {
        port.contains(":monitor_") && !port.contains("wf-recorder")
    })
}

/// Retrieves microphone / voice capture ports.
fn get_mic_output_ports() -> Vec<String> {
    get_pw_ports("-o", |port| {
        (port.contains(":capture_") || port.contains("input"))
            && !port.contains(":monitor_")
            && !port.contains("wf-recorder")
    })
}

/// Links or unlinks audio port pairs between an output and recorder input ports.
fn link_channel_pairs(outputs: &[String], inputs: &[String], disconnect: bool) {
    if outputs.is_empty() || inputs.is_empty() {
        return;
    }

    for out in outputs {
        for inp in inputs {
            let is_match = (out.contains("_FL") && inp.contains("_FL"))
                || (out.contains("_FR") && inp.contains("_FR"))
                || (out.contains("_0") && inp.contains("_FL"))
                || (out.contains("_1") && inp.contains("_FR"))
                || (outputs.len() == 1); // Mono source connects to all inputs

            if is_match {
                let mut cmd = Command::new("pw-link");
                if disconnect {
                    cmd.arg("-d");
                }
                cmd.args([out, inp]);
                let _ = cmd.output();
            }
        }
    }
}

/// Fallback mechanism: toggles mute on wf-recorder source-output via `pactl`.
fn pactl_set_recorder_mute(pid: u32, muted: bool) {
    let out = Command::new("pactl")
        .args(["list", "source-outputs"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default();

    let mut current_id: Option<String> = None;
    let pid_str = format!("application.process.id = \"{pid}\"");

    for line in out.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Source Output #") {
            current_id = trimmed
                .strip_prefix("Source Output #")
                .map(|s| s.to_string());
        }
        if (trimmed.contains(&pid_str)
            || trimmed.contains("application.process.binary = \"wf-recorder\""))
            && current_id.is_some()
        {
            if let Some(id) = current_id.take() {
                let mute_arg = if muted { "1" } else { "0" };
                let _ = Command::new("pactl")
                    .args(["set-source-output-mute", &id, mute_arg])
                    .output();
                break;
            }
        }
    }
}

/// Initializes audio state for a new recording session.
pub fn init_recording_audio(pid: u32, audio_enabled: bool) {
    ACTIVE_PID.store(pid, Ordering::SeqCst);
    AUDIO_ENABLED.store(audio_enabled, Ordering::SeqCst);
    IS_AUDIO_MUTED.store(false, Ordering::SeqCst);
    IS_MIC_MUTED.store(false, Ordering::SeqCst);

    if audio_enabled {
        // Allow wf-recorder to initialize its PipeWire node before discovering ports
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(350));
            if ACTIVE_PID.load(Ordering::SeqCst) != pid {
                return;
            }

            let inputs = get_recorder_input_ports();
            if !inputs.is_empty() {
                // Auto-link microphone into the recording stream so voice is also captured
                let mics = get_mic_output_ports();
                if !mics.is_empty() && !IS_MIC_MUTED.load(Ordering::SeqCst) {
                    link_channel_pairs(&mics, &inputs, false);
                }
            }
        });
    }
}

/// Cleans up state when a recording session terminates.
pub fn reset_recording_audio() {
    ACTIVE_PID.store(0, Ordering::SeqCst);
    AUDIO_ENABLED.store(false, Ordering::SeqCst);
    IS_AUDIO_MUTED.store(false, Ordering::SeqCst);
    IS_MIC_MUTED.store(false, Ordering::SeqCst);
}

/// Checks whether the recorded video audio stream is muted.
pub fn is_recording_audio_muted() -> bool {
    if !AUDIO_ENABLED.load(Ordering::SeqCst) {
        return true;
    }
    IS_AUDIO_MUTED.load(Ordering::SeqCst)
}

/// Checks whether the recorded video microphone stream is muted.
pub fn is_recording_mic_muted() -> bool {
    if !AUDIO_ENABLED.load(Ordering::SeqCst) {
        return true;
    }
    IS_MIC_MUTED.load(Ordering::SeqCst)
}

/// Sets whether system desktop audio is recorded in the current video.
pub fn set_recording_audio_muted(muted: bool) -> bool {
    if !AUDIO_ENABLED.load(Ordering::SeqCst) {
        return true;
    }

    let inputs = get_recorder_input_ports();
    let monitors = get_monitor_output_ports();

    if !inputs.is_empty() && !monitors.is_empty() {
        link_channel_pairs(&monitors, &inputs, muted);
    } else {
        let pid = ACTIVE_PID.load(Ordering::SeqCst);
        if pid > 0 {
            pactl_set_recorder_mute(pid, muted);
        }
    }

    IS_AUDIO_MUTED.store(muted, Ordering::SeqCst);
    muted
}

/// Sets whether microphone voice is recorded in the current video.
pub fn set_recording_mic_muted(muted: bool) -> bool {
    if !AUDIO_ENABLED.load(Ordering::SeqCst) {
        return true;
    }

    let inputs = get_recorder_input_ports();
    let mics = get_mic_output_ports();

    if !inputs.is_empty() && !mics.is_empty() {
        link_channel_pairs(&mics, &inputs, muted);
    }

    IS_MIC_MUTED.store(muted, Ordering::SeqCst);
    muted
}

/// Toggles system desktop audio recording for the active video.
pub fn toggle_recording_audio_mute() -> bool {
    let curr = is_recording_audio_muted();
    set_recording_audio_muted(!curr)
}

/// Toggles microphone recording for the active video.
pub fn toggle_recording_mic_mute() -> bool {
    let curr = is_recording_mic_muted();
    set_recording_mic_muted(!curr)
}
