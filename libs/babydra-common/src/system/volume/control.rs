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
