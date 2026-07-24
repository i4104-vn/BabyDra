use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, Orientation};
use babydra_common::models::startup_command::StartupCommand;

pub struct StartupWidget {
    pub container: Box,
    pub list_box: Box,
    pub add_btn: Button,
    pub save_btn: Button,
    pub entries: Vec<Entry>,
}

pub fn build(commands: &[StartupCommand]) -> StartupWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_margin_top(16);
    container.set_margin_bottom(16);
    container.set_margin_start(16);
    container.set_margin_end(16);

    // Header
    let header_box = Box::new(Orientation::Horizontal, 12);
    let title_label = Label::new(Some("Startup Applications"));
    title_label.add_css_class("settings-title-label");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);

    let add_btn = Button::with_label("+ Add New");
    add_btn.add_css_class("connect-pill-btn");

    let save_btn = Button::with_label("Save Changes");
    save_btn.add_css_class("connect-pill-btn");

    header_box.append(&title_label);
    header_box.append(&add_btn);
    header_box.append(&save_btn);
    container.append(&header_box);

    // List container card
    let list_card = Box::new(Orientation::Vertical, 8);
    list_card.add_css_class("settings-card");

    let mut entries = Vec::new();

    for cmd in commands {
        let row = Box::new(Orientation::Horizontal, 12);
        row.add_css_class("settings-card-row");

        let badge = Label::new(Some("exec-once"));
        badge.add_css_class("connected-pill");

        let entry = Entry::new();
        entry.set_text(&cmd.command);
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

        entries.push(entry);
    }

    container.append(&list_card);

    StartupWidget {
        container,
        list_box: list_card,
        add_btn,
        save_btn,
        entries,
    }
}
