use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align, ListBox, ListBoxRow};
use babydra_common::i18n::t;

/// Builds the keyboard shortcuts list page inside the Settings Dialog.
pub fn build_keybinds_page() -> Box {
    let page = Box::new(Orientation::Vertical, 10);

    let lbl_title = Label::builder()
        .label(&t("explore.keybinds_title"))
        .halign(Align::Start)
        .build();
    lbl_title.add_css_class("settings-title-label");
    page.append(&lbl_title);

    let scroll = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .build();
    page.append(&scroll);

    let listbox = ListBox::new();
    listbox.set_selection_mode(gtk4::SelectionMode::None);
    listbox.add_css_class("settings-listbox");
    scroll.set_child(Some(&listbox));

    let add_keybind_row = |listbox: &ListBox, action_desc: &str, shortcut: &str| {
        let row = ListBoxRow::new();
        let hbox = Box::new(Orientation::Horizontal, 12);
        hbox.set_margin_top(10);
        hbox.set_margin_bottom(10);
        hbox.set_margin_start(12);
        hbox.set_margin_end(12);

        let lbl_desc = Label::builder()
            .label(action_desc)
            .halign(Align::Start)
            .hexpand(true)
            .build();
        lbl_desc.add_css_class("settings-row-title");
        hbox.append(&lbl_desc);

        let lbl_shortcut = Label::builder()
            .label(shortcut)
            .halign(Align::End)
            .build();
        lbl_shortcut.add_css_class("keybind-pill");
        hbox.append(&lbl_shortcut);

        row.set_child(Some(&hbox));
        listbox.append(&row);
    };

    // Add shortcuts
    add_keybind_row(&listbox, &t("explore.shortcut_open"), "Enter / Double Click");
    add_keybind_row(&listbox, &t("explore.shortcut_split"), "F3");
    add_keybind_row(&listbox, &t("explore.shortcut_preview"), "F4");
    add_keybind_row(&listbox, &t("explore.shortcut_hidden"), "Ctrl + H");
    add_keybind_row(&listbox, &t("explore.shortcut_copy"), "Ctrl + C");
    add_keybind_row(&listbox, &t("explore.shortcut_cut"), "Ctrl + X");
    add_keybind_row(&listbox, &t("explore.shortcut_paste"), "Ctrl + V");

    page
}
