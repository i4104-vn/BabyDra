//! Power action button widget.

use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, CenterBox, EventControllerMotion, GestureClick, Label, Orientation,
};

#[derive(Clone)]
pub struct PowerButtonWidget {
    pub container: GtkBox,
    pub icon_holder: CenterBox,
    pub title_label: Label,
    pub key_label: Label,
    pub click_gesture: GestureClick,
    pub motion_controller: EventControllerMotion,
}

impl PowerButtonWidget {
    pub fn new(
        icon_name: &str,
        icon_color: &str,
        key_char: &str,
        title: &str,
        css_action_class: &str,
    ) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 0);
        container.add_css_class("power-popover-btn");
        container.add_css_class(css_action_class);
        container.set_halign(Align::Center);
        container.set_valign(Align::Center);
        container.set_size_request(100, 100);
        container.set_focusable(false);
        container.set_cursor_from_name(Some("pointer"));

        // 1. Dead-centered circular icon holder via CenterBox
        let icon_holder = CenterBox::new();
        icon_holder.add_css_class("power-btn-icon");
        icon_holder.set_size_request(40, 40);
        icon_holder.set_halign(Align::Center);
        icon_holder.set_valign(Align::Center);
        icon_holder.set_margin_bottom(5);

        let icon = babydra_ui_kit::ui::icon::get_icon_colored(icon_name, 20, icon_color);
        icon.set_halign(Align::Center);
        icon.set_valign(Align::Center);
        icon_holder.set_center_widget(Some(&icon));
        container.append(&icon_holder);

        // 2. Action title e.g. "Tắt máy"
        let title_label = Label::new(Some(title));
        title_label.add_css_class("power-btn-title");
        title_label.set_halign(Align::Center);
        title_label.set_valign(Align::Center);
        title_label.set_wrap(false);
        title_label.set_margin_bottom(5);
        container.append(&title_label);

        // 3. Hotkey keycap badge e.g. "1"
        let key_label = Label::new(Some(key_char));
        key_label.add_css_class("power-btn-key");
        key_label.set_halign(Align::Center);
        key_label.set_valign(Align::Center);
        container.append(&key_label);

        let click_gesture = GestureClick::new();
        container.add_controller(click_gesture.clone());

        let motion_controller = EventControllerMotion::new();
        container.add_controller(motion_controller.clone());

        Self {
            container,
            icon_holder,
            title_label,
            key_label,
            click_gesture,
            motion_controller,
        }
    }
}
