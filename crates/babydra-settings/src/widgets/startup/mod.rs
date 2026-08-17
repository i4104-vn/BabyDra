pub mod render;

use babydra_core::models::startup_command::StartupCommand;
use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Orientation, Widget};

/// Creates a new `startup widget`.
pub fn create_startup_widget() -> Widget {
    let commands = babydra_core::services::system::startup::get_startup_commands();
    let widget = render::build(&commands);

    let list_card = widget.list_box.clone();
    widget.add_btn.connect_clicked(move |_| {
        let row = Box::new(Orientation::Horizontal, 12);
        row.add_css_class("settings-card-row");

        let entry = Entry::new();
        entry.set_placeholder_text(Some(&babydra_core::i18n::t(
            "settings.startup_command_placeholder",
        )));
        entry.set_hexpand(true);
        entry.add_css_class("sidebar-search-entry");

        let delete_btn = Button::new();
        delete_btn.add_css_class("icon-btn");
        delete_btn.add_css_class("circular");
        delete_btn.add_css_class("delete-btn");
        delete_btn.set_valign(gtk4::Align::Center);
        let del_icon = babydra_ui_kit::ui::icon::get_icon("edit-delete", 16);
        del_icon.set_pixel_size(16);
        delete_btn.set_child(Some(&del_icon));

        let row_copy = row.clone();
        let list_card_copy = list_card.clone();
        delete_btn.connect_clicked(move |_| {
            list_card_copy.remove(&row_copy);
        });

        row.append(&entry);
        row.append(&delete_btn);
        list_card.append(&row);
    });

    let list_card_save = widget.list_box.clone();
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
                        if !text.trim().is_empty() {
                            cmds.push(StartupCommand { id, command: text });
                            id += 1;
                        }
                        break;
                    }
                    item = it.next_sibling();
                }
            }
            row_child = c.next_sibling();
        }
        let _ = babydra_core::services::system::startup::save_startup_commands(&cmds);
    });

    widget.container.into()
}
