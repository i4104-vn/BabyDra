//! Widget construction for the compact notch player view.
//!
//! Visibility of `music_view` is managed by the island controller (it shows /
//! hides the view container), so the view itself stays visible by default.

use std::cell::Cell;
use std::rc::Rc;

use gtk4::prelude::*;

use super::visualizer::{create_visualizer, start_visualizer_animation};

/// Widget references of the compact notch player view.
pub(crate) struct PlayerWidgets {
    pub music_view: gtk4::Box,
    pub track_label: gtk4::Label,
    pub art_container: gtk4::Box,
}

impl PlayerWidgets {
    /// Builds the view hierarchy and starts the visualizer animation.
    /// Returns the widgets plus the shared play-state flag.
    pub(crate) fn build() -> (Self, Rc<Cell<bool>>) {
        let (visualizer_box, bars) = create_visualizer();
        let is_playing = Rc::new(Cell::new(false));

        let music_view = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        music_view.set_valign(gtk4::Align::Center);
        music_view.set_halign(gtk4::Align::Fill);
        music_view.set_hexpand(true);
        music_view.set_vexpand(true);

        let art_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        art_container.set_valign(gtk4::Align::Center);
        art_container.set_vexpand(true);

        let track_label = gtk4::Label::new(Some(&babydra_core::i18n::t("island.no_media")));
        track_label.add_css_class("notch-player-text");
        track_label.set_hexpand(true);
        track_label.set_vexpand(true);
        track_label.set_halign(gtk4::Align::Center);
        track_label.set_valign(gtk4::Align::Center);

        music_view.append(&art_container);
        music_view.append(&track_label);
        music_view.append(&visualizer_box);

        start_visualizer_animation(bars, is_playing.clone());

        (
            Self {
                music_view,
                track_label,
                art_container,
            },
            is_playing,
        )
    }
}
