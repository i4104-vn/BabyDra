//! Volume control action triggers.

pub fn set_volume(val: f64) {
    let percent = val as i32;
    let _ = std::process::Command::new("wpctl")
        .args(&["set-volume", "@DEFAULT_AUDIO_SINK@", &format!("{}%", percent)])
        .spawn();
    let _ = std::process::Command::new("pactl")
        .args(&["set-sink-volume", "@DEFAULT_SINK@", &format!("{}%", percent)])
        .spawn();
    let _ = std::process::Command::new("amixer")
        .args(&["set", "Master", &format!("{}%", percent)])
        .spawn();

    if percent > 0 {
        let _ = std::process::Command::new("wpctl")
            .args(&["set-mute", "@DEFAULT_AUDIO_SINK@", "0"])
            .spawn();
        let _ = std::process::Command::new("pactl")
            .args(&["set-sink-mute", "@DEFAULT_SINK@", "0"])
            .spawn();
        let _ = std::process::Command::new("amixer")
            .args(&["set", "Master", "unmute"])
            .spawn();
    }
}

/// Toggles the master mute status.
pub fn set_muted(muted: bool) {
    let mute_val = if muted { "1" } else { "0" };
    let _ = std::process::Command::new("wpctl")
        .args(&["set-mute", "@DEFAULT_AUDIO_SINK@", mute_val])
        .spawn();
    let _ = std::process::Command::new("pactl")
        .args(&["set-sink-mute", "@DEFAULT_SINK@", mute_val])
        .spawn();
    let _ = std::process::Command::new("amixer")
        .args(&["set", "Master", if muted { "mute" } else { "unmute" }])
        .spawn();
}

/// Commands wpctl to activate/default a specific audio device or route configuration.
pub fn select_audio_device(name: &str) {
    if name.starts_with("route:") {
        let parts: Vec<&str> = name.split(':').collect();
        if parts.len() == 4 {
            let card_id = parts[1];
            let route_index = parts[2];
            let profile_index = parts[3];
            let _ = std::process::Command::new("wpctl")
                .args(&["set-profile", card_id, profile_index])
                .status();
            let _ = std::process::Command::new("wpctl")
                .args(&["set-route", card_id, route_index])
                .status();
        }
    } else if name.starts_with("profile:") {
        let parts: Vec<&str> = name.split(':').collect();
        if parts.len() == 3 {
            let card_id = parts[1];
            let profile_index = parts[2];
            let _ = std::process::Command::new("wpctl")
                .args(&["set-profile", card_id, profile_index])
                .status();
        }
    } else {
        let _ = std::process::Command::new("wpctl")
            .args(&["set-default", name])
            .status();
    }
}
