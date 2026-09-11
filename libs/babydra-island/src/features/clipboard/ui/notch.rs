//! Widget construction for the compact notch clipboard capsule view.

use gtk4::pango::EllipsizeMode;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, CenterBox, GestureClick, Label, Orientation};

#[derive(Clone)]
pub struct ClipboardNotchWidgets {
    pub notch_view: GtkBox,
    #[allow(dead_code)]
    pub title_label: Label,
    pub click_gesture: GestureClick,
}

impl ClipboardNotchWidgets {
    pub fn build() -> Self {
        let notch_view = GtkBox::new(Orientation::Horizontal, 0);
        notch_view.add_css_class("notch-content");
        notch_view.set_valign(Align::Center);
        notch_view.set_halign(Align::Fill);
        notch_view.set_hexpand(true);
        notch_view.set_vexpand(true);
        notch_view.set_focusable(false);
        notch_view.set_can_focus(false);

        let center_box = CenterBox::new();
        center_box.set_hexpand(true);
        center_box.set_valign(Align::Center);

        let icon = babydra_ui_kit::ui::icon::get_icon_colored("paste", 14, "#3b82f6");
        icon.set_valign(Align::Center);
        icon.set_halign(Align::Start);
        icon.set_margin_start(4);

        let title_label = Label::new(Some(&babydra_core::i18n::trans("island.clipboard_notch")));
        title_label.add_css_class("notch-player-text");
        title_label.set_valign(Align::Center);
        title_label.set_halign(Align::Center);
        title_label.set_xalign(0.5);
        title_label.set_ellipsize(EllipsizeMode::End);
        title_label.set_single_line_mode(true);

        center_box.set_start_widget(Some(&icon));
        center_box.set_center_widget(Some(&title_label));
        notch_view.append(&center_box);

        let click_gesture = GestureClick::new();

        Self {
            notch_view,
            title_label,
            click_gesture,
        }
    }
}
