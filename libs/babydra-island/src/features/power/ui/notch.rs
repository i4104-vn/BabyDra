//! Notch widget for the power island feature.

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, GestureClick, Label, Orientation};

#[derive(Clone)]
pub struct PowerNotchWidgets {
    pub container: GtkBox,
    pub title_label: Label,
    pub click_gesture: GestureClick,
}

impl PowerNotchWidgets {
    pub fn build() -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 6);
        container.add_css_class("notch-content");
        container.set_valign(Align::Center);
        container.set_halign(Align::Center);

        let icon = babydra_ui_kit::ui::icon::get_icon_colored("power", 14, "#ff5555");
        icon.set_valign(Align::Center);
        container.append(&icon);

        let title_label = Label::new(Some(&babydra_core::i18n::trans("island.power_notch")));
        title_label.add_css_class("notch-player-text");
        title_label.set_valign(Align::Center);
        container.append(&title_label);

        let click_gesture = GestureClick::new();
        container.add_controller(click_gesture.clone());

        Self {
            container,
            title_label,
            click_gesture,
        }
    }
}
