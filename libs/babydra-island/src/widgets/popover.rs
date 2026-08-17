//! UI layout renderer for the Dynamic Island media controller popover.

use babydra_core::i18n::t;
use gtk4::prelude::*;

/// Builds and registers the glassmorphic media control Popover anchored to the notch capsule.
pub fn create_media_popover(
    notch_capsule: &gtk4::Box,
    notification_view: &gtk4::Box,
) -> (
    gtk4::Popover,
    gtk4::Label,
    gtk4::Label,
    gtk4::Box,
    gtk4::Label,
    gtk4::Image,
    gtk4::Box,
    gtk4::ProgressBar,
    gtk4::Label,
    gtk4::Label,
) {
    let popover = babydra_ui_kit::components::create_popover(
        notch_capsule,
        gtk4::PositionType::Bottom,
        "media-popover",
    );
    popover.set_has_arrow(false);
    popover.set_offset(0, 10);

    let popover_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    popover_box.add_css_class("media-popover-box");

    let popover_header = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    popover_header.add_css_class("media-popover-header");
    popover_header.set_valign(gtk4::Align::Center);
    let popover_app_icon = babydra_ui_kit::ui::icon::get_icon_colored("logo", 14, "#3b82f6");
    let popover_app_name = gtk4::Label::new(Some(&t("island.music_player")));
    popover_app_name.add_css_class("media-popover-app-name");
    popover_header.append(&popover_app_icon);
    popover_header.append(&popover_app_name);
    popover_box.append(&popover_header);

    let popover_art_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    popover_art_container.set_valign(gtk4::Align::Fill);
    popover_art_container.set_halign(gtk4::Align::Fill);
    popover_box.append(&popover_art_container);

    let popover_title = gtk4::Label::new(Some(&t("island.unknown_title")));
    popover_title.add_css_class("media-popover-title");
    popover_title.set_halign(gtk4::Align::Center);
    popover_title.set_justify(gtk4::Justification::Center);
    popover_title.set_wrap(true);
    popover_title.set_max_width_chars(25);

    let popover_artist = gtk4::Label::new(Some(&t("island.unknown_artist")));
    popover_artist.add_css_class("media-popover-artist");
    popover_artist.set_halign(gtk4::Align::Center);
    popover_artist.set_justify(gtk4::Justification::Center);
    popover_artist.set_wrap(true);
    popover_artist.set_max_width_chars(30);

    popover_box.append(&popover_title);
    popover_box.append(&popover_artist);

    // Progress Bar Container
    let progress_container = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    progress_container.add_css_class("media-popover-progress-container");
    progress_container.set_margin_start(16);
    progress_container.set_margin_end(16);
    progress_container.set_margin_top(10);
    progress_container.set_margin_bottom(6);

    let progress_bar = gtk4::ProgressBar::new();
    progress_bar.add_css_class("media-popover-progress-bar");
    progress_bar.set_fraction(0.0);

    let time_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    time_box.add_css_class("media-popover-time-box");

    let pos_label = gtk4::Label::new(Some("0:00"));
    pos_label.add_css_class("media-popover-time-label");
    pos_label.set_halign(gtk4::Align::Start);

    let time_spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    time_spacer.set_hexpand(true);

    let len_label = gtk4::Label::new(Some("0:00"));
    len_label.add_css_class("media-popover-time-label");
    len_label.set_halign(gtk4::Align::End);

    time_box.append(&pos_label);
    time_box.append(&time_spacer);
    time_box.append(&len_label);

    progress_container.append(&progress_bar);
    progress_container.append(&time_box);
    popover_box.append(&progress_container);

    let controls_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 18);
    controls_box.add_css_class("media-popover-controls");
    controls_box.set_halign(gtk4::Align::Center);

    let prev_btn = gtk4::Button::new();
    prev_btn.add_css_class("media-control-btn");
    let prev_img = babydra_ui_kit::ui::icon::get_icon("previous", 16);
    prev_btn.set_child(Some(&prev_img));
    prev_btn.connect_clicked(|_| {
        let _ = std::process::Command::new("playerctl")
            .arg("previous")
            .spawn();
    });

    let play_btn = gtk4::Button::new();
    play_btn.add_css_class("media-control-btn");
    let play_btn_icon = babydra_ui_kit::ui::icon::get_icon("play", 22);
    play_btn.set_child(Some(&play_btn_icon));
    play_btn.connect_clicked(move |_| {
        let _ = std::process::Command::new("playerctl")
            .arg("play-pause")
            .spawn();
    });

    let next_btn = gtk4::Button::new();
    next_btn.add_css_class("media-control-btn");
    let next_img = babydra_ui_kit::ui::icon::get_icon("next", 16);
    next_btn.set_child(Some(&next_img));
    next_btn.connect_clicked(|_| {
        let _ = std::process::Command::new("playerctl").arg("next").spawn();
    });

    controls_box.append(&prev_btn);
    controls_box.append(&play_btn);
    controls_box.append(&next_btn);
    popover_box.append(&controls_box);

    popover.set_child(Some(&popover_box));

    let click_gesture = gtk4::GestureClick::new();
    let popover_clone = popover.clone();
    let popover_box_clone = popover_box.clone();

    let is_animating = std::rc::Rc::new(std::cell::Cell::new(false));
    let is_animating_clone = is_animating.clone();

    let notification_view_clone = notification_view.clone();
    click_gesture.connect_pressed(move |_, _, _, _| {
        if is_animating_clone.get() {
            return;
        }
        if notification_view_clone.is_visible() {
            let active_app_name = crate::widgets::notification::SHARED_NOTIFICATION
                .with(|sn| sn.borrow().as_ref().map(|n| n.icon.clone()));
            if let Some(app_name) = active_app_name {
                if !app_name.is_empty() && !app_name.starts_with('/') {
                    let _ = std::process::Command::new("wlrctl")
                        .args(&["window", "focus", &app_name])
                        .spawn();
                    let _ = std::process::Command::new("wlrctl")
                        .args(&["window", "focus", &app_name.to_lowercase()])
                        .spawn();
                    let _ = std::process::Command::new("wmctrl")
                        .args(&["-a", &app_name])
                        .spawn();
                }
            }
            return;
        }

        if popover_clone.is_visible() {
            let p_clone = popover_clone.clone();
            let is_animating_cb = is_animating_clone.clone();
            is_animating_cb.set(true);

            babydra_ui_kit::ui::animation::slide_out_cb(
                popover_box_clone.upcast_ref(),
                babydra_ui_kit::ui::animation::SlideDirection::Up,
                15,
                450,
                false,
                move || {
                    p_clone.popdown();
                    is_animating_cb.set(false);
                },
            );
        } else {
            popover_clone.popup();
        }
    });
    notch_capsule.add_controller(click_gesture);

    let popover_box_clone2 = popover_box.clone();
    let notch_capsule_clone = notch_capsule.clone();
    popover.connect_map(move |_| {
        notch_capsule_clone.add_css_class("popover-open");
        babydra_ui_kit::ui::animation::slide_in(
            popover_box_clone2.upcast_ref(),
            babydra_ui_kit::ui::animation::SlideDirection::Down,
            15,
            450,
        );
    });

    let notch_capsule_clone2 = notch_capsule.clone();
    popover.connect_unmap(move |_| {
        notch_capsule_clone2.remove_css_class("popover-open");
    });

    (
        popover,
        popover_title,
        popover_artist,
        popover_art_container,
        popover_app_name,
        play_btn_icon,
        progress_container,
        progress_bar,
        pos_label,
        len_label,
    )
}
