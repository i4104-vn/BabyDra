use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, Orientation};
use babydra_common::models::env_var::EnvVar;

pub struct EnvWidget {
    pub container: Box,
    pub list_box: Box,
    pub add_btn: Button,
    pub save_btn: Button,
}

pub fn build(vars: &[EnvVar]) -> EnvWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_margin_top(16);
    container.set_margin_bottom(16);
    container.set_margin_start(16);
    container.set_margin_end(16);

    // Header
    let header_box = Box::new(Orientation::Horizontal, 12);
    let title_label = Label::new(Some("Environment Variables"));
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

    let list_card = Box::new(Orientation::Vertical, 8);
    list_card.add_css_class("settings-card");

    for v in vars {
        let row = create_env_row(v, list_card.clone());
        list_card.append(&row);
    }

    container.append(&list_card);

    EnvWidget {
        container,
        list_box: list_card,
        add_btn,
        save_btn,
    }
}

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

    let sep = Label::new(Some(","));
    sep.add_css_class("settings-row-desc");

    let val_entry = Entry::new();
    val_entry.set_text(&v.value);
    val_entry.set_placeholder_text(Some("Value"));
    val_entry.set_hexpand(true);
    val_entry.add_css_class("sidebar-search-entry");

    let delete_btn = Button::with_label("Remove");
    delete_btn.add_css_class("connect-pill-btn");

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
