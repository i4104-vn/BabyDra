//! Custom user shortcut row renderer and reader.

use crate::widgets::keybinds::dialog::format::{set_combo_button, MOD_LETTERS};
use crate::widgets::keybinds::dialog::show_capture_dialog;
use babydra_core::models::shortcut::Shortcut;
use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Orientation, Window};

/// CSS class marking editable custom data rows of the shortcuts table.
pub const DATA_ROW_CSS_CLASS: &str = "keybind-data-row";

/// Creates an editable custom row: [Shortcut Button] [Command Entry (expands)] [Delete Button].
pub fn create_custom_shortcut_row(sc: &Shortcut, parent: Box) -> Box {
    let row = Box::new(Orientation::Horizontal, 12);
    row.add_css_class("settings-card-row");
    row.add_css_class(DATA_ROW_CSS_CLASS);

    let combo_btn = Button::new();
    combo_btn.set_width_request(160);
    combo_btn.add_css_class("shortcut-combo-btn");
    combo_btn.set_halign(gtk4::Align::Start);
    set_combo_button(&combo_btn, &sc.modifiers, &sc.key);
    {
        combo_btn.connect_clicked(move |btn| {
            let Some(parent_window) = btn.root().and_then(|root| root.downcast::<Window>().ok()) else {
                return;
            };
            show_capture_dialog(&parent_window, btn);
        });
    }

    let cmd_entry = Entry::new();
    cmd_entry.set_text(&sc.command);
    cmd_entry.set_hexpand(true);
    cmd_entry.set_placeholder_text(Some("~/.local/bin/my-script, kitty, opera..."));
    cmd_entry.add_css_class("sidebar-search-entry");

    let delete_btn = Button::new();
    delete_btn.add_css_class("icon-btn");
    delete_btn.add_css_class("circular");
    delete_btn.add_css_class("delete-btn");
    delete_btn.set_valign(gtk4::Align::Center);
    let del_icon = babydra_ui_kit::ui::icon::get_icon("edit-delete", 16);
    del_icon.set_pixel_size(16);
    delete_btn.set_child(Some(&del_icon));

    let row_copy = row.clone();
    delete_btn.connect_clicked(move |_| {
        parent.remove(&row_copy);
    });

    row.append(&combo_btn);
    row.append(&cmd_entry);
    row.append(&delete_btn);

    row
}

/// Extracts `(modifiers, key)` from a data row created by [`create_custom_shortcut_row`].
pub fn read_combo(row: &Box) -> Option<(String, String)> {
    let children = row.observe_children();
    let combo_btn = children.item(0)?.downcast::<Button>().ok()?;
    let tooltip = combo_btn.tooltip_text()?;
    let combo = tooltip.to_string();
    let (modifiers, key) = match combo.rsplit_once('-') {
        Some((mods, key)) => (mods, key),
        None => ("", combo.as_str()),
    };

    let valid = modifiers
        .split('-')
        .filter(|m| !m.is_empty())
        .all(|m| MOD_LETTERS.contains(&m));
    if !valid || key.is_empty() {
        return None;
    }

    Some((modifiers.to_string(), key.to_string()))
}

/// Extracts the shell command from a data row created by [`create_custom_shortcut_row`].
pub fn read_command(row: &Box) -> Option<String> {
    let children = row.observe_children();
    let cmd_entry = children.item(1)?.downcast::<Entry>().ok()?;
    let command = cmd_entry.text().trim().to_string();
    if command.is_empty() {
        None
    } else {
        Some(command)
    }
}
