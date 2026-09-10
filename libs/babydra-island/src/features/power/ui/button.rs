//! Power action button widget.

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, GestureClick, Label, Orientation};

#[derive(Clone)]
pub struct PowerButtonWidget {
    pub container: GtkBox,
    pub icon_holder: GtkBox,
    pub key_label: Label,
    pub title_label: Label,
    pub click_gesture: GestureClick,
}

impl PowerButtonWidget {
    pub fn new(
        icon_name: &str,
        icon_color: &str,
        key_char: &str,
        title: &str,
        css_action_class: &str,
    ) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 4);
        container.add_css_class("power-popover-btn");
        container.add_css_class(css_action_class);
        container.set_halign(Align::Center);
        container.set_valign(Align::Center);
        container.set_cursor_from_name(Some("pointer"));

        // Circular icon holder
        let icon_holder = GtkBox::new(Orientation::Horizontal, 0);
        icon_holder.add_css_class("power-btn-icon");
        icon_holder.set_halign(Align::Center);
        icon_holder.set_valign(Align::Center);
        let icon = babydra_ui_kit::ui::icon::get_icon_colored(icon_name, 22, icon_color);
        icon.set_halign(Align::Center);
        icon.set_valign(Align::Center);
        icon_holder.append(&icon);
        container.append(&icon_holder);

        // Key shortcut badge e.g. "1"
        let key_label = Label::new(Some(key_char));
        key_label.add_css_class("power-btn-key");
        key_label.set_halign(Align::Center);
        container.append(&key_label);

        // Title e.g. "Tắt máy"
        let title_label = Label::new(Some(title));
        title_label.add_css_class("power-btn-title");
        title_label.set_halign(Align::Center);
        title_label.set_wrap(false);
        container.append(&title_label);

        let click_gesture = GestureClick::new();
        container.add_controller(click_gesture.clone());

        Self {
            container,
            icon_holder,
            key_label,
            title_label,
            click_gesture,
        }
    }
}
