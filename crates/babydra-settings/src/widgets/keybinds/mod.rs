pub mod render;

use babydra_core::models::shortcut::Shortcut;
use gtk4::prelude::*;
use gtk4::Widget;

/// Creates a new `keybinds widget`.
///
/// Shortcuts are loaded from `~/.config/babydra/keymap.toml` via the shared
/// core service; pressing *Save* writes the file back, which the
/// `babydra-keymap` daemon hot-reloads.
pub fn create_keybinds() -> Widget {
    let shortcuts = babydra_core::services::system::keymap::get_shortcuts();
    let widget = render::build(&shortcuts);

    // Add an empty editable row.
    let parent_card = widget.table_box.clone();
    widget.add_btn.connect_clicked(move |_| {
        let empty = Shortcut {
            id: 0,
            modifiers: String::new(),
            key: String::new(),
            command: String::new(),
        };
        let row = render::create_shortcut_row(&empty, parent_card.clone());
        parent_card.append(&row);
    });

    // Refresh from disk.
    let container_refresh = widget.container.clone();
    let table_card_refresh = widget.table_box.clone();
    widget.refresh_btn.connect_clicked(move |_| {
        rebuild_rows(&container_refresh, &table_card_refresh);
    });

    // Collect rows and persist to keymap.toml.
    let table_card_save = widget.table_box.clone();
    widget.save_btn.connect_clicked(move |_| {
        let mut shortcuts = Vec::new();
        let mut id = 1;
        for row in iter_data_rows(&table_card_save) {
            if let Some(command) = render::read_command(&row) {
                if let Some((modifiers, key)) = render::read_combo(&row) {
                    shortcuts.push(Shortcut {
                        id,
                        modifiers,
                        key,
                        command,
                    });
                    id += 1;
                }
            }
        }
        match babydra_core::services::system::keymap::save_shortcuts(&shortcuts) {
            Ok(_) => {
                let title = babydra_core::i18n::trans("settings.notif_keybinds_saved_title");
                let msg = babydra_core::i18n::trans("settings.notif_keybinds_saved_msg")
                    .replace("{}", &shortcuts.len().to_string());
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
    });

    widget.container.into()
}

/// Removes every data row and re-renders them from the config on disk.
fn rebuild_rows(container: &gtk4::Box, table_card: &gtk4::Box) {
    let mut current = table_card.first_child();
    while let Some(child) = current {
        current = child.next_sibling();
        if child.has_css_class(render::DATA_ROW_CSS_CLASS) {
            table_card.remove(&child);
        }
    }

    let shortcuts = babydra_core::services::system::keymap::get_shortcuts();
    for sc in &shortcuts {
        let row = render::create_shortcut_row(sc, table_card.clone());
        table_card.append(&row);
    }

    container.queue_draw();
}

/// Iterates over the data rows of the shortcuts table.
fn iter_data_rows(table_card: &gtk4::Box) -> Vec<gtk4::Box> {
    let mut rows = Vec::new();
    let mut current = table_card.first_child();
    while let Some(child) = current {
        current = child.next_sibling();
        if child.has_css_class(render::DATA_ROW_CSS_CLASS) {
            if let Ok(row) = child.downcast::<gtk4::Box>() {
                rows.push(row);
            }
        }
    }
    rows
}
