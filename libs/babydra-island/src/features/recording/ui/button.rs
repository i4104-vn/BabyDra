//! Interactive button widget for the recording popover controls.

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, CenterBox, GestureClick, Label, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct RecordingButtonWidget {
    pub container: GtkBox,
    pub icon_holder: CenterBox,
    pub title_label: Label,
    pub click_gesture: GestureClick,
    current_icon: Rc<RefCell<String>>,
}

impl RecordingButtonWidget {
    pub fn new(icon_name: &str, title: &str, css_class: &str) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 4);
        container.add_css_class("recording-popover-btn");
        if !css_class.is_empty() {
            container.add_css_class(css_class);
        }
        container.set_halign(Align::Center);
        container.set_valign(Align::Center);
        container.set_size_request(80, 72);
        container.set_cursor_from_name(Some("pointer"));

        let icon_holder = CenterBox::new();
        icon_holder.add_css_class("recording-btn-icon-holder");
        icon_holder.set_size_request(36, 36);
        icon_holder.set_halign(Align::Center);
        icon_holder.set_valign(Align::Center);

        let icon = babydra_ui_kit::ui::icon::get_icon(icon_name, 18);
        icon.set_halign(Align::Center);
        icon.set_valign(Align::Center);
        icon_holder.set_center_widget(Some(&icon));
        container.append(&icon_holder);

        let title_label = Label::new(Some(title));
        title_label.add_css_class("recording-btn-title");
        title_label.set_halign(Align::Center);
        title_label.set_valign(Align::Center);
        title_label.set_wrap(false);
        container.append(&title_label);

        let click_gesture = GestureClick::new();
        container.add_controller(click_gesture.clone());

        Self {
            container,
            icon_holder,
            title_label,
            click_gesture,
            current_icon: Rc::new(RefCell::new(icon_name.to_string())),
        }
    }

    pub fn set_icon_and_title(&self, icon_name: &str, title: &str) {
        self.title_label.set_text(title);

        let mut curr = self.current_icon.borrow_mut();
        if *curr != icon_name {
            *curr = icon_name.to_string();
            let icon = babydra_ui_kit::ui::icon::get_icon(icon_name, 18);
            icon.set_halign(Align::Center);
            icon.set_valign(Align::Center);
            self.icon_holder.set_center_widget(Some(&icon));
        }
    }

    pub fn set_alert(&self, is_alert: bool) {
        if is_alert {
            self.container.add_css_class("alert-active");
        } else {
            self.container.remove_css_class("alert-active");
        }
    }
}
