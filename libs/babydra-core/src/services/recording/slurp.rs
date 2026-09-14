//! Interactive region and window selection helpers using `slurp`.

use std::process::Command;

/// Uses `slurp` to interactively select a screen area and returns `(x, y, width, height)`.
pub fn select_area_with_slurp() -> Option<(i32, i32, i32, i32)> {
    let output = Command::new("slurp")
        .args(["-f", "%x,%y %w %h"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = text.trim().split([' ', ',']).collect();
    if parts.len() == 4 {
        let x = parts[0].parse::<i32>().ok()?;
        let y = parts[1].parse::<i32>().ok()?;
        let w = parts[2].parse::<i32>().ok()?;
        let h = parts[3].parse::<i32>().ok()?;
        Some((x, y, w, h))
    } else {
        None
    }
}

/// Uses `slurp` to interactively select a screen region and returns the raw geometry string (`"X,Y WxH"`).
pub fn select_geometry_str_with_slurp() -> Option<String> {
    let output = Command::new("slurp").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
