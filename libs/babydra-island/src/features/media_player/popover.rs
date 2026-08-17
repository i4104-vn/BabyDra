//! Glassmorphic media control popover anchored to the Dynamic Island capsule.
//!
//! Owned by the media player feature; the island manager dispatches capsule
//! clicks to the active view, which calls [`MediaPopover::toggle`].

use std::cell::Cell;
use std::rc::Rc;

use babydra_core::i18n::t;
use gtk4::prelude::*;

/// Widget references of the media control popover.
#[derive(Clone)]
pub(crate) struct MediaPopover {
    popover: gtk4::Popover,
    popover_box: gtk4::Box,
    is_animating: Rc<Cell<bool>>,
    /// Popover header label (track title).
    pub title: gtk4::Label,
    /// Popover subtitle label (artist).
    pub artist: gtk4::Label,
    /// Album art container inside the popover.
    pub art_container: gtk4::Box,
    /// Application name label.
    pub app_name: gtk4::Label,
    /// Play/pause toggle button icon.
    pub play_btn_icon: gtk4::Image,
    /// Progress bar container.
    pub progress_container: gtk4::Box,
    /// Media progress bar.
    pub progress_bar: gtk4::ProgressBar,
    /// Current position time label (e.g. `1:23`).
    pub position_lbl: gtk4::Label,
    /// Total track length time label (e.g. `3:45`).
    pub length_lbl: gtk4::Label,
}

impl MediaPopover {
    /// Builds and registers the popover anchored to the notch capsule.
    pub(crate) fn new(capsule: &gtk4::Box) -> Self {
        let popover = babydra_ui_kit::components::create_popover(
            capsule,
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
        let app_name = gtk4::Label::new(Some(&t("island.music_player")));
        app_name.add_css_class("media-popover-app-name");
        popover_header.append(&popover_app_icon);
        popover_header.append(&app_name);
        popover_box.append(&popover_header);

        let art_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        art_container.set_valign(gtk4::Align::Fill);
        art_container.set_halign(gtk4::Align::Fill);
        popover_box.append(&art_container);

        let title = gtk4::Label::new(Some(&t("island.unknown_title")));
        title.add_css_class("media-popover-title");
        title.set_halign(gtk4::Align::Center);
        title.set_justify(gtk4::Justification::Center);
        title.set_wrap(true);
        title.set_max_width_chars(25);

        let artist = gtk4::Label::new(Some(&t("island.unknown_artist")));
        artist.add_css_class("media-popover-artist");
        artist.set_halign(gtk4::Align::Center);
        artist.set_justify(gtk4::Justification::Center);
        artist.set_wrap(true);
        artist.set_max_width_chars(30);

        popover_box.append(&title);
        popover_box.append(&artist);

        // Progress bar container.
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

        let position_lbl = gtk4::Label::new(Some("0:00"));
        position_lbl.add_css_class("media-popover-time-label");
        position_lbl.set_halign(gtk4::Align::Start);

        let time_spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        time_spacer.set_hexpand(true);

        let length_lbl = gtk4::Label::new(Some("0:00"));
        length_lbl.add_css_class("media-popover-time-label");
        length_lbl.set_halign(gtk4::Align::End);

        time_box.append(&position_lbl);
        time_box.append(&time_spacer);
        time_box.append(&length_lbl);

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
        play_btn.connect_clicked(|_| {
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

        let popover_box_slide = popover_box.clone();
        let capsule_map = capsule.clone();
        popover.connect_map(move |_| {
            capsule_map.add_css_class("popover-open");
            babydra_ui_kit::ui::animation::slide_in(
                popover_box_slide.upcast_ref(),
                babydra_ui_kit::ui::animation::SlideDirection::Down,
                15,
                450,
            );
        });

        let capsule_unmap = capsule.clone();
        popover.connect_unmap(move |_| {
            capsule_unmap.remove_css_class("popover-open");
        });

        Self {
            popover,
            popover_box,
            is_animating: Rc::new(Cell::new(false)),
            title,
            artist,
            art_container,
            app_name,
            play_btn_icon,
            progress_container,
            progress_bar,
            position_lbl,
            length_lbl,
        }
    }

    /// Closes the popover.
    pub(crate) fn popdown(&self) {
        self.popover.popdown();
    }

    /// Toggles the popover with a slide animation.
    pub(crate) fn toggle(&self) {
        if self.is_animating.get() {
            return;
        }
        if self.popover.is_visible() {
            self.is_animating.set(true);
            let box_c = self.popover_box.clone();
            let popover_c = self.popover.clone();
            let anim_c = self.is_animating.clone();
            babydra_ui_kit::ui::animation::slide_out_cb(
                box_c.upcast_ref(),
                babydra_ui_kit::ui::animation::SlideDirection::Up,
                15,
                450,
                false,
                move || {
                    popover_c.popdown();
                    anim_c.set(false);
                },
            );
        } else {
            self.popover.popup();
        }
    }
}
