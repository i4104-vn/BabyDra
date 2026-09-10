//! Notch capsule widget for desktop notifications.

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, GestureClick, Label, Orientation};

#[derive(Clone)]
pub struct NotificationNotchWidgets {
    pub container: GtkBox,
    pub icon_widget: gtk4::Image,
    pub title_label: Label,
    pub click_gesture: GestureClick,
}

impl NotificationNotchWidgets {
    pub fn build() -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 6);
        container.add_css_class("notch-content");
        container.set_valign(Align::Center);
        container.set_halign(Align::Center);
        container.set_focusable(false);
        container.set_can_focus(false);

        let icon_widget = babydra_ui_kit::ui::icon::get_icon_colored("bell", 14, "#f59e0b");
        icon_widget.set_valign(Align::Center);
        container.append(&icon_widget);

        let title_label = Label::new(Some(&babydra_core::i18n::trans("island.notification")));
        title_label.add_css_class("notch-player-text");
        title_label.set_valign(Align::Center);
        container.append(&title_label);

        let click_gesture = GestureClick::new();
        container.add_controller(click_gesture.clone());

        Self {
            container,
            icon_widget,
            title_label,
            click_gesture,
        }
    }
}
