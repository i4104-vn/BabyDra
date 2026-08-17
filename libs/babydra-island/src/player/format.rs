//! Pure formatting & icon helpers for the media player UI.
//!
//! Split out of `player_loop.rs` so they can be unit-tested without GTK.

/// Formats a duration in seconds as `H:MM:SS` or `M:SS`.
pub fn format_time(secs: f64) -> String {
    if secs <= 0.0 || secs.is_nan() || secs.is_infinite() {
        return "0:00".to_string();
    }
    let total_seconds = secs as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{}:{:02}", minutes, seconds)
    }
}

/// Resolves an icon name for the active player (desktop-app aware).
pub fn get_player_icon_name(player_name_raw: &str) -> String {
    let lower_player = player_name_raw.to_lowercase();
    if lower_player.is_empty() {
        return "music".to_string();
    }

    // Dynamic search across registered desktop application entry files
    let apps = babydra_core::find_desktop_apps();
    for app in &apps {
        let app_name = app.name.to_lowercase();
        let app_exec = app.exec.to_lowercase();
        if app_exec.contains(&lower_player) || app_name.contains(&lower_player) {
            if let Some(ref icon) = app.icon {
                return icon.clone();
            }
        }
    }

    // Direct fallback using the raw name
    lower_player
}
