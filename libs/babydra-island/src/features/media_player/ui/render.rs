//! Data → widgets: parses playerctl metadata and pushes it into the view.

use std::time::Instant;

use gtk4::prelude::*;

use crate::features::media_player::service::{art, format_time};
use crate::features::media_player::MediaPlayerFeature;

/// Parsed playerctl metadata for one refresh cycle.
#[derive(Default)]
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
    meta.playing = status_str == "Playing";
    let player_active = status_str == "Playing" || status_str == "Paused";
    (meta, player_active)
}

impl MediaPlayerFeature {
    /// Updates labels, progress and artwork (throttled where cheap wins).
    pub(crate) fn update_player_view(&self, meta: &PlayerMeta, song_changed: bool) {
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

        // A view can be created after metadata has already been cached. In that
        // case `song_changed` is false, but the widgets still contain their
        // default placeholder text. Refresh labels whenever the view is first
        // rendered as well as when the track changes.
        if song_changed || !self.view_ready.get() {
            let fallback_title = babydra_core::i18n::trans("island.unknown_title");
            let fallback_artist = babydra_core::i18n::trans("island.unknown_artist");
            let display_title = if meta.title.is_empty() {
                if meta.artist.is_empty() {
                    fallback_title.clone()
                } else {
                    meta.artist.clone()
                }
            } else {
                meta.title.clone()
            };

            let label_text = if meta.title.is_empty() {
                if !meta.artist.is_empty() {
                    meta.artist.clone()
                } else if !meta.player_name_raw.is_empty() {
                    meta.player_name_raw.clone()
                } else {
                    babydra_core::i18n::trans("island.music_player")
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
                popover.title.set_text(&display_title);
                popover.artist.set_text(if meta.artist.is_empty() {
                    &fallback_artist
                } else {
                    &meta.artist
                });

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
            let app_icon_name = &self.player_icon_name;
            let popover_art = popover.map(|p| p.art_container.clone());

            if meta.art_url.is_empty() {
                art::set_art_fallback_icon(
                    &self.widgets.art_container,
                    popover_art.as_ref(),
                    app_icon_name,
                );
                self.art_loaded_for_current_song.set(true);
            } else {
                let last_attempt = self.last_attempted_url.borrow().clone();
                let retries = self.fail_count.get();
                let retry_ready = self
                    .next_art_retry_at
                    .get()
                    .map(|deadline| Instant::now() >= deadline)
                    .unwrap_or(true);

                if !self.art_request_pending.get()
                    && (meta.art_url != last_attempt || (retries < 3 && retry_ready))
                {
                    *self.last_attempted_url.borrow_mut() = meta.art_url.clone();
                    self.art_request_pending.set(true);
                    self.next_art_retry_at.set(None);

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
            if self.play_icon_state.get() != Some(meta.playing) {
                let icon = if meta.playing { "pause" } else { "play" };
                babydra_ui_kit::ui::icon::set_image_from_icon(&popover.play_btn_icon, icon, 22);
                self.play_icon_state.set(Some(meta.playing));
            }
        }
    }
}
