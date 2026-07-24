use gtk4::prelude::*;
use gtk4::{Box, Button, DropDown, Entry, Label, Orientation, StringList};
use babydra_common::models::keybind::Keybind;

pub struct KeybindsWidget {
    pub container: Box,
    pub table_box: Box,
    pub add_btn: Button,
    pub refresh_btn: Button,
    pub save_btn: Button,
}

pub fn build(keybinds: &[Keybind]) -> KeybindsWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_margin_top(16);
    container.set_margin_bottom(16);
    container.set_margin_start(16);
    container.set_margin_end(16);

    // Header
    let header_box = Box::new(Orientation::Horizontal, 12);
    let title_label = Label::new(Some("Keybinds"));
    title_label.add_css_class("settings-title-label");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);

    let refresh_btn = Button::with_label("Refresh");
    refresh_btn.add_css_class("connect-pill-btn");

    let add_btn = Button::with_label("+ Add New");
    add_btn.add_css_class("connect-pill-btn");

    let save_btn = Button::with_label("Save Changes");
    save_btn.add_css_class("connect-pill-btn");

    header_box.append(&title_label);
    header_box.append(&refresh_btn);
    header_box.append(&add_btn);
    header_box.append(&save_btn);
    container.append(&header_box);

    // Table Container Card
    let table_card = Box::new(Orientation::Vertical, 4);
    table_card.add_css_class("settings-card");

    // Table Header Row
    let th_row = Box::new(Orientation::Horizontal, 12);
    th_row.add_css_class("settings-card-row");
    th_row.set_margin_bottom(8);

    let col_type = Label::new(Some("TYPE"));
    col_type.set_width_request(100);
    col_type.add_css_class("settings-section-title");

    let col_mod = Label::new(Some("MODIFIER"));
    col_mod.set_width_request(120);
    col_mod.add_css_class("settings-section-title");

    let col_key = Label::new(Some("KEY"));
    col_key.set_width_request(80);
    col_key.add_css_class("settings-section-title");

    let col_disp = Label::new(Some("DISPATCHER"));
    col_disp.set_hexpand(true);
    col_disp.add_css_class("settings-section-title");

    let col_args = Label::new(Some("ARGS"));
    col_args.set_hexpand(true);
    col_args.add_css_class("settings-section-title");

    let col_act = Label::new(Some(""));
    col_act.set_width_request(60);

    th_row.append(&col_type);
    th_row.append(&col_mod);
    th_row.append(&col_key);
    th_row.append(&col_disp);
    th_row.append(&col_args);
    th_row.append(&col_act);
    table_card.append(&th_row);

    let types = vec!["bind", "binde", "bindm", "bindl"];
    let mods = vec!["SUPER", "ALT", "CTRL", "SHIFT", "SUPER_SHIFT", "ALT_SHIFT"];

    for kb in keybinds {
        let row = create_keybind_row(kb, &types, &mods, table_card.clone());
        table_card.append(&row);
    }

    container.append(&table_card);

    KeybindsWidget {
        container,
        table_box: table_card,
        add_btn,
        refresh_btn,
        save_btn,
    }
}

pub fn create_keybind_row(
    kb: &Keybind,
    types: &[&str],
    mods: &[&str],
    parent: Box,
) -> Box {
    let row = Box::new(Orientation::Horizontal, 8);
    row.add_css_class("settings-card-row");

    let type_model = StringList::new(types);
    let type_dropdown = DropDown::new(Some(type_model), Option::<gtk4::Expression>::None);
    type_dropdown.set_width_request(100);

    let mod_model = StringList::new(mods);
    let mod_dropdown = DropDown::new(Some(mod_model), Option::<gtk4::Expression>::None);
    mod_dropdown.set_width_request(120);

    let key_entry = Entry::new();
    key_entry.set_text(&kb.key);
    key_entry.set_width_request(80);
    key_entry.add_css_class("sidebar-search-entry");

    let disp_entry = Entry::new();
    disp_entry.set_text(&kb.dispatcher);
    disp_entry.set_hexpand(true);
    disp_entry.add_css_class("sidebar-search-entry");

    let args_entry = Entry::new();
    args_entry.set_text(&kb.args);
    args_entry.set_hexpand(true);
    args_entry.add_css_class("sidebar-search-entry");

    let delete_btn = Button::with_label("X");
    delete_btn.add_css_class("connect-pill-btn");
    delete_btn.set_width_request(40);

    let row_copy = row.clone();
    delete_btn.connect_clicked(move |_| {
        parent.remove(&row_copy);
    });

    row.append(&type_dropdown);
    row.append(&mod_dropdown);
    row.append(&key_entry);
    row.append(&disp_entry);
    row.append(&args_entry);
    row.append(&delete_btn);

    row
}
