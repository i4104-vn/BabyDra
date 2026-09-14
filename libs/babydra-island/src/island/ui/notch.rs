//! Shared notch capsule widget for Dynamic Island features.
//!
//! Provides a standardized layout across all island notch capsule views:
//! - Start (left): Feature icon (14px, colored, centered).
//! - Center: Feature title/text (ellipsize, single-line, centered).
//! - End (right, optional): Status/value text (e.g. "80%" or "Mute").

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::pango::EllipsizeMode;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, CenterBox, Label, Orientation};

/// Standardized notch capsule presentation widget.
#[derive(Clone)]
pub struct NotchWidget {
    pub container: GtkBox,
    pub icon_container: GtkBox,
    pub title_label: Label,
    pub value_label: Option<Label>,
    current_icon_name: Rc<RefCell<String>>,
}

impl NotchWidget {
    /// Builds a basic notch widget: icon on the left, title in the center.
    pub fn new(icon_name: &str, icon_color: &str, title: &str) -> Self {
        Self::create(icon_name, icon_color, title, None)
    }

    /// Builds an extended notch widget with an indicator value on the right (e.g. volume/brightness).
    pub fn with_value(icon_name: &str, icon_color: &str, title: &str, initial_value: &str) -> Self {
        Self::create(icon_name, icon_color, title, Some(initial_value))
    }

    /// Backwards-compatible alias for `with_value`.
    pub fn build(icon_name: &str, icon_color: &str, title: &str, initial_value: &str) -> Self {
        Self::with_value(icon_name, icon_color, title, initial_value)
    }

    fn create(icon_name: &str, icon_color: &str, title: &str, initial_value: Option<&str>) -> Self {
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

        center_box.set_start_widget(Some(&icon_container));
        center_box.set_center_widget(Some(&title_label));

        // 3. End widget: Optional value label
        let value_label = initial_value.map(|val| {
            let label = Label::new(Some(val));
            label.add_css_class("notch-system-value");
            label.set_valign(Align::Center);
            label.set_halign(Align::End);
            label.set_xalign(1.0);
            label.set_margin_end(4);
            center_box.set_end_widget(Some(&label));
            label
        });

        container.append(&center_box);

        Self {
            container,
            icon_container,
            title_label,
            value_label,
            current_icon_name: Rc::new(RefCell::new(icon_name.to_string())),
        }
    }

    /// Updates the displayed icon if it changed.
    pub fn set_icon(&self, icon_name: &str, icon_color: &str) {
        let mut curr = self.current_icon_name.borrow_mut();
        if *curr != icon_name {
            *curr = icon_name.to_string();
            while let Some(child) = self.icon_container.first_child() {
                self.icon_container.remove(&child);
            }
            let icon_widget = babydra_ui_kit::ui::icon::get_icon_colored(icon_name, 14, icon_color);
            icon_widget.set_valign(Align::Center);
            icon_widget.set_halign(Align::Start);
            self.icon_container.append(&icon_widget);
        }
    }

    /// Sets the title label text.
    pub fn set_title(&self, text: &str) {
        self.title_label.set_text(text);
    }

    /// Sets the value label text if a value label is present.
    pub fn set_value(&self, text: &str) {
        if let Some(ref label) = self.value_label {
            label.set_text(text);
        }
    }

    /// Updates both icon (if changed) and value label simultaneously.
    pub fn update(&self, icon_name: &str, icon_color: &str, value_text: &str) {
        self.set_icon(icon_name, icon_color);
        self.set_value(value_text);
    }

    /// Updates icon (if changed), title text, and value label simultaneously.
    pub fn update_all(
        &self,
        icon_name: &str,
        icon_color: &str,
        title_text: &str,
        value_text: &str,
    ) {
        self.set_icon(icon_name, icon_color);
        self.set_title(title_text);
        self.set_value(value_text);
    }
}
