//! Volume and mute status querying.

pub fn is_muted() -> bool {
    if let Ok(output) = std::process::Command::new("wpctl")
        .args(&["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout.contains("[MUTED]");
    }
    if let Ok(output) = std::process::Command::new("pactl")
        .args(&["get-sink-mute", "@DEFAULT_SINK@"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout.contains("Mute: yes");
    }
    false
}

pub fn get_current_volume() -> f64 {
    if let Ok(output) = std::process::Command::new("wpctl")
        .args(&["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(vol_str) = stdout.split_whitespace().nth(1) {
            if let Ok(vol) = vol_str.parse::<f64>() {
                return vol * 100.0;
            }
        }
    }
    if let Ok(output) = std::process::Command::new("pactl")
        .args(&["get-sink-volume", "@DEFAULT_SINK@"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(pos) = stdout.find('%') {
            let start = stdout[..pos].rfind(' ').unwrap_or(0);
            if let Ok(vol) = stdout[start..pos].trim().parse::<f64>() {
                return vol;
            }
        }
    }
    80.0
}
