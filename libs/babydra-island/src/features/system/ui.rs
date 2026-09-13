//! Shared notch capsule widget for system indicators (volume, brightness, etc.).
//!
//! Layout mirrors other island features:
//! - Start (left): Feature icon (adaptive status).
//! - Center: Feature name (translated, centered).
//! - End (right): Percentage or status value (e.g. "80%" or "Mute"), matching player's visualizer slot.

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::pango::EllipsizeMode;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, CenterBox, Label, Orientation};

#[derive(Clone)]
pub struct SystemIndicatorWidget {
    pub container: GtkBox,
    pub icon_container: GtkBox,
    pub title_label: Label,
    pub value_label: Label,
    current_icon_name: Rc<RefCell<String>>,
}

impl SystemIndicatorWidget {
    /// Builds a new system indicator widget with initial icon, color, title, and value.
    pub fn build(icon_name: &str, icon_color: &str, title: &str, initial_value: &str) -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 0);
        container.add_css_class("notch-content");
        container.set_valign(Align::Center);
        container.set_halign(Align::Fill);
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_focusable(false);
        container.set_can_focus(false);

        let center_box = CenterBox::new();
        center_box.set_hexpand(true);
        center_box.set_valign(Align::Center);

        // 1. Start widget: Icon container
        let icon_container = GtkBox::new(Orientation::Horizontal, 0);
        icon_container.set_valign(Align::Center);
        icon_container.set_halign(Align::Start);
        icon_container.set_margin_start(4);

        let icon_widget = babydra_ui_kit::ui::icon::get_icon_colored(icon_name, 14, icon_color);
        icon_widget.set_valign(Align::Center);
        icon_widget.set_halign(Align::Start);
        icon_container.append(&icon_widget);

        // 2. Center widget: Title label
        let title_label = Label::new(Some(title));
        title_label.add_css_class("notch-player-text");
        title_label.set_valign(Align::Center);
        title_label.set_halign(Align::Center);
        title_label.set_xalign(0.5);
        title_label.set_ellipsize(EllipsizeMode::End);
        title_label.set_single_line_mode(true);

        // 3. End widget: Value label (e.g. "80%" or "Mute")
        let value_label = Label::new(Some(initial_value));
        value_label.add_css_class("notch-system-value");
        value_label.set_valign(Align::Center);
        value_label.set_halign(Align::End);
        value_label.set_xalign(1.0);
        value_label.set_margin_end(4);

        center_box.set_start_widget(Some(&icon_container));
        center_box.set_center_widget(Some(&title_label));
        center_box.set_end_widget(Some(&value_label));
        container.append(&center_box);

        Self {
            container,
            icon_container,
            title_label,
            value_label,
            current_icon_name: Rc::new(RefCell::new(icon_name.to_string())),
        }
    }

    /// Updates the displayed icon (if changed) and value label.
    pub fn update(&self, icon_name: &str, icon_color: &str, value_text: &str) {
        let mut curr = self.current_icon_name.borrow_mut();
        if *curr != icon_name {
            *curr = icon_name.to_string();
            while let Some(child) = self.icon_container.first_child() {
                self.icon_container.remove(&child);
            }
            let icon_widget =
                babydra_ui_kit::ui::icon::get_icon_colored(icon_name, 14, icon_color);
            icon_widget.set_valign(Align::Center);
            icon_widget.set_halign(Align::Start);
            self.icon_container.append(&icon_widget);
        }
        self.value_label.set_text(value_text);
    }
}
