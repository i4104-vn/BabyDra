use gtk4::prelude::*;
use std::cell::{Cell, RefCell};

use crate::models::IslandWidgets;
use babydra_core::decode_uri;

use super::format::{format_time, get_player_icon_name};

pub fn update_notification_view(
    widgets: &IslandWidgets,
    notif: &crate::models::ActiveNotification,
    last_art_url: &RefCell<String>,
    last_attempted_url: &RefCell<String>,
) {
    let truncated_title = if notif.title.chars().count() > 35 {
        notif.title.chars().take(35).collect::<String>() + "..."
    } else {
        notif.title.clone()
    };
    widgets.notif_title_lbl.set_text(&truncated_title);

    let truncated_body = if notif.body.chars().count() > 80 {
        notif.body.chars().take(80).collect::<String>() + "..."
    } else {
        notif.body.clone()
    };
    widgets.notif_body_lbl.set_text(&truncated_body);

    if let Some(child) = widgets.notif_art_container.first_child() {
        widgets.notif_art_container.remove(&child);
    }

    // Check if the icon file or system icon name exists, otherwise use logo
    let mut use_logo = notif.icon.is_empty();
    if !use_logo {
        if notif.icon.starts_with('/') {
            if !std::path::Path::new(&notif.icon).exists() {
                use_logo = true;
            }
        } else {
            let mut clean_name = notif.icon.clone();
            for ext in &[".png", ".svg", ".xpm", ".jpg", ".jpeg", ".gif"] {
                if clean_name.to_lowercase().ends_with(ext) {
                    clean_name = clean_name[..clean_name.len() - ext.len()].to_string();
                    break;
                }
            }
            if let Some(disp) = gdk4::Display::default() {
                let theme = gtk4::IconTheme::for_display(&disp);
                if !theme.has_icon(&clean_name) {
                    use_logo = true;
                }
            } else {
                use_logo = true;
            }
        }
    }

    let notif_icon = if use_logo {
        babydra_ui_kit::ui::icon::get_icon("logo", 24)
    } else {
        babydra_ui_kit::ui::icon::get_system_or_file_icon(
            &notif.icon,
            "preferences-system-notifications-symbolic",
        )
    };
    notif_icon.set_pixel_size(24);
    notif_icon.add_css_class("notch-album-art");
    widgets.notif_art_container.append(&notif_icon);

    *last_art_url.borrow_mut() = String::new();
    *last_attempted_url.borrow_mut() = String::new();

    widgets.default_view.set_visible(false);
    widgets.music_view.set_visible(false);
    widgets.notification_view.set_visible(true);
}

/// Updates the Dynamic Island views to display metadata from the active media player,
/// handles loading and scaling cover artwork with failure retries, and synchronizes popover controls.
pub fn update_player_view(
    widgets: &IslandWidgets,
    is_playing_state: &Cell<bool>,
    poll_counter: &Cell<u32>,
    last_title: &RefCell<String>,
    art_loaded_for_current_song: &Cell<bool>,
    _last_art_url: &RefCell<String>,
    last_attempted_url: &RefCell<String>,
    fail_count: &Cell<u32>,
    playing: bool,
    title: &str,
    artist: &str,
    player_name_raw: &str,
    art_url: &str,
    pos_secs: f64,
    len_secs: f64,
    art_sender: &tokio::sync::mpsc::UnboundedSender<(String, String, Result<Vec<u8>, ()>)>,
) {
    widgets.notch_capsule.remove_css_class("notification-mode");
    is_playing_state.set(playing);

    let count = poll_counter.get();
    poll_counter.set(count + 1);

    if len_secs > 0.0 {
        let fraction = (pos_secs / len_secs).clamp(0.0, 1.0);
        widgets.popover_progress_bar.set_fraction(fraction);
        widgets
            .popover_position_lbl
            .set_text(&format_time(pos_secs));
        widgets.popover_length_lbl.set_text(&format_time(len_secs));
        widgets.popover_progress_container.set_visible(true);
    } else {
        widgets.popover_progress_container.set_visible(false);
    }

    // Track metadata change (title, artist, or art_url)
    let meta_key = format!("{}|{}|{}|{}", title, artist, player_name_raw, art_url);
    let song_changed = {
        let mut last_title_borrow = last_title.borrow_mut();
        if meta_key != *last_title_borrow {
            *last_title_borrow = meta_key;
            true
        } else {
            false
        }
    };

    if song_changed {
        art_loaded_for_current_song.set(false);
        fail_count.set(0);
        *last_attempted_url.borrow_mut() = String::new();
    }

    // Always update labels when song/metadata changes or on periodic tick
    if song_changed || count % 2 == 0 {
        let label_text = if title.is_empty() {
            if !player_name_raw.is_empty() {
                player_name_raw.to_string()
            } else {
                "Media Player".to_string()
            }
        } else if artist.is_empty() {
            title.to_string()
        } else {
            format!("{} - {}", artist, title)
        };

        let display_text = if label_text.chars().count() > 18 {
            let truncated: String = label_text.chars().take(15).collect();
            format!("{}...", truncated)
        } else {
            label_text
        };
        widgets.track_label.set_text(&display_text);

        let pop_title = if title.is_empty() {
            if !player_name_raw.is_empty() {
                player_name_raw
            } else {
                "Media Player"
            }
        } else {
            title
        };
        let pop_artist = if artist.is_empty() {
            "Playing Media"
        } else {
            artist
        };

        widgets.popover_title.set_text(pop_title);
        widgets.popover_artist.set_text(pop_artist);

        let player_name = if !player_name_raw.is_empty() {
            let mut chars = player_name_raw.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        } else {
            "Music Player".to_string()
        };
        widgets.popover_app_name.set_text(&player_name);
    }

    // Artwork loading & retry logic
    if !art_loaded_for_current_song.get() {
        let app_icon_name = get_player_icon_name(player_name_raw);

        if art_url.is_empty() {
            // Display fallback icon for now, but keep retrying for 5 ticks in case browser delays artUrl
            set_art_fallback_icon(widgets, &app_icon_name);
            if count > 5 {
                art_loaded_for_current_song.set(true);
            }
        } else {
            let last_attempt = last_attempted_url.borrow().clone();
            let retries = fail_count.get();

            // Refetch if URL changed, or if previous attempt failed but retries < 3
            if art_url != last_attempt || retries < 3 {
                *last_attempted_url.borrow_mut() = art_url.to_string();

                let art_url_clone = art_url.to_string();
                let app_icon_name_clone = app_icon_name.clone();
                let art_sender_clone = art_sender.clone();

                std::thread::spawn(move || {
                    let result = if let Some(path_str) = art_url_clone.strip_prefix("file://") {
                        let local_path = decode_uri(&path_str);
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

    if playing {
        babydra_ui_kit::ui::icon::set_image_from_icon(&widgets.play_btn_icon, "pause", 22);
    } else {
        babydra_ui_kit::ui::icon::set_image_from_icon(&widgets.play_btn_icon, "play", 22);
    }

    widgets.default_view.set_visible(false);
    widgets.music_view.set_visible(true);
    if !widgets.notch_capsule.is_visible() {
        widgets.notch_capsule.add_css_class("active-music");
        babydra_ui_kit::ui::animation::island_zoom_in(
            widgets.notch_capsule.clone().upcast_ref(),
            200,
            10,
            500,
        );
    }
}

/// Sets the album-art containers to a fallback icon (used when no artwork URL is available).
pub fn set_art_fallback_icon(widgets: &IslandWidgets, icon_name: &str) {
    if let Some(child) = widgets.art_container.first_child() {
        widgets.art_container.remove(&child);
    }
    let music_icon_s = babydra_ui_kit::ui::icon::get_icon_colored(icon_name, 14, "#3b82f6");
    music_icon_s.add_css_class("notch-album-art");
    widgets.art_container.append(&music_icon_s);

    if let Some(child) = widgets.popover_art_container.first_child() {
        widgets.popover_art_container.remove(&child);
    }

    let fallback_card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    fallback_card.add_css_class("fallback-art-box");
    fallback_card.set_size_request(200, 130);
    fallback_card.set_hexpand(true);
    fallback_card.set_halign(gtk4::Align::Center);
    fallback_card.set_valign(gtk4::Align::Center);

    let music_icon_l = babydra_ui_kit::ui::icon::get_icon_colored(icon_name, 56, "#3b82f6");
    music_icon_l.set_halign(gtk4::Align::Center);
    music_icon_l.set_valign(gtk4::Align::Center);
    music_icon_l.set_hexpand(true);
    music_icon_l.set_vexpand(true);

    fallback_card.append(&music_icon_l);
    widgets.popover_art_container.append(&fallback_card);
}
