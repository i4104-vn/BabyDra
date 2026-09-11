//! Notch widget for the power island feature.

use gtk4::pango::EllipsizeMode;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, CenterBox, GestureClick, Label, Orientation};

#[derive(Clone)]
pub struct PowerNotchWidgets {
    pub container: CenterBox,
    pub title_label: Label,
    pub click_gesture: GestureClick,
}

impl PowerNotchWidgets {
    pub fn build() -> Self {
        let container = CenterBox::new();
        container.add_css_class("notch-content");
        container.set_valign(Align::Center);
        container.set_halign(Align::Fill);
        container.set_hexpand(true);
        container.set_vexpand(true);

        let icon = babydra_ui_kit::ui::icon::get_icon_colored("power", 14, "#ff5555");
        icon.set_valign(Align::Center);
        icon.set_halign(Align::Start);
        container.set_start_widget(Some(&icon));

        let title_label = Label::new(Some(&babydra_core::i18n::trans("island.power_notch")));
        title_label.add_css_class("notch-player-text");
        title_label.set_valign(Align::Center);
        title_label.set_halign(Align::Center);
        title_label.set_ellipsize(EllipsizeMode::End);
        title_label.set_single_line_mode(true);
        container.set_center_widget(Some(&title_label));

        let dummy = GtkBox::new(Orientation::Horizontal, 0);
        dummy.set_size_request(14, 14);
        container.set_end_widget(Some(&dummy));

        let click_gesture = GestureClick::new();
        container.add_controller(click_gesture.clone());

        Self {
            container,
            title_label,
            click_gesture,
        }
    }
}
