//! System crate shortcut row renderer and reader.

use crate::widgets::keybinds::dialog::format::set_combo_button;
use crate::widgets::keybinds::dialog::show_capture_dialog;
use babydra_core::models::shortcut::SystemShortcut;
use babydra_ui_kit::components::CustomSwitch;
use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, Window};

/// CSS class marking system crate shortcut rows.
pub const SYSTEM_ROW_CSS_CLASS: &str = "system-keybind-data-row";

/// Creates a dedicated system crate shortcut row:
/// [Title + Description] [Shortcut Combo Button] [CustomSwitch]
pub fn create_system_shortcut_row(sc: &SystemShortcut) -> Box {
    let row = Box::new(Orientation::Horizontal, 16);
    row.add_css_class("settings-card-row");
    row.add_css_class(SYSTEM_ROW_CSS_CLASS);
    row.set_widget_name(&sc.id);

    // Left info column: Title + Description (no tag/badge)
    let info_box = Box::new(Orientation::Vertical, 3);
    info_box.set_hexpand(true);
    info_box.set_valign(gtk4::Align::Center);

    let title_lbl = Label::new(Some(&babydra_core::i18n::trans(&sc.name_key)));
    title_lbl.add_css_class("settings-item-title");
    title_lbl.set_halign(gtk4::Align::Start);

    let desc_lbl = Label::new(Some(&babydra_core::i18n::trans(&sc.description_key)));
    desc_lbl.add_css_class("settings-item-description");
    desc_lbl.set_halign(gtk4::Align::Start);

    info_box.append(&title_lbl);
    info_box.append(&desc_lbl);

    // Controls: Shortcut Button + CustomSwitch (NO delete button)
    let ctrl_box = Box::new(Orientation::Horizontal, 12);
    ctrl_box.set_valign(gtk4::Align::Center);

    let combo_btn = Button::new();
    combo_btn.set_width_request(160);
    combo_btn.add_css_class("shortcut-combo-btn");
    combo_btn.set_halign(gtk4::Align::End);
    combo_btn.set_sensitive(sc.enabled);
    set_combo_button(&combo_btn, &sc.modifiers, &sc.key);

    {
        let combo_btn_c = combo_btn.clone();
        combo_btn_c.connect_clicked(move |btn| {
            let Some(parent_window) = btn.root().and_then(|root| root.downcast::<Window>().ok()) else {
                return;
            };
            show_capture_dialog(&parent_window, btn);
        });
    }

    let toggle_switch = CustomSwitch::new(sc.enabled);
    toggle_switch.container.set_valign(gtk4::Align::Center);

    {
        let combo_btn_toggle = combo_btn.clone();
        let row_toggle = row.clone();
        toggle_switch.connect_state_set(move |state| {
            combo_btn_toggle.set_sensitive(state);
            if state {
                row_toggle.remove_css_class("keybind-disabled");
            } else {
                row_toggle.add_css_class("keybind-disabled");
            }
        });
    }

    if !sc.enabled {
        row.add_css_class("keybind-disabled");
    }

    ctrl_box.append(&combo_btn);
    ctrl_box.append(&toggle_switch.container);

    row.append(&info_box);
    row.append(&ctrl_box);

    row
}

/// Reads a system shortcut row: returns `Some((id, modifiers, key, enabled))`.
pub fn read_system_row(row: &Box) -> Option<(String, String, String, bool)> {
    let id = row.widget_name().to_string();
    let children = row.observe_children();

    // Second child is ctrl_box [combo_btn, toggle_switch.container]
    let ctrl_box = children.item(1)?.downcast::<Box>().ok()?;
    let ctrl_children = ctrl_box.observe_children();

    let combo_btn = ctrl_children.item(0)?.downcast::<Button>().ok()?;

    let tooltip = combo_btn.tooltip_text()?;
    let combo = tooltip.to_string();
    let (modifiers, key) = match combo.rsplit_once('-') {
        Some((mods, key)) => (mods, key),
        None => ("", combo.as_str()),
    };

    let enabled = !row.has_css_class("keybind-disabled");
    Some((id, modifiers.to_string(), key.to_string(), enabled))
}
