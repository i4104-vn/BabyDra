use gtk4::prelude::*;
use gtk4::{Box, Button, DropDown, Entry, Label, Orientation, StringList};
use babydra_common::models::keybind::{Keybind, KeybindsWidget};

pub fn build(keybinds: &[Keybind]) -> KeybindsWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // Header
    let header_box = Box::new(Orientation::Horizontal, 12);
    let title_label = Label::new(Some(&babydra_common::i18n::t("settings.keybinds_title_page")));
    title_label.add_css_class("settings-page-title");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);

    let refresh_btn = Button::with_label(&babydra_common::i18n::t("settings.refresh"));
    refresh_btn.add_css_class("connect-pill-btn");

    let add_btn = Button::with_label(&babydra_common::i18n::t("settings.startup_add_new"));
    add_btn.add_css_class("connect-pill-btn");

    let save_btn = Button::with_label(&babydra_common::i18n::t("settings.save_changes"));
    save_btn.add_css_class("suggested-action");

    header_box.append(&title_label);
    header_box.append(&refresh_btn);
    header_box.append(&add_btn);
    header_box.append(&save_btn);
    container.append(&header_box);

    let glass_card = Box::new(Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");
    glass_card.set_vexpand(true);
    glass_card.set_valign(gtk4::Align::Fill);

    // Table Container Card
    let table_card = Box::new(Orientation::Vertical, 4);

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

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&table_card));

    glass_card.append(&scroll);
    container.append(&glass_card);

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

    let delete_btn = Button::new();
    delete_btn.add_css_class("icon-btn");
    delete_btn.add_css_class("circular");
    delete_btn.add_css_class("delete-btn");
    delete_btn.set_valign(gtk4::Align::Center);
    let del_icon = babydra_utils::ui::icon::get_icon("edit-delete", 16);
    del_icon.set_pixel_size(16);
    delete_btn.set_child(Some(&del_icon));

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
