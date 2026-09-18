pub mod render;

use babydra_core::models::startup_command::StartupCommand;
use gtk4::prelude::*;
use gtk4::{Box, Entry, Widget};

/// Creates a new `startup widget`.
pub fn create_startup() -> Widget {
    let commands = babydra_core::services::system::startup::get_startup_commands();
    let widget = render::build(&commands);

    let list_card = widget.list_box.clone();
    let save_btn_for_add = widget.save_btn.clone();
    widget.add_btn.connect_clicked(move |_| {
        let (row, entry) = render::create_row("", &list_card, &save_btn_for_add);
        list_card.append(&row);
        entry.grab_focus();
    });

    let list_card_save = widget.list_box.clone();
    let save_btn_c = widget.save_btn.clone();
    widget.save_btn.connect_clicked(move |_| {
        let mut cmds = Vec::new();
        let mut id = 1;
        let mut row_child = list_card_save.first_child();
        while let Some(c) = row_child {
            if let Some(row_box) = c.downcast_ref::<Box>() {
                let mut item = row_box.first_child();
                while let Some(it) = item {
                    if let Some(entry) = it.downcast_ref::<Entry>() {
                        let text = entry.text().to_string();
                        let trimmed = text.trim();
                        if !trimmed.is_empty() {
                            cmds.push(StartupCommand {
                                id,
                                command: trimmed.to_string(),
                            });
                            id += 1;
                        }
                        break;
                    }
                    item = it.next_sibling();
                }
            }
            row_child = c.next_sibling();
        }

        match babydra_core::services::system::startup::save_startup_cmds(&cmds) {
            Ok(_) => {
                let title = babydra_core::i18n::trans("settings.notif_startup_saved_title");
                let msg = babydra_core::i18n::trans("settings.notif_startup_saved_msg")
                    .replace("{}", &cmds.len().to_string());
                babydra_core::send_settings_notif(&title, &msg);

                // Button visual feedback
                let orig_label = babydra_core::i18n::trans("settings.save_changes");
                let saved_label = format!("✓ {}", babydra_core::i18n::trans("settings.save"));
                save_btn_c.set_label(&saved_label);
                let btn_restore = save_btn_c.clone();
                glib::timeout_add_local_once(std::time::Duration::from_millis(1500), move || {
                    btn_restore.set_label(&orig_label);
                });
            }
            Err(e) => {
                let title = babydra_core::i18n::trans("settings.notif_startup_failed_title");
                let msg = format!(
                    "{}: {}",
                    babydra_core::i18n::trans("settings.notif_startup_failed_msg"),
                    e
                );
                babydra_core::send_settings_notif(&title, &msg);
            }
        }
    });

    widget.container.into()
}
