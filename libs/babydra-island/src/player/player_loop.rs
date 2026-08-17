//! State machine + polling loop for the Dynamic Island media player.
//! Background scheduling loop managing transitions between playerctl metadata
//! updates and DBus notifications. View rendering moved to `views.rs`,
//! pure helpers to `format.rs`.

use super::format::format_time;
use super::views::{set_art_fallback_icon, update_notification_view, update_player_view};
use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::playerctl::load_album_art_from_bytes;
use crate::models::IslandWidgets;
use babydra_core::{decode_uri, run_playerctl};

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

/// Starts a background timer loop that polls active D-Bus notifications and playerctl
/// state every second. It orchestrates the Dynamic Island layout updates (compact logo,
/// active notification, or media player) and updates their corresponding widgets.
pub fn start_player_polling_loop(is_playing_state: Rc<Cell<bool>>, widgets: IslandWidgets) {
    let last_art_url = Rc::new(RefCell::new(String::new()));
    let last_attempted_url = Rc::new(RefCell::new(String::new()));
    let fail_count = Rc::new(Cell::new(0u32));
    let poll_counter = Rc::new(Cell::new(0u32));
    let last_title = Rc::new(RefCell::new(String::new()));
    let art_loaded_for_current_song = Rc::new(Cell::new(false));

    // Create a tokio channel to send playerctl metadata from the background thread to the main thread
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<Option<String>>();

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

    let (art_sender, mut art_receiver) =
        tokio::sync::mpsc::unbounded_channel::<(String, String, Result<Vec<u8>, ()>)>();
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

        if matches!(current_state, IslandState::Hidden)
            && active_notif.is_none()
            && metadata_opt.is_none()
        {
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
            let is_new_notif = match current_state {
                IslandState::NotificationActive { timestamp } => timestamp != notif.timestamp,
                _ => true,
            };

            if is_new_notif {
                is_playing_state.set(false);
                update_notification_view(&widgets, &notif, &last_art_url, &last_attempted_url);
            }

            let target_width = 280;
            let (_, nat_height, _, _) = widgets
                .notification_view
                .measure(gtk4::Orientation::Vertical, target_width - 32);
            let target_height = (nat_height + 16).max(48); // 16px for top+bottom padding, at least 48px

            match current_state {
                IslandState::Hidden => {
                    island_state.set(IslandState::NotificationActive {
                        timestamp: notif.timestamp,
                    });
                    widgets.visualizer_box.set_visible(false);
                    widgets.notch_capsule.add_css_class("active-music");
                    widgets.notch_capsule.add_css_class("notification-mode");
                    babydra_ui_kit::ui::animation::island_zoom_in(
                        widgets.notch_capsule.clone().upcast_ref(),
                        target_width,
                        target_height,
                        350,
                    );
                }
                IslandState::PlayerActive => {
                    island_state.set(IslandState::NotificationActive {
                        timestamp: notif.timestamp,
                    });
                    widgets.visualizer_box.set_visible(false);
                    widgets.notch_capsule.add_css_class("notification-mode");
                    babydra_ui_kit::ui::animation::island_animate_size(
                        widgets.notch_capsule.clone().upcast_ref(),
                        200,
                        target_width,
                        26,
                        target_height,
                        350,
                        || {},
                    );
                }
                IslandState::NotificationActive { timestamp } => {
                    if timestamp != notif.timestamp {
                        island_state.set(IslandState::NotificationActive {
                            timestamp: notif.timestamp,
                        });
                        babydra_ui_kit::ui::animation::island_animate_size(
                            widgets.notch_capsule.clone().upcast_ref(),
                            widgets.notch_capsule.width(),
                            target_width,
                            widgets.notch_capsule.height(),
                            target_height,
                            250,
                            || {},
                        );
                    }
                }
                IslandState::ShrinkingToPlayer { .. } | IslandState::ZoomingOut => {
                    island_state.set(IslandState::NotificationActive {
                        timestamp: notif.timestamp,
                    });
                    widgets.visualizer_box.set_visible(false);
                    widgets.notch_capsule.set_visible(true);
                    widgets.notch_capsule.add_css_class("active-music");
                    widgets.notch_capsule.add_css_class("notification-mode");
                    babydra_ui_kit::ui::animation::island_animate_size(
                        widgets.notch_capsule.clone().upcast_ref(),
                        200,
                        target_width,
                        26,
                        target_height,
                        350,
                        || {},
                    );
                }
            }
        } else {
            match current_state {
                IslandState::NotificationActive { .. } => {
                    island_state.set(IslandState::ShrinkingToPlayer {
                        had_player_before: player_active,
                    });

                    let state_clone = island_state.clone();
                    let widgets_clone = widgets.clone();
                    let last_title_clone = last_title.clone();
                    let art_loaded_clone = art_loaded_for_current_song.clone();
                    let is_playing_clone = is_playing_state.clone();
                    let latest_metadata_clone = latest_metadata.clone();

                    let cur_w = widgets.notch_capsule.width().max(200);
                    let cur_h = widgets.notch_capsule.height().max(26);

                    widgets.notification_view.set_opacity(0.0);

                    let target_w = if player_active { 200 } else { 0 };
                    let target_h = if player_active { 26 } else { 0 };

                    babydra_ui_kit::ui::animation::island_animate_size(
                        widgets.notch_capsule.clone().upcast_ref(),
                        cur_w,
                        target_w,
                        cur_h,
                        target_h,
                        250,
                        move || {
                            widgets_clone.notification_view.set_visible(false);
                            widgets_clone.notification_view.set_opacity(1.0);
                            widgets_clone
                                .notch_capsule
                                .remove_css_class("notification-mode");

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
                                babydra_ui_kit::ui::icon::set_image_from_icon(
                                    &widgets_clone.play_btn_icon,
                                    "play",
                                    22,
                                );
                                if let Some(child) = widgets_clone.art_container.first_child() {
                                    widgets_clone.art_container.remove(&child);
                                }
                                if let Some(child) =
                                    widgets_clone.popover_art_container.first_child()
                                {
                                    widgets_clone.popover_art_container.remove(&child);
                                }
                            }
                        },
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
                        widgets
                            .play_btn_icon
                            .set_icon_name(Some("media-playback-start-symbolic"));
                        if let Some(child) = widgets.art_container.first_child() {
                            widgets.art_container.remove(&child);
                        }
                        if let Some(child) = widgets.popover_art_container.first_child() {
                            widgets.popover_art_container.remove(&child);
                        }

                        babydra_ui_kit::ui::animation::island_zoom_out(
                            widgets.notch_capsule.clone().upcast_ref(),
                            200,
                            500,
                            true,
                        );

                        let state_final = island_state.clone();
                        let notch_clone = widgets.notch_capsule.clone();
                        glib::timeout_add_local_once(
                            std::time::Duration::from_millis(500),
                            move || {
                                state_final.set(IslandState::Hidden);
                                notch_clone.remove_css_class("active-music");
                                notch_clone.remove_css_class("notification-mode");
                            },
                        );
                    }
                }
                IslandState::Hidden => {
                    if player_active {
                        island_state.set(IslandState::PlayerActive);
                        widgets.music_view.set_visible(true);
                        widgets.visualizer_box.set_visible(true);
                        widgets.notch_capsule.add_css_class("active-music");
                        babydra_ui_kit::ui::animation::island_zoom_in(
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
