//! Notch widget for the power island feature.

use gtk4::pango::EllipsizeMode;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, CenterBox, GestureClick, Label, Orientation};

#[derive(Clone)]
pub struct PowerNotchWidgets {
    pub container: GtkBox,
    pub title_label: Label,
    pub click_gesture: GestureClick,
}

impl PowerNotchWidgets {
    pub fn build() -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 0);
        container.add_css_class("notch-content");
        container.set_valign(Align::Center);
        container.set_halign(Align::Fill);
        container.set_hexpand(true);
        container.set_vexpand(true);

        let center_box = CenterBox::new();
        center_box.set_hexpand(true);
        center_box.set_valign(Align::Center);

        let icon = babydra_ui_kit::ui::icon::get_icon_colored("power", 14, "#ff5555");
        icon.set_valign(Align::Center);
        icon.set_halign(Align::Start);
        icon.set_margin_start(4);

        let title_label = Label::new(Some(&babydra_core::i18n::trans("island.power_notch")));
        title_label.add_css_class("notch-player-text");
        title_label.set_valign(Align::Center);
        title_label.set_halign(Align::Center);
        title_label.set_xalign(0.5);
        title_label.set_ellipsize(EllipsizeMode::End);
        title_label.set_single_line_mode(true);

        center_box.set_start_widget(Some(&icon));
        center_box.set_center_widget(Some(&title_label));
        container.append(&center_box);

        let click_gesture = GestureClick::new();

        Self {
            container,
            title_label,
            click_gesture,
        }
    }
}
