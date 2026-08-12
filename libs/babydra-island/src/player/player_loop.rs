//! Background scheduling loop managing the Dynamic Island transitions.
//! Handles switching between playerctl metadata updates and DBus notifications.

use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::playerctl::load_album_art_from_bytes;
use babydra_common::{run_playerctl, decode_uri};
use crate::models::IslandWidgets;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IslandState {
    Hidden,
    PlayerActive,
    NotificationActive { timestamp: std::time::Instant },
    ShrinkingToPlayer { had_player_before: bool },
    ZoomingOut,
}

thread_local! {
    pub static IS_NOTIF_HOVERED: Cell<bool> = Cell::new(false);
}

fn format_time(secs: f64) -> String {
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

fn set_art_fallback_icon(widgets: &IslandWidgets, icon_name: &str) {
    if let Some(child) = widgets.art_container.first_child() {
        widgets.art_container.remove(&child);
    }
    let music_icon_s = babydra_utils::ui::icon::get_icon_colored(icon_name, 14, "#3b82f6");
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

    let music_icon_l = babydra_utils::ui::icon::get_icon_colored(icon_name, 48, "#3b82f6");
    music_icon_l.set_halign(gtk4::Align::Center);
    music_icon_l.set_valign(gtk4::Align::Center);
    music_icon_l.set_hexpand(true);
    music_icon_l.set_vexpand(true);

    fallback_card.append(&music_icon_l);
    widgets.popover_art_container.append(&fallback_card);
}

/// Starts a background timer loop that polls active D-Bus notifications and playerctl
/// state every second. It orchestrates the Dynamic Island layout updates (compact logo,
/// active notification, or media player) and updates their corresponding widgets.
pub fn start_player_polling_loop(
    is_playing_state: Rc<Cell<bool>>,
    widgets: IslandWidgets,
) {
    let last_art_url = Rc::new(RefCell::new(String::new()));
    let last_attempted_url = Rc::new(RefCell::new(String::new()));
    let fail_count = Rc::new(Cell::new(0u32));
    let poll_counter = Rc::new(Cell::new(0u32));
    let last_title = Rc::new(RefCell::new(String::new()));
    let art_loaded_for_current_song = Rc::new(Cell::new(false));

    // Create a tokio channel to send playerctl metadata from the background thread to the main thread
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<Option<String>>();

    // Spawn background thread to poll playerctl metadata every second
    std::thread::spawn(move || {
        loop {
            let metadata = run_playerctl(&["metadata", "--format", "{{ status }}|//|{{ title }}|//|{{ artist }}|//|{{ playerName }}|//|{{ mpris:artUrl }}|//|{{ position }}|//|{{ mpris:length }}"]);
            if sender.send(metadata).is_err() {
                break; // Exit thread if receiver has been dropped
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    });

    let latest_metadata = Rc::new(RefCell::new(None));
    let latest_metadata_clone = latest_metadata.clone();

    // Hook up receiver to cache the latest metadata on the main thread
    glib::MainContext::default().spawn_local(async move {
        while let Some(metadata) = receiver.recv().await {
            *latest_metadata_clone.borrow_mut() = metadata;
        }
    });

    let (art_sender, mut art_receiver) = tokio::sync::mpsc::unbounded_channel::<(String, String, Result<Vec<u8>, ()>)>();
    let widgets_clone = widgets.clone();
    let last_art_url_clone = last_art_url.clone();
    let last_attempted_url_clone = last_attempted_url.clone();
    let art_loaded_clone = art_loaded_for_current_song.clone();
    let fail_count_clone = fail_count.clone();

    glib::MainContext::default().spawn_local(async move {
        while let Some((url, app_icon_name, result)) = art_receiver.recv().await {
            if url == *last_attempted_url_clone.borrow() {
                if let Ok(bytes) = result {
                    let small_art = load_album_art_from_bytes(&bytes, 16);
                    let large_art = load_album_art_from_bytes(&bytes, 240);

                    if let (Some(s_art), Some(l_art)) = (small_art, large_art) {
                        *last_art_url_clone.borrow_mut() = url;
                        art_loaded_clone.set(true);

                        if let Some(child) = widgets_clone.art_container.first_child() {
                            widgets_clone.art_container.remove(&child);
                        }
                        s_art.add_css_class("notch-album-art");
                        widgets_clone.art_container.append(&s_art);

                        if let Some(child) = widgets_clone.popover_art_container.first_child() {
                            widgets_clone.popover_art_container.remove(&child);
                        }
                        l_art.add_css_class("media-popover-art");
                        l_art.set_hexpand(true);
                        l_art.set_vexpand(true);
                        l_art.set_halign(gtk4::Align::Center);
                        l_art.set_valign(gtk4::Align::Center);
                        widgets_clone.popover_art_container.append(&l_art);
                    } else {
                        let current_fails = fail_count_clone.get() + 1;
                        fail_count_clone.set(current_fails);
                        if current_fails >= 3 {
                            *last_art_url_clone.borrow_mut() = url;
                            art_loaded_clone.set(true);

                            set_art_fallback_icon(&widgets_clone, &app_icon_name);
                        } else {
                            *last_attempted_url_clone.borrow_mut() = String::new();
                        }
                    }
                } else {
                    let current_fails = fail_count_clone.get() + 1;
                    fail_count_clone.set(current_fails);
                    if current_fails >= 3 {
                        *last_art_url_clone.borrow_mut() = url;
                        art_loaded_clone.set(true);

                        set_art_fallback_icon(&widgets_clone, &app_icon_name);
                    } else {
                        *last_attempted_url_clone.borrow_mut() = String::new();
                    }
                }
            }
        }
    });

    let island_state = Rc::new(Cell::new(IslandState::Hidden));

    // Main thread loop to check notifications and update player view from the cached metadata
    glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
        let current_state = island_state.get();
        let metadata_opt = latest_metadata.borrow().clone();

        let is_hovering = IS_NOTIF_HOVERED.with(|h| h.get());
        let mut active_notif = None;
        crate::widgets::notification::SHARED_NOTIFICATION.with(|sn| {
            if let Some(ref mut notif) = *sn.borrow_mut() {
                if is_hovering {
                    notif.timestamp = std::time::Instant::now();
                    active_notif = Some(notif.clone());
                } else if notif.timestamp.elapsed() < std::time::Duration::from_secs(5) {
                    active_notif = Some(notif.clone());
                }
            }
            if active_notif.is_none() {
                *sn.borrow_mut() = None;
            }
        });

        if matches!(current_state, IslandState::Hidden) && active_notif.is_none() && metadata_opt.is_none() {
            return glib::ControlFlow::Continue;
        }

        let metadata = metadata_opt;
        let mut player_active = false;
        let mut player_playing = false;
        let mut player_title = String::new();
        let mut player_artist = String::new();
        let mut player_name_raw = String::new();
        let mut player_art_url = String::new();
        let mut player_pos_secs = 0.0f64;
        let mut player_len_secs = 0.0f64;

        if let Some(ref line) = metadata {
            let parts: Vec<&str> = line.split("|//|").collect();
            if parts.len() >= 5 {
                let status_str = parts[0].trim();
                player_title = parts[1].trim().to_string();
                player_artist = parts[2].trim().to_string();
                player_name_raw = parts[3].trim().to_string();
                player_art_url = parts[4].trim().to_string();
                if parts.len() >= 7 {
                    let pos_us = parts[5].trim().parse::<f64>().unwrap_or(0.0);
                    let len_us = parts[6].trim().parse::<f64>().unwrap_or(0.0);
                    player_pos_secs = pos_us / 1_000_000.0;
                    player_len_secs = len_us / 1_000_000.0;
                }
                player_playing = status_str == "Playing";
                if status_str == "Playing" || status_str == "Paused" {
                    player_active = true;
                }
            }
        }

        if let Some(notif) = active_notif {
            let mut should_show_notif = false;
            match current_state {
                IslandState::Hidden => {
                    island_state.set(IslandState::NotificationActive { timestamp: notif.timestamp });
                    should_show_notif = true;
                    widgets.visualizer_box.set_visible(false);
                    widgets.notch_capsule.add_css_class("active-music");
                    widgets.notch_capsule.add_css_class("notification-mode");
                    babydra_utils::ui::animation::island_zoom_in(
                        widgets.notch_capsule.clone().upcast_ref(),
                        300,
                        70,
                        500,
                    );
                }
                IslandState::PlayerActive => {
                    island_state.set(IslandState::NotificationActive { timestamp: notif.timestamp });
                    should_show_notif = true;
                    widgets.visualizer_box.set_visible(false);
                    widgets.notch_capsule.add_css_class("notification-mode");
                    babydra_utils::ui::animation::island_animate_size(
                        widgets.notch_capsule.clone().upcast_ref(),
                        200,
                        300,
                        30,
                        70,
                        400,
                        || {},
                    );
                }
                IslandState::NotificationActive { timestamp } => {
                    if timestamp != notif.timestamp {
                        island_state.set(IslandState::NotificationActive { timestamp: notif.timestamp });
                        should_show_notif = true;
                    }
                }
                IslandState::ShrinkingToPlayer { .. } | IslandState::ZoomingOut => {
                    island_state.set(IslandState::NotificationActive { timestamp: notif.timestamp });
                    should_show_notif = true;
                    widgets.visualizer_box.set_visible(false);
                    widgets.notch_capsule.set_visible(true);
                    widgets.notch_capsule.add_css_class("active-music");
                    widgets.notch_capsule.add_css_class("notification-mode");
                    babydra_utils::ui::animation::island_animate_size(
                        widgets.notch_capsule.clone().upcast_ref(),
                        200,
                        300,
                        30,
                        70,
                        400,
                        || {},
                    );
                }
            }

            if should_show_notif {
                is_playing_state.set(false);
                update_notification_view(
                    &widgets,
                    &notif,
                    &last_art_url,
                    &last_attempted_url,
                );
            }
        } else {
            match current_state {
                IslandState::NotificationActive { .. } => {
                    island_state.set(IslandState::ShrinkingToPlayer { had_player_before: player_active });
                    
                    let state_clone = island_state.clone();
                    let widgets_clone = widgets.clone();
                    let last_title_clone = last_title.clone();
                    let art_loaded_clone = art_loaded_for_current_song.clone();
                    let is_playing_clone = is_playing_state.clone();
                    let latest_metadata_clone = latest_metadata.clone();
                    
                    let cur_w = widgets.notch_capsule.width().max(300);
                    let cur_h = widgets.notch_capsule.height().max(70);

                    widgets.notification_view.set_opacity(0.0);

                    let target_w = if player_active { 200 } else { 0 };
                    let target_h = if player_active { 30 } else { 0 };

                    babydra_utils::ui::animation::island_animate_size(
                        widgets.notch_capsule.clone().upcast_ref(),
                        cur_w,
                        target_w,
                        cur_h,
                        target_h,
                        250,
                        move || {
                            widgets_clone.notification_view.set_visible(false);
                            widgets_clone.notification_view.set_opacity(1.0);
                            widgets_clone.notch_capsule.remove_css_class("notification-mode");

                            let metadata_fresh = latest_metadata_clone.borrow().clone();
                            let mut player_active_fresh = false;
                            if let Some(ref line) = metadata_fresh {
                                let parts: Vec<&str> = line.split("|//|").collect();
                                if parts.len() >= 5 {
                                    let status_str = parts[0].trim();
                                    if status_str == "Playing" || status_str == "Paused" {
                                        player_active_fresh = true;
                                    }
                                }
                            }

                            if player_active_fresh {
                                state_clone.set(IslandState::PlayerActive);
                                last_title_clone.borrow_mut().clear();
                                art_loaded_clone.set(false);
                                widgets_clone.music_view.set_visible(true);
                                widgets_clone.visualizer_box.set_visible(true);
                            } else {
                                state_clone.set(IslandState::Hidden);
                                is_playing_clone.set(false);
                                widgets_clone.notch_capsule.set_visible(false);
                                widgets_clone.notch_capsule.remove_css_class("active-music");
                                babydra_utils::ui::icon::set_image_from_icon(&widgets_clone.play_btn_icon, "play", 22);
                                if let Some(child) = widgets_clone.art_container.first_child() {
                                    widgets_clone.art_container.remove(&child);
                                }
                                if let Some(child) = widgets_clone.popover_art_container.first_child() {
                                    widgets_clone.popover_art_container.remove(&child);
                                }
                            }
                        }
                    );
                }
                IslandState::PlayerActive => {
                    if player_active {
                        update_player_view(
                            &widgets,
                            &is_playing_state,
                            &poll_counter,
                            &last_title,
                            &art_loaded_for_current_song,
                            &last_art_url,
                            &last_attempted_url,
                            &fail_count,
                            player_playing,
                            &player_title,
                            &player_artist,
                            &player_name_raw,
                            &player_art_url,
                            player_pos_secs,
                            player_len_secs,
                            &art_sender,
                        );
                    } else {
                        island_state.set(IslandState::ZoomingOut);
                        is_playing_state.set(false);
                        widgets.play_btn_icon.set_icon_name(Some("media-playback-start-symbolic"));
                        if let Some(child) = widgets.art_container.first_child() {
                            widgets.art_container.remove(&child);
                        }
                        if let Some(child) = widgets.popover_art_container.first_child() {
                            widgets.popover_art_container.remove(&child);
                        }

                        babydra_utils::ui::animation::island_zoom_out(
                            widgets.notch_capsule.clone().upcast_ref(),
                            200,
                            500,
                            true,
                        );
                        
                        let state_final = island_state.clone();
                        let notch_clone = widgets.notch_capsule.clone();
                        glib::timeout_add_local_once(std::time::Duration::from_millis(500), move || {
                            state_final.set(IslandState::Hidden);
                            notch_clone.remove_css_class("active-music");
                            notch_clone.remove_css_class("notification-mode");
                        });
                    }
                }
                IslandState::Hidden => {
                    if player_active {
                        island_state.set(IslandState::PlayerActive);
                        widgets.music_view.set_visible(true);
                        widgets.visualizer_box.set_visible(true);
                        widgets.notch_capsule.add_css_class("active-music");
                        babydra_utils::ui::animation::island_zoom_in(
                            widgets.notch_capsule.clone().upcast_ref(),
                            200,
                            30,
                            500,
                        );
                        
                        update_player_view(
                            &widgets,
                            &is_playing_state,
                            &poll_counter,
                            &last_title,
                            &art_loaded_for_current_song,
                            &last_art_url,
                            &last_attempted_url,
                            &fail_count,
                            player_playing,
                            &player_title,
                            &player_artist,
                            &player_name_raw,
                            &player_art_url,
                            player_pos_secs,
                            player_len_secs,
                            &art_sender,
                        );
                    }
                }
                IslandState::ShrinkingToPlayer { .. } | IslandState::ZoomingOut => {}
            }
        }

        glib::ControlFlow::Continue
    });
}

/// Updates the Dynamic Island views to display incoming D-Bus notification details,
/// resolves system icon paths.
fn update_notification_view(
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
        babydra_utils::ui::icon::get_icon("logo", 24)
    } else {
        babydra_utils::ui::icon::get_system_or_file_icon(&notif.icon, "preferences-system-notifications-symbolic")
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

fn get_player_icon_name(player_name_raw: &str) -> String {
    let lower_player = player_name_raw.to_lowercase();
    if lower_player.is_empty() {
        return "music".to_string();
    }

    // Dynamic search across registered desktop application entry files
    let apps = babydra_common::find_desktop_apps();
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

/// Updates the Dynamic Island views to display metadata from the active media player,
/// handles loading and scaling cover artwork with failure retries, and synchronizes popover controls.
fn update_player_view(
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

    // Update Progress Bar
    if len_secs > 0.0 {
        let fraction = (pos_secs / len_secs).clamp(0.0, 1.0);
        widgets.popover_progress_bar.set_fraction(fraction);
        widgets.popover_position_lbl.set_text(&format_time(pos_secs));
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
        let pop_artist = if artist.is_empty() { "Playing Media" } else { artist };

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
                    } else if art_url_clone.starts_with("http://") || art_url_clone.starts_with("https://") {
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
        babydra_utils::ui::icon::set_image_from_icon(&widgets.play_btn_icon, "pause", 22);
    } else {
        babydra_utils::ui::icon::set_image_from_icon(&widgets.play_btn_icon, "play", 22);
    }

    widgets.default_view.set_visible(false);
    widgets.music_view.set_visible(true);
    if !widgets.notch_capsule.is_visible() {
        widgets.notch_capsule.add_css_class("active-music");
        babydra_utils::ui::animation::island_zoom_in(
            widgets.notch_capsule.clone().upcast_ref(),
            200,
            10,
            500,
        );
    }
}

