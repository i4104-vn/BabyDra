//! Interactive button widget for the recording popover controls.

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use babydra_ui_kit::components::create_icon_button;
use babydra_ui_kit::ui::icon::get_icon;
use gtk4::prelude::*;
use gtk4::Button;

#[derive(Clone)]
pub struct RecordingButtonWidget {
    pub button: Button,
    current_icon: Rc<RefCell<String>>,
}

impl RecordingButtonWidget {
    pub fn new(icon_name: &str, tooltip: &str, css_class: &str) -> Self {
        let mut classes = vec!["circular"];
        if !css_class.is_empty() {
            classes.push(css_class);
        }
        let button = create_icon_button(icon_name, 18, &classes, Some(tooltip), || {});
        button.set_cursor_from_name(Some("pointer"));

        Self {
            button,
            current_icon: Rc::new(RefCell::new(icon_name.to_string())),
        }
    }

    pub fn set_icon_and_title(&self, icon_name: &str, title: &str) {
        self.button.set_tooltip_text(Some(title));

        let mut curr = self.current_icon.borrow_mut();
        if *curr != icon_name {
            *curr = icon_name.to_string();
            let icon = get_icon(icon_name, 18);
            self.button.set_child(Some(&icon));
        }
    }

    pub fn set_alert(&self, is_alert: bool) {
        if is_alert {
            self.button.add_css_class("delete-btn");
        } else {
            self.button.remove_css_class("delete-btn");
        }
    }
}

impl Deref for RecordingButtonWidget {
    type Target = Button;

    fn deref(&self) -> &Self::Target {
        &self.button
    }
}
