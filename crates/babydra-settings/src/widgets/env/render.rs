use crate::widgets::state::EnvWidget;
use babydra_core::models::env_var::EnvVar;
use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, Orientation};

/// Build.
pub fn build(vars: &[EnvVar]) -> EnvWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // Header
    let header_box = Box::new(Orientation::Horizontal, 12);
    let title_box = Box::new(Orientation::Vertical, 4);
    title_box.set_hexpand(true);
    title_box.set_halign(gtk4::Align::Start);

    let title_label = Label::new(Some(&babydra_core::i18n::trans("settings.env_title")));
    title_label.add_css_class("settings-page-title");
    title_label.set_halign(gtk4::Align::Start);

    let subtitle_label = Label::new(Some(&babydra_core::i18n::trans("settings.env_subtitle")));
    subtitle_label.add_css_class("settings-page-subtitle");
    subtitle_label.set_halign(gtk4::Align::Start);

    title_box.append(&title_label);
    title_box.append(&subtitle_label);

    let add_btn = Button::with_label(&babydra_core::i18n::trans("settings.startup_add_new"));
    add_btn.add_css_class("connect-pill-btn");

    let save_btn = Button::with_label(&babydra_core::i18n::trans("settings.save_changes"));
    save_btn.add_css_class("suggested-action");

    header_box.append(&title_box);
    header_box.append(&add_btn);
    header_box.append(&save_btn);
    container.append(&header_box);

    let glass_card = Box::new(Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");
    glass_card.set_vexpand(true);
    glass_card.set_valign(gtk4::Align::Fill);

    let list_card = Box::new(Orientation::Vertical, 8);

    for v in vars {
        let row = create_env_row(v, list_card.clone());
        list_card.append(&row);
    }

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&list_card));

    glass_card.append(&scroll);
    container.append(&glass_card);

    EnvWidget {
        container,
        list_box: list_card,
        add_btn,
        save_btn,
    }
}

/// Creates a new `env row`.
pub fn create_env_row(v: &EnvVar, parent: Box) -> Box {
    let row = Box::new(Orientation::Horizontal, 12);
    row.add_css_class("settings-card-row");

    let prefix = Label::new(Some("env ="));
    prefix.add_css_class("settings-row-desc");

    let key_entry = Entry::new();
    key_entry.set_text(&v.key);
    key_entry.set_placeholder_text(Some("VARIABLE"));
    key_entry.set_width_request(180);
    key_entry.add_css_class("sidebar-search-entry");

    let sep = Label::new(Some("="));
    sep.add_css_class("settings-row-desc");

    let val_entry = Entry::new();
    val_entry.set_text(&v.value);
    val_entry.set_placeholder_text(Some("Value"));
    val_entry.set_hexpand(true);
    val_entry.add_css_class("sidebar-search-entry");

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

    row.append(&prefix);
    row.append(&key_entry);
    row.append(&sep);
    row.append(&val_entry);
    row.append(&delete_btn);

    row
}
