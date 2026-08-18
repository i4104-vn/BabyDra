use crate::widgets::state::StartupWidget;
use babydra_core::models::startup_command::StartupCommand;
use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, Orientation};

/// Build.
pub fn build(commands: &[StartupCommand]) -> StartupWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // Header
    let header_box = Box::new(Orientation::Horizontal, 12);
    let title_label = Label::new(Some(&babydra_core::i18n::trans("settings.startup_title")));
    title_label.add_css_class("settings-page-title");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);

    let add_btn = Button::with_label(&babydra_core::i18n::trans("settings.startup_add_new"));
    add_btn.add_css_class("connect-pill-btn");

    let save_btn = Button::with_label(&babydra_core::i18n::trans("settings.save_changes"));
    save_btn.add_css_class("suggested-action");

    header_box.append(&title_label);
    header_box.append(&add_btn);
    header_box.append(&save_btn);
    container.append(&header_box);

    // List container glass card
    let glass_card = Box::new(Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");
    glass_card.set_vexpand(true);
    glass_card.set_valign(gtk4::Align::Fill);

    let list_card = Box::new(Orientation::Vertical, 8);

    let mut entries = Vec::new();

    for cmd in commands {
        let row = Box::new(Orientation::Horizontal, 12);
        row.add_css_class("settings-card-row");

        let entry = Entry::new();
        entry.set_text(&cmd.command);
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

        entries.push(entry);
    }

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&list_card));

    glass_card.append(&scroll);
    container.append(&glass_card);

    StartupWidget {
        container,
        list_box: list_card,
        add_btn,
        save_btn,
        entries,
    }
}
