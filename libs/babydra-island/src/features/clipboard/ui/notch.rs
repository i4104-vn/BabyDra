//! Widget construction for the compact notch clipboard capsule view.

use gtk4::pango::EllipsizeMode;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, CenterBox, Label, Orientation};

#[derive(Clone)]
pub struct ClipboardNotchWidgets {
    pub notch_view: CenterBox,
    #[allow(dead_code)]
    pub title_label: Label,
}

impl ClipboardNotchWidgets {
    pub fn build() -> Self {
        let notch_view = CenterBox::new();
        notch_view.add_css_class("notch-content");
        notch_view.set_valign(Align::Center);
        notch_view.set_halign(Align::Fill);
        notch_view.set_hexpand(true);
        notch_view.set_vexpand(true);

        let icon = babydra_ui_kit::ui::icon::get_icon_colored("paste", 13, "#3b82f6");
        icon.set_valign(Align::Center);
        icon.set_halign(Align::Start);
        notch_view.set_start_widget(Some(&icon));

        let title_label = Label::new(Some(&babydra_core::i18n::trans("island.clipboard")));
        title_label.add_css_class("notch-player-text");
        title_label.set_valign(Align::Center);
        title_label.set_halign(Align::Center);
        title_label.set_ellipsize(EllipsizeMode::End);
        title_label.set_single_line_mode(true);
        notch_view.set_center_widget(Some(&title_label));

        let dummy = GtkBox::new(Orientation::Horizontal, 0);
        dummy.set_size_request(13, 13);
        notch_view.set_end_widget(Some(&dummy));

        Self {
            notch_view,
            title_label,
        }
    }
}
