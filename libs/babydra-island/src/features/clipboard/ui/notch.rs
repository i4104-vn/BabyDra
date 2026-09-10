//! Widget construction for the compact notch clipboard capsule view.

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Label, Orientation};

#[derive(Clone)]
pub struct ClipboardNotchWidgets {
    pub notch_view: GtkBox,
    #[allow(dead_code)]
    pub title_label: Label,
}

impl ClipboardNotchWidgets {
    pub fn build() -> Self {
        let notch_view = GtkBox::new(Orientation::Horizontal, 6);
        notch_view.set_valign(Align::Center);
        notch_view.set_halign(Align::Fill);
        notch_view.set_hexpand(true);
        notch_view.set_vexpand(true);

        let icon = babydra_ui_kit::ui::icon::get_icon_colored("paste", 13, "#3b82f6");
        icon.set_valign(Align::Center);

        let title_label = Label::new(Some(&babydra_core::i18n::trans("island.clipboard")));
        title_label.add_css_class("notch-player-text");
        title_label.set_valign(Align::Center);
        title_label.set_halign(Align::Center);
        title_label.set_hexpand(true);

        notch_view.append(&icon);
        notch_view.append(&title_label);

        Self {
            notch_view,
            title_label,
        }
    }
}
