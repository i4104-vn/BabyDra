//! Audio volume helpers calling `wpctl` and `pactl`.

/// Volume/Audio device configuration model.
#[derive(Clone, Debug)]
pub struct AudioDevice {
    pub name: String,
    pub description: String,
    pub is_default: bool,
}

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

pub fn get_audio_devices(is_source: bool) -> Vec<AudioDevice> {
    let mut devices = Vec::new();
    let output = std::process::Command::new("wpctl")
        .arg("status")
        .output();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let mut in_sinks_section = false;
        let mut in_sources_section = false;

        for line in stdout.lines() {
            let line_trimmed = line.trim();
            
            if line_trimmed.contains("Sinks:") {
                in_sinks_section = true;
                in_sources_section = false;
                continue;
            } else if line_trimmed.contains("Sources:") {
                in_sinks_section = false;
                in_sources_section = true;
                continue;
            } else if line_trimmed.contains("Devices:") || line_trimmed.contains("Filters:") || line_trimmed.contains("Streams:") || line_trimmed.contains("Settings:") {
                in_sinks_section = false;
                in_sources_section = false;
                continue;
            }

            let active_section = if is_source { in_sources_section } else { in_sinks_section };
            if !active_section {
                continue;
            }

            let clean_line = line.replace('│', "")
                                 .replace('├', "")
                                 .replace('└', "")
                                 .replace('─', "");
            let mut clean_trimmed = clean_line.trim().to_string();
            if clean_trimmed.is_empty() {
                continue;
            }

            let mut is_default = false;
            if clean_trimmed.starts_with('*') {
                is_default = true;
                clean_trimmed = clean_trimmed[1..].trim().to_string();
            }

            if let Some(dot_pos) = clean_trimmed.find('.') {
                let id_str = &clean_trimmed[..dot_pos];
                if id_str.chars().all(|c| c.is_ascii_digit()) {
                    let id = id_str.to_string();
                    let mut desc = clean_trimmed[dot_pos + 1..].trim().to_string();
                    
                    if let Some(bracket_pos) = desc.rfind('[') {
                        desc = desc[..bracket_pos].trim().to_string();
                    }
                    
                    if !id.is_empty() && !desc.is_empty() {
                        devices.push(AudioDevice {
                            name: id,
                            description: desc,
                            is_default,
                        });
                    }
                }
            }
        }
    }
    devices
}
