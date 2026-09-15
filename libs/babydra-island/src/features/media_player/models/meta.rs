//! Playerctl metadata models for the media player island feature.

/// Parsed playerctl metadata for one refresh cycle.
#[derive(Default, Clone, Debug)]
pub struct PlayerMeta {
    pub playing: bool,
    pub title: String,
    pub artist: String,
    pub player_name_raw: String,
    pub art_url: String,
    pub pos_secs: f64,
    pub len_secs: f64,
}

/// Parses one raw `playerctl metadata --format` line.
/// Returns `(meta, player_active)` — active when status is Playing/Paused.
pub fn parse_metadata(line: &str) -> (PlayerMeta, bool) {
    let mut meta = PlayerMeta::default();
    let parts: Vec<&str> = line.split("|//|").collect();
    if parts.len() < 5 {
        return (meta, false);
    }
    let status_str = parts[0].trim();
    meta.title = parts[1].trim().to_string();
    meta.artist = parts[2].trim().to_string();
    meta.player_name_raw = parts[3].trim().to_string();
    meta.art_url = parts[4].trim().to_string();
    if parts.len() >= 7 {
        let pos_us = parts[5].trim().parse::<f64>().unwrap_or(0.0);
        let len_us = parts[6].trim().parse::<f64>().unwrap_or(0.0);
        meta.pos_secs = pos_us / 1_000_000.0;
        meta.len_secs = len_us / 1_000_000.0;
    }
    let is_playing = status_str.eq_ignore_ascii_case("playing");
    let is_paused = status_str.eq_ignore_ascii_case("paused");
    meta.playing = is_playing;
    (meta, is_playing || is_paused)
}
