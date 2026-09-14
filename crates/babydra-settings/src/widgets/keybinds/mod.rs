//! Global keybindings settings page module.

pub mod dialog;
pub mod handler;
pub mod render;
pub mod rows;
pub mod sections;

use babydra_core::models::shortcut::Shortcut;
use gtk4::prelude::*;
use gtk4::Widget;
use rows::create_custom_shortcut_row;

/// Creates the redesigned keybinds widget with dedicated BabyDra Crates system area and Custom area.
pub fn create_keybinds() -> Widget {
    let system_shortcuts = babydra_core::services::system::keymap::get_system_shortcuts();
    let custom_shortcuts = babydra_core::services::system::keymap::get_custom_shortcuts();
    let widget = render::build(&system_shortcuts, &custom_shortcuts);

    // Add empty row in Custom Shortcuts table
    let custom_table_clone = widget.custom_table_box.clone();
    widget.add_btn.connect_clicked(move |_| {
        let empty = Shortcut {
            id: 0,
            modifiers: String::new(),
            key: String::new(),
            command: String::new(),
            enabled: true,
        };
        let row = create_custom_shortcut_row(&empty, custom_table_clone.clone());
        custom_table_clone.append(&row);
    });

    // Refresh from disk
    let sys_table_refresh = widget.system_table_box.clone();
    let custom_table_refresh = widget.custom_table_box.clone();
    widget.refresh_btn.connect_clicked(move |_| {
        handler::rebuild_rows(&sys_table_refresh, &custom_table_refresh);
    });

    // Save changes for both system crate shortcuts and custom shortcuts
    let sys_table_save = widget.system_table_box.clone();
    let custom_table_save = widget.custom_table_box.clone();
    widget.save_btn.connect_clicked(move |_| {
        handler::handle_save(&sys_table_save, &custom_table_save);
    });

    widget.container.into()
}
