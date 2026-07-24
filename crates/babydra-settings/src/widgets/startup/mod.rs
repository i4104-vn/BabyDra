pub mod render;

use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, Orientation, Widget};
use babydra_common::models::startup_command::StartupCommand;

pub fn create_startup_widget() -> Widget {
    let commands = babydra_common::services::system::startup::get_startup_commands();
    let widget = render::build(&commands);

    let list_card = widget.list_box.clone();
    widget.add_btn.connect_clicked(move |_| {
        let row = Box::new(Orientation::Horizontal, 12);
        row.add_css_class("settings-card-row");

        let badge = Label::new(Some("exec-once"));
        badge.add_css_class("connected-pill");

        let entry = Entry::new();
        entry.set_placeholder_text(Some("Enter command (e.g. waybar)"));
        entry.set_hexpand(true);
        entry.add_css_class("sidebar-search-entry");

        let delete_btn = Button::with_label("Remove");
        delete_btn.add_css_class("connect-pill-btn");

        let row_copy = row.clone();
        let list_card_copy = list_card.clone();
        delete_btn.connect_clicked(move |_| {
            list_card_copy.remove(&row_copy);
        });

        row.append(&badge);
        row.append(&entry);
        row.append(&delete_btn);
        list_card.append(&row);
    });

    let list_card_save = widget.list_box.clone();
    widget.save_btn.connect_clicked(move |_| {
        let mut cmds = Vec::new();
        let mut id = 1;
        let mut child = list_card_save.first_child();
        while let Some(c) = child {
            if let Some(row_box) = c.downcast_ref::<Box>() {
                if let Some(first_child) = row_box.first_child() {
                    let mut next = first_child.next_sibling();
                    while let Some(n) = next {
                        if let Some(entry) = n.downcast_ref::<Entry>() {
                            let text = entry.text().to_string();
                            if !text.trim().is_empty() {
                                cmds.push(StartupCommand { id, command: text });
                                id += 1;
                            }
                            break;
                        }
                        next = n.next_sibling();
                    }
                }
            }
            child = c.next_sibling();
        }
        let _ = babydra_common::services::system::startup::save_startup_commands(&cmds);
    });

    widget.container.into()
}
