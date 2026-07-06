//! UI structure assembly for Dynamic Island overlays and notification sliders.

use gtk4::prelude::*;
use std::cell::Cell;
use std::rc::Rc;
use crate::widgets::visualizer::{create_visualizer, start_visualizer_animation};
use crate::player::player_loop::start_player_polling_loop;
use crate::models;
use crate::widgets;

/// Creates the main Dynamic Island box container, initializing Notch and notification badge layout hierarchies.
pub fn create_system_island() -> gtk4::Box {
    let container_vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    container_vbox.set_valign(gtk4::Align::Start);
    container_vbox.set_halign(gtk4::Align::Center);

    let notch_capsule = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    notch_capsule.add_css_class("panel-notch");
    notch_capsule.set_valign(gtk4::Align::Center);
    notch_capsule.set_halign(gtk4::Align::Center);
    notch_capsule.set_visible(false);

    let notch_content = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    notch_content.add_css_class("notch-content");
    notch_content.set_valign(gtk4::Align::Center);
    notch_content.set_halign(gtk4::Align::Fill);
    notch_content.set_hexpand(true);

    let default_view = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    default_view.set_valign(gtk4::Align::Center);
    default_view.set_halign(gtk4::Align::Center);
    let default_icon = babydra_common::icon::get_icon("logo", 12);
    default_view.append(&default_icon);
    notch_content.append(&default_view);

    let music_view = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    music_view.set_valign(gtk4::Align::Center);
    music_view.set_halign(gtk4::Align::Fill);
    music_view.set_hexpand(true);
    music_view.set_visible(false);

    let art_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    art_container.set_valign(gtk4::Align::Center);

    let track_label = gtk4::Label::new(Some("No media"));
    track_label.add_css_class("notch-player-text");
    track_label.set_hexpand(true);
    track_label.set_halign(gtk4::Align::Center);

    let (visualizer_box, bars) = create_visualizer();

    music_view.append(&art_container);
    music_view.append(&track_label);
    music_view.append(&visualizer_box);
    notch_content.append(&music_view);

    let notification_view = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    notification_view.set_valign(gtk4::Align::Center);
    notification_view.set_halign(gtk4::Align::Fill);
    notification_view.set_hexpand(true);
    notification_view.set_visible(false);

    let click_gesture = gtk4::GestureClick::new();
    click_gesture.set_button(0);
    click_gesture.connect_pressed(move |_, _, _, _| {
        let app_to_activate = widgets::notification::SHARED_NOTIFICATION.with(|sn| {
            sn.borrow().as_ref().map(|n| n.app_name.clone())
        });
        
        if let Some(app_name) = app_to_activate {
            let apps = babydra_common::desktop::find_desktop_apps();
            let mut found_app = None;
            let lower_name = app_name.to_lowercase();
            
            for app in &apps {
                if app.name.to_lowercase() == lower_name {
                    found_app = Some(app.clone());
                    break;
                }
            }
            
            if found_app.is_none() {
                for app in &apps {
                    if app.name.to_lowercase().contains(&lower_name) || lower_name.contains(&app.name.to_lowercase()) {
                        found_app = Some(app.clone());
                        break;
                    }
                }
            }
            
            if let Some(app) = found_app {
                let exec_parts: Vec<&str> = app.exec.split_whitespace().collect();
                let exec_name = if !exec_parts.is_empty() {
                    std::path::Path::new(exec_parts[0])
                        .file_name()
                        .map(|f| f.to_string_lossy().to_string())
                        .unwrap_or_default()
                } else {
                    String::new()
                };

                if !exec_name.is_empty() {
                    let _ = std::process::Command::new("wlrctl")
                        .args(&["window", "focus", &exec_name])
                        .spawn();
                    let _ = std::process::Command::new("wlrctl")
                        .args(&["window", "focus", &exec_name.to_lowercase()])
                        .spawn();
                }
                
                if !app.exec.is_empty() {
                    let _ = std::process::Command::new("wlrctl")
                        .args(&["window", "focus", &app.exec])
                        .spawn();
                }

                let _ = std::process::Command::new("wlrctl")
                    .args(&["window", "focus", &app.name])
                    .spawn();
            } else {
                let _ = std::process::Command::new("wlrctl")
                    .args(&["window", "focus", &app_name])
                    .spawn();
                let _ = std::process::Command::new("wlrctl")
                    .args(&["window", "focus", &app_name.to_lowercase()])
                    .spawn();
            }
        }
    });
    notification_view.add_controller(click_gesture);

    let notif_art_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    notif_art_container.set_valign(gtk4::Align::Center);

    let notif_text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    notif_text_box.set_valign(gtk4::Align::Center);
    notif_text_box.set_hexpand(true);

    let notif_title_lbl = gtk4::Label::new(None);
    notif_title_lbl.add_css_class("badge-title");
    notif_title_lbl.set_halign(gtk4::Align::Start);
    notif_title_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);

    let notif_body_lbl = gtk4::Label::new(None);
    notif_body_lbl.add_css_class("badge-desc");
    notif_body_lbl.set_halign(gtk4::Align::Start);
    notif_body_lbl.set_wrap(true);
    notif_body_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    notif_body_lbl.set_lines(2);

    notif_text_box.append(&notif_title_lbl);
    notif_text_box.append(&notif_body_lbl);

    notification_view.append(&notif_art_container);
    notification_view.append(&notif_text_box);
    notch_content.append(&notification_view);

    notch_capsule.append(&notch_content);
    container_vbox.append(&notch_capsule);

    let notification_badge = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    notification_badge.add_css_class("island-badge");
    notification_badge.set_valign(gtk4::Align::Start);
    notification_badge.set_halign(gtk4::Align::Center);
    notification_badge.set_visible(false);

    let badge_icon_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    badge_icon_container.set_valign(gtk4::Align::Center);
    let badge_icon = babydra_common::icon::get_icon_colored("bell", 14, "#3b82f6");
    badge_icon_container.append(&badge_icon);

    let badge_text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let badge_title = gtk4::Label::new(Some("Notification"));
    badge_title.add_css_class("badge-title");
    badge_title.set_halign(gtk4::Align::Start);
    let badge_desc = gtk4::Label::new(Some("New Message"));
    badge_desc.add_css_class("badge-desc");
    badge_desc.set_halign(gtk4::Align::Start);
    
    badge_text_box.append(&badge_title);
    badge_text_box.append(&badge_desc);

    notification_badge.append(&badge_icon_container);
    notification_badge.append(&badge_text_box);
    container_vbox.append(&notification_badge);

    let (
        _popover,
        popover_title,
        popover_artist,
        popover_art_container,
        popover_app_name,
        play_btn_icon,
    ) = widgets::popover::create_media_popover(&notch_capsule, &notification_view);

    let is_playing_state = Rc::new(Cell::new(false));

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<models::NotificationMsg>();
    widgets::notification::spawn_dbus_listener(tx);

    glib::MainContext::default().spawn_local(async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                models::NotificationMsg::New { summary, body, icon, app_name, timeout } => {
                    widgets::notification::show_notification_popup(&summary, &body, &icon, &app_name, timeout);
                }
                models::NotificationMsg::Close => {
                    widgets::notification::close_notification_popup();
                }
            }
        }
    });

    start_visualizer_animation(bars, is_playing_state.clone());
    let island_widgets = models::IslandWidgets {
        notch_capsule: notch_capsule.clone(),
        default_view,
        music_view,
        track_label,
        art_container,
        visualizer_box: visualizer_box.clone(),
        notification_badge,
        badge_title,
        badge_desc,
        badge_icon_container,
        popover_title,
        popover_artist,
        popover_art_container,
        popover_app_name,
        play_btn_icon,
        notification_view,
        notif_art_container,
        notif_title_lbl,
        notif_body_lbl,
    };
    start_player_polling_loop(
        is_playing_state.clone(),
        island_widgets,
    );

    container_vbox
}

