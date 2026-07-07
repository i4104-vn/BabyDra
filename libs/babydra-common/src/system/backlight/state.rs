//! Querying current monitor and notebook backlight state levels.

use super::detection::{DDC_BUS, has_backlight};

pub static BRIGHTNESS_STATE: std::sync::Mutex<f64> = std::sync::Mutex::new(60.0);

pub fn query_ddcutil_brightness() -> Option<f64> {
    let mut cmd = std::process::Command::new("ddcutil");
    if let Ok(guard) = DDC_BUS.lock() {
        if let Some(bus) = *guard {
            cmd.args(&["--bus", &bus.to_string()]);
        }
    }
    cmd.args(&["--sleep-multiplier", "0.1", "--disable-dynamic-sleep", "getvcp", "10", "--terse"]);
    
    if let Ok(output) = cmd.output() {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = stdout.split_whitespace().collect();
            if parts.len() >= 4 && parts[0] == "VCP" && (parts[1] == "10" || parts[1] == "0x10") {
                if let Ok(val) = parts[3].parse::<f64>() {
                    return Some(val);
                }
            }
            if let Some(pos) = stdout.find("current value =") {
                let start = pos + "current value =".len();
                let sub = &stdout[start..];
                let num_str: String = sub.chars()
                    .skip_while(|c| c.is_whitespace())
                    .take_while(|c| c.is_numeric())
                    .collect();
                if let Ok(val) = num_str.parse::<f64>() {
                    return Some(val);
                }
            }
        }
    }
    None
}

pub fn get_current_brightness() -> f64 {
    if has_backlight() {
        if let Ok(output) = std::process::Command::new("brightnessctl")
            .args(&["-m"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = stdout.lines().next() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 4 {
                    let pct_str = parts[3].trim_end_matches('%');
                    if let Ok(pct) = pct_str.parse::<f64>() {
                        return pct;
                    }
                }
            }
        }
    } else {
        if let Ok(guard) = BRIGHTNESS_STATE.lock() {
            return *guard;
        }
    }
    60.0
}
