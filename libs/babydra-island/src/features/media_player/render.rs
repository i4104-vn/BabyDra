//! Data → widgets: parses playerctl metadata and pushes it into the view.

use gtk4::prelude::*;

use super::art;
use super::format::{format_time, get_player_icon_name};
use super::MediaPlayerFeature;

/// Parsed playerctl metadata for one refresh cycle.
#[derive(Default)]
pub(crate) struct PlayerMeta {
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
pub(crate) fn parse_metadata(line: &str) -> (PlayerMeta, bool) {
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
    meta.playing = status_str == "Playing";
    let player_active = status_str == "Playing" || status_str == "Paused";
    (meta, player_active)
}

impl MediaPlayerFeature {
    /// Updates labels, progress and artwork (throttled where cheap wins).
    pub(crate) fn update_player_view(&self, meta: &PlayerMeta) {
        let count = self.poll_counter.get() + 1;
        self.poll_counter.set(count);

        let popover = self.popover.borrow();
        let popover = popover.as_ref();

        if let Some(popover) = popover {
            if meta.len_secs > 0.0 {
                let fraction = (meta.pos_secs / meta.len_secs).clamp(0.0, 1.0);
                popover.progress_bar.set_fraction(fraction);
                popover.position_lbl.set_text(&format_time(meta.pos_secs));
                popover.length_lbl.set_text(&format_time(meta.len_secs));
                popover.progress_container.set_visible(true);
            } else {
                popover.progress_container.set_visible(false);
            }
        }

        let meta_key = format!(
            "{}|{}|{}|{}",
            meta.title, meta.artist, meta.player_name_raw, meta.art_url
        );
        let song_changed = {
            let mut last = self.last_meta_key.borrow_mut();
            if meta_key != *last {
                *last = meta_key;
                true
            } else {
                false
            }
        };

        if song_changed {
            self.art_loaded_for_current_song.set(false);
            self.fail_count.set(0);
            *self.last_attempted_url.borrow_mut() = String::new();
        }

        if song_changed || count.is_multiple_of(7) {
            let label_text = if meta.title.is_empty() {
                if !meta.player_name_raw.is_empty() {
                    meta.player_name_raw.clone()
                } else {
                    "Media Player".to_string()
                }
            } else if meta.artist.is_empty() {
                meta.title.clone()
            } else {
                format!("{} - {}", meta.artist, meta.title)
            };

            let display_text = if label_text.chars().count() > 18 {
                let truncated: String = label_text.chars().take(15).collect();
                format!("{}...", truncated)
            } else {
                label_text
            };
            self.widgets.track_label.set_text(&display_text);

            if let Some(popover) = popover {
                let pop_title = if meta.title.is_empty() {
                    if !meta.player_name_raw.is_empty() {
                        meta.player_name_raw.clone()
                    } else {
                        "Media Player".to_string()
                    }
                } else {
                    meta.title.clone()
                };
                let pop_artist = if meta.artist.is_empty() {
                    "Playing Media"
                } else {
                    &meta.artist
                };
                popover.title.set_text(&pop_title);
                popover.artist.set_text(pop_artist);

                let player_name = if !meta.player_name_raw.is_empty() {
                    let mut chars = meta.player_name_raw.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                } else {
                    "Music Player".to_string()
                };
                popover.app_name.set_text(&player_name);
            }
        }

        // Artwork loading & retry logic.
        if !self.art_loaded_for_current_song.get() {
            let app_icon_name = get_player_icon_name(&meta.player_name_raw);
            let popover_art = popover.map(|p| p.art_container.clone());

            if meta.art_url.is_empty() {
                art::set_art_fallback_icon(
                    &self.widgets.art_container,
                    popover_art.as_ref(),
                    &app_icon_name,
                );
                if count > 5 {
                    self.art_loaded_for_current_song.set(true);
                }
            } else {
                let last_attempt = self.last_attempted_url.borrow().clone();
                let retries = self.fail_count.get();

                // Refetch if URL changed, or if previous attempt failed but retries < 3.
                if meta.art_url != last_attempt || retries < 3 {
                    *self.last_attempted_url.borrow_mut() = meta.art_url.clone();

                    let art_url_clone = meta.art_url.clone();
                    let app_icon_name_clone = app_icon_name.clone();
                    let art_sender_clone = self.art_sender.clone();

                    std::thread::spawn(move || {
                        let result = if let Some(path_str) = art_url_clone.strip_prefix("file://") {
                            let local_path = babydra_core::decode_uri(path_str);
                            std::fs::read(&local_path).map_err(|_| ())
                        } else if art_url_clone.starts_with('/') {
                            std::fs::read(&art_url_clone).map_err(|_| ())
                        } else if art_url_clone.starts_with("http://")
                            || art_url_clone.starts_with("https://")
                        {
                            std::process::Command::new("curl")
                                .args(["-s", "-L", "--max-time", "5", &art_url_clone])
                                .output()
                                .ok()
                                .filter(|o| o.status.success() && !o.stdout.is_empty())
                                .map(|o| o.stdout)
                                .ok_or(())
                        } else {
                            Err(())
                        };
                        let _ = art_sender_clone.send((art_url_clone, app_icon_name_clone, result));
                    });
                }
            }
        }

        if let Some(popover) = popover {
            let icon = if meta.playing { "pause" } else { "play" };
            babydra_ui_kit::ui::icon::set_image_from_icon(&popover.play_btn_icon, icon, 22);
        }
    }
}
