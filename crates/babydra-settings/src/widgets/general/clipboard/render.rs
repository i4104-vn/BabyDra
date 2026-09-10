//! Clipboard Settings card UI builder.

use babydra_core::i18n::trans;
use babydra_ui_kit::components::cards::create_collapsible_card;
use babydra_ui_kit::components::{create_list_row, CustomSwitch};
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button};

/// Holds widget references for the Clipboard card.
pub struct ClipboardWidgets {
    pub container: GtkBox,
    pub enabled_switch: CustomSwitch,
    pub shortcut_btn: Button,
}

/// Renders the Clipboard Settings collapsible card.
pub fn render_clipboard_card() -> ClipboardWidgets {
    let conf = babydra_core::config::load_babydra_config();

    let card = create_collapsible_card(
        &trans("settings.general_clipboard"),
        Some(&trans("settings.general_clipboard_desc")),
        Some("paste"),
        false,
    );

    // 1. Enable / Disable Toggle Switch
    let enabled_switch = CustomSwitch::new(conf.clipboard.enabled);
    enabled_switch.container.set_valign(Align::Center);
    let switch_row = create_list_row(
        "",
        &trans("settings.general_clipboard_enable"),
        &trans("settings.general_clipboard_enable_desc"),
        Some(&enabled_switch.container),
    );
    card.content.append(&switch_row);

    // 2. Shortcut Pill Button
    let shortcut_btn = Button::new();
    shortcut_btn.add_css_class("connect-pill-btn");
    shortcut_btn.set_cursor_from_name(Some("pointer"));
    shortcut_btn.set_valign(Align::Center);

    let combo_label = crate::widgets::keybinds::render::pretty_combo(
        &conf.clipboard.shortcut_modifiers,
        &conf.clipboard.shortcut_key,
    );
    shortcut_btn.set_label(if combo_label.trim().is_empty() {
        "Super + V"
    } else {
        &combo_label
    });
    shortcut_btn.set_sensitive(conf.clipboard.enabled);

    let shortcut_row = create_list_row(
        "",
        &trans("settings.general_clipboard_shortcut"),
        &trans("settings.general_clipboard_shortcut_desc"),
        Some(&shortcut_btn),
    );
    card.content.append(&shortcut_row);

    ClipboardWidgets {
        container: card.container,
        enabled_switch,
        shortcut_btn,
    }
}
