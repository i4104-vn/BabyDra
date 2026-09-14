//! Event handlers and data synchronization for keybindings.

use super::rows::{
    create_custom_shortcut_row, create_system_shortcut_row, read_combo, read_command,
    read_system_row, DATA_ROW_CSS_CLASS, SYSTEM_ROW_CSS_CLASS,
};
use babydra_core::models::shortcut::Shortcut;
use gtk4::prelude::*;
use gtk4::Box;

/// Saves all changes from both the system crate table and the custom shortcuts table.
pub fn handle_save(sys_table: &Box, custom_table: &Box) {
    let mut original_system = babydra_core::services::system::keymap::get_system_shortcuts();

    // Read updated system crate shortcuts
    for row in iter_rows(sys_table, SYSTEM_ROW_CSS_CLASS) {
        if let Some((id, modifiers, key, enabled)) = read_system_row(&row) {
            if let Some(s) = original_system.iter_mut().find(|s| s.id == id) {
                s.modifiers = modifiers;
                s.key = key;
                s.enabled = enabled;
            }
        }
    }

    // Read custom shortcuts
    let mut custom_shortcuts = Vec::new();
    let mut id = 1;
    for row in iter_rows(custom_table, DATA_ROW_CSS_CLASS) {
        if let Some(command) = read_command(&row) {
            if let Some((modifiers, key)) = read_combo(&row) {
                custom_shortcuts.push(Shortcut {
                    id,
                    modifiers,
                    key,
                    command,
                    enabled: true,
                });
                id += 1;
            }
        }
    }

    match babydra_core::services::system::keymap::save_keymap_configuration(
        &original_system,
        &custom_shortcuts,
    ) {
        Ok(_) => {
            let title = babydra_core::i18n::trans("settings.notif_keybinds_saved_title");
            let active_count =
                original_system.iter().filter(|s| s.enabled).count() + custom_shortcuts.len();
            let msg = babydra_core::i18n::trans("settings.notif_keybinds_saved_msg")
                .replace("{}", &active_count.to_string());
            babydra_core::send_settings_notif(&title, &msg);
        }
        Err(e) => {
            let title = babydra_core::i18n::trans("settings.notif_keybinds_failed_title");
            let msg = format!(
                "{}: {}",
                babydra_core::i18n::trans("settings.notif_keybinds_failed_msg"),
                e
            );
            babydra_core::send_settings_notif(&title, &msg);
        }
    }
}

/// Removes all rows in both tables and re-renders from config on disk.
pub fn rebuild_rows(sys_table: &Box, custom_table: &Box) {
    clear_children(sys_table);
    clear_children(custom_table);

    let system_shortcuts = babydra_core::services::system::keymap::get_system_shortcuts();
    for sc in &system_shortcuts {
        let row = create_system_shortcut_row(sc);
        sys_table.append(&row);
    }

    let custom_shortcuts = babydra_core::services::system::keymap::get_custom_shortcuts();
    for sc in &custom_shortcuts {
        let row = create_custom_shortcut_row(sc, custom_table.clone());
        custom_table.append(&row);
    }
}

/// Removes all children from a Gtk Box.
pub fn clear_children(table: &Box) {
    let mut current = table.first_child();
    while let Some(child) = current {
        current = child.next_sibling();
        table.remove(&child);
    }
}

/// Iterates child boxes that match a specific CSS class.
pub fn iter_rows(table: &Box, css_class: &str) -> Vec<Box> {
    let mut rows = Vec::new();
    let mut current = table.first_child();
    while let Some(child) = current {
        current = child.next_sibling();
        if child.has_css_class(css_class) {
            if let Ok(row) = child.downcast::<Box>() {
                rows.push(row);
            }
        }
    }
    rows
}
