//! Background scheduling loop managing the Dynamic Island transitions.
//! Handles switching between playerctl metadata updates and DBus notifications.

use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::playerctl::{run_playerctl, load_album_art_from_bytes, decode_uri};
use crate::models::IslandWidgets;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IslandState {
    Hidden,
    PlayerActive,
    NotificationActive { timestamp: std::time::Instant },
    ShrinkingToPlayer { had_player_before: bool },
    ZoomingOut,
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
    let _was_custom_active = Rc::new(Cell::new(false));
    let poll_counter = Rc::new(Cell::new(0u32));
    let last_title = Rc::new(RefCell::new(String::new()));
    let art_loaded_for_current_song = Rc::new(Cell::new(false));

    // Create a tokio channel to send playerctl metadata from the background thread to the main thread
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<Option<String>>();

    // Spawn background thread to poll playerctl metadata every second
    std::thread::spawn(move || {
        loop {
            let metadata = run_playerctl(&["metadata", "--format", "{{ status }}|//|{{ title }}|//|{{ artist }}|//|{{ playerName }}|//|{{ mpris:artUrl }}"]);
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

                            if let Some(child) = widgets_clone.art_container.first_child() {
                                widgets_clone.art_container.remove(&child);
                            }
                            let music_icon_s = babydra_common::icon::get_icon_colored(&app_icon_name, 14, "#3b82f6");
                            music_icon_s.add_css_class("notch-album-art");
                            widgets_clone.art_container.append(&music_icon_s);

                            if let Some(child) = widgets_clone.popover_art_container.first_child() {
                                widgets_clone.popover_art_container.remove(&child);
                            }
                            let music_icon_l = babydra_common::icon::get_icon_colored(&app_icon_name, 120, "#3b82f6");
                            music_icon_l.add_css_class("media-popover-art");
                            music_icon_l.set_size_request(240, 240);
                            music_icon_l.set_hexpand(true);
                            music_icon_l.set_vexpand(true);
                            music_icon_l.set_halign(gtk4::Align::Fill);
                            music_icon_l.set_valign(gtk4::Align::Fill);
                            widgets_clone.popover_art_container.append(&music_icon_l);
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

                        if let Some(child) = widgets_clone.art_container.first_child() {
                            widgets_clone.art_container.remove(&child);
                        }
                        let music_icon_s = babydra_common::icon::get_icon_colored(&app_icon_name, 14, "#3b82f6");
                        music_icon_s.add_css_class("notch-album-art");
                        widgets_clone.art_container.append(&music_icon_s);

                        if let Some(child) = widgets_clone.popover_art_container.first_child() {
                            widgets_clone.popover_art_container.remove(&child);
                        }
                        let music_icon_l = babydra_common::icon::get_icon_colored(&app_icon_name, 120, "#3b82f6");
                        music_icon_l.add_css_class("media-popover-art");
                        music_icon_l.set_size_request(240, 240);
                        music_icon_l.set_hexpand(true);
                        music_icon_l.set_vexpand(true);
                        music_icon_l.set_halign(gtk4::Align::Fill);
                        music_icon_l.set_valign(gtk4::Align::Fill);
                        widgets_clone.popover_art_container.append(&music_icon_l);
                    } else {
                        *last_attempted_url_clone.borrow_mut() = String::new();
                    }
                }
            }
        }
    });

    let island_state = Rc::new(Cell::new(IslandState::Hidden));

    // Main thread loop to check notifications and update player view from the cached metadata
    glib::timeout_add_local(std::time::Duration::from_millis(1000), move || {
        let mut active_notif = None;
        crate::widgets::notification::SHARED_NOTIFICATION.with(|sn| {
            if let Some(ref notif) = *sn.borrow() {
                if notif.timestamp.elapsed() < std::time::Duration::from_secs(5) {
                    active_notif = Some(notif.clone());
                }
            }
            if active_notif.is_none() {
                *sn.borrow_mut() = None;
            }
        });

        let metadata = latest_metadata.borrow().clone();
        let mut player_active = false;
        let mut player_playing = false;
        let mut player_title = String::new();
        let mut player_artist = String::new();
        let mut player_name_raw = String::new();
        let mut player_art_url = String::new();

        if let Some(ref line) = metadata {
            let parts: Vec<&str> = line.split("|//|").collect();
            if parts.len() >= 5 {
                let status_str = parts[0].trim();
                player_title = parts[1].trim().to_string();
                player_artist = parts[2].trim().to_string();
                player_name_raw = parts[3].trim().to_string();
                player_art_url = parts[4].trim().to_string();
                player_playing = status_str == "Playing";
                if status_str == "Playing" || status_str == "Paused" {
                    player_active = true;
                }
            }
        }

        let current_state = island_state.get();

        if let Some(notif) = active_notif {
            let mut should_show_notif = false;
            match current_state {
                IslandState::Hidden => {
                    island_state.set(IslandState::NotificationActive { timestamp: notif.timestamp });
                    should_show_notif = true;
                    widgets.visualizer_box.set_visible(false);
                    widgets.notch_capsule.add_css_class("active-music");
                    babydra_common::animation::island_zoom_in(
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
                    babydra_common::animation::island_animate_size(
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
                    babydra_common::animation::island_animate_size(
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
                    
                    babydra_common::animation::island_animate_size(
                        widgets.notch_capsule.clone().upcast_ref(),
                        280,
                        200,
                        70,
                        30,
                        400,
                        move || {
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

                            widgets_clone.notification_view.set_visible(false);

                            if player_active_fresh {
                                state_clone.set(IslandState::PlayerActive);
                                last_title_clone.borrow_mut().clear();
                                art_loaded_clone.set(false);
                                widgets_clone.music_view.set_visible(true);
                                widgets_clone.visualizer_box.set_visible(true);
                            } else {
                                state_clone.set(IslandState::ZoomingOut);
                                is_playing_clone.set(false);
                                widgets_clone.play_btn_icon.set_icon_name(Some("media-playback-start-symbolic"));
                                if let Some(child) = widgets_clone.art_container.first_child() {
                                    widgets_clone.art_container.remove(&child);
                                }
                                if let Some(child) = widgets_clone.popover_art_container.first_child() {
                                    widgets_clone.popover_art_container.remove(&child);
                                }

                                babydra_common::animation::island_zoom_out(
                                    widgets_clone.notch_capsule.clone().upcast_ref(),
                                    200,
                                    500,
                                    true,
                                );
                                
                                let state_final = state_clone.clone();
                                let notch_clone = widgets_clone.notch_capsule.clone();
                                glib::timeout_add_local_once(std::time::Duration::from_millis(500), move || {
                                    state_final.set(IslandState::Hidden);
                                    notch_clone.remove_css_class("active-music");
                                });
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

                        babydra_common::animation::island_zoom_out(
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
                        });
                    }
                }
                IslandState::Hidden => {
                    if player_active {
                        island_state.set(IslandState::PlayerActive);
                        widgets.music_view.set_visible(true);
                        widgets.visualizer_box.set_visible(true);
                        widgets.notch_capsule.add_css_class("active-music");
                        babydra_common::animation::island_zoom_in(
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
    let icon_symbol = if notif.icon.is_empty() { "preferences-system-notifications-symbolic" } else { &notif.icon };
    let notif_icon = babydra_common::icon::get_system_or_file_icon(icon_symbol, "preferences-system-notifications-symbolic");
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
    last_art_url: &RefCell<String>,
    last_attempted_url: &RefCell<String>,
    fail_count: &Cell<u32>,
    playing: bool,
    title: &str,
    artist: &str,
    player_name_raw: &str,
    art_url: &str,
    art_sender: &tokio::sync::mpsc::UnboundedSender<(String, String, Result<Vec<u8>, ()>)>,
) {
    is_playing_state.set(playing);

    let song_changed = {
        let mut last_title_borrow = last_title.borrow_mut();
        if title != *last_title_borrow {
            *last_title_borrow = title.to_string();
            true
        } else {
            false
        }
    };

    if song_changed {
        art_loaded_for_current_song.set(false);
        poll_counter.set(0);
    }

    let count = poll_counter.get();
    poll_counter.set(count + 1);

    if song_changed || count % 5 == 0 {
        let label_text = if artist.is_empty() {
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

        widgets.popover_title.set_text(title);
        widgets.popover_artist.set_text(artist);

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

    if !art_loaded_for_current_song.get() {
        let app_icon_name = get_player_icon_name(player_name_raw);
        let mut last_url = last_art_url.borrow_mut();

        if art_url.is_empty() {
            *last_url = art_url.to_string();
            art_loaded_for_current_song.set(true);

            if let Some(child) = widgets.art_container.first_child() {
                widgets.art_container.remove(&child);
            }
            let music_icon_s = babydra_common::icon::get_icon_colored(&app_icon_name, 14, "#3b82f6");
            music_icon_s.add_css_class("notch-album-art");
            widgets.art_container.append(&music_icon_s);

            if let Some(child) = widgets.popover_art_container.first_child() {
                widgets.popover_art_container.remove(&child);
            }
            let music_icon_l = babydra_common::icon::get_icon_colored(&app_icon_name, 120, "#3b82f6");
            music_icon_l.add_css_class("media-popover-art");
            music_icon_l.set_size_request(240, 240);
            music_icon_l.set_hexpand(true);
            music_icon_l.set_vexpand(true);
            music_icon_l.set_halign(gtk4::Align::Fill);
            music_icon_l.set_valign(gtk4::Align::Fill);
            widgets.popover_art_container.append(&music_icon_l);
        } else {
            if art_url != *last_attempted_url.borrow() {
                *last_attempted_url.borrow_mut() = art_url.to_string();
                fail_count.set(0);

                let art_url_clone = art_url.to_string();
                let app_icon_name_clone = app_icon_name.clone();
                let art_sender_clone = art_sender.clone();

                std::thread::spawn(move || {
                    let local_path = if let Some(path_str) = art_url_clone.strip_prefix("file://") {
                        decode_uri(&path_str)
                    } else if art_url_clone.starts_with('/') {
                        art_url_clone.to_string()
                    } else {
                        let _ = art_sender_clone.send((art_url_clone, app_icon_name_clone, Err(())));
                        return;
                    };

                    match std::fs::read(&local_path) {
                        Ok(bytes) => {
                            let _ = art_sender_clone.send((art_url_clone, app_icon_name_clone, Ok(bytes)));
                        }
                        Err(_) => {
                            let _ = art_sender_clone.send((art_url_clone, app_icon_name_clone, Err(())));
                        }
                    }
                });
            }
        }
    }

    if playing {
        widgets.play_btn_icon.set_icon_name(Some("media-playback-pause-symbolic"));
    } else {
        widgets.play_btn_icon.set_icon_name(Some("media-playback-start-symbolic"));
    }

    widgets.default_view.set_visible(false);
    widgets.music_view.set_visible(true);
    if !widgets.notch_capsule.is_visible() {
        widgets.notch_capsule.add_css_class("active-music");
        babydra_common::animation::island_zoom_in(
            widgets.notch_capsule.clone().upcast_ref(),
            200,
            10,
            500,
        );
    }
}

