//! User action dispatchers via DBus and core services.

/// Toggles pause state via DBus or core signal.
pub fn toggle_pause_via_dbus_or_signal() {
    std::thread::spawn(|| {
        if let Ok(conn) = zbus::blocking::Connection::session() {
            if let Ok(proxy) = zbus::blocking::Proxy::new(
                &conn,
                "org.babydra.Recorder",
                "/org/babydra/Recorder",
                "org.babydra.Recorder",
            ) {
                let res: Result<bool, _> = proxy.call("TogglePause", &());
                if res.is_ok() {
                    return;
                }
            }
        }
        let _ = babydra_core::services::recording::toggle_pause();
    });
}

/// Dispatches a stop command to the recording service via DBus or core fallback.
pub fn stop_recording_via_dbus_or_signal() {
    std::thread::spawn(|| {
        if let Ok(conn) = zbus::blocking::Connection::session() {
            if let Ok(proxy) = zbus::blocking::Proxy::new(
                &conn,
                "org.babydra.Recorder",
                "/org/babydra/Recorder",
                "org.babydra.Recorder",
            ) {
                let res: Result<String, _> = proxy.call("StopRecording", &());
                if res.is_ok() {
                    return;
                }
            }
        }
        let _ = babydra_core::services::recording::stop_recording();
    });
}

/// Toggles system audio mute and returns new state.
pub fn toggle_audio_mute() -> bool {
    let muted = babydra_core::services::system::volume::is_muted();
    babydra_core::services::system::volume::set_muted(!muted);
    !muted
}

/// Returns whether system audio is currently muted.
pub fn is_audio_muted() -> bool {
    babydra_core::services::system::volume::is_muted()
}

/// Toggles microphone mute and returns new state.
pub fn toggle_mic_mute() -> bool {
    let muted = babydra_core::services::system::volume::is_microphone_muted();
    babydra_core::services::system::volume::set_microphone_muted(!muted);
    !muted
}

/// Returns whether microphone is currently muted.
pub fn is_mic_muted() -> bool {
    babydra_core::services::system::volume::is_microphone_muted()
}
