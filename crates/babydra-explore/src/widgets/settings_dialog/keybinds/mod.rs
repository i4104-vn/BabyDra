use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align, ListBox, ListBoxRow};
use babydra_common::i18n::t;

/// Builds the keyboard shortcuts list page inside the Settings Dialog.
pub fn build_keybinds_page() -> Box {
    let page = Box::new(Orientation::Vertical, 10);
    page.set_margin_top(8);
    page.set_margin_bottom(8);

    let scroll = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .build();
    page.append(&scroll);

    let listbox = ListBox::new();
    listbox.set_selection_mode(gtk4::SelectionMode::None);
    listbox.add_css_class("settings-card");
    scroll.set_child(Some(&listbox));

    let add_keybind_row = |listbox: &ListBox, icon_name: &str, action_desc: &str, shortcut: &str| {
        let row = ListBoxRow::new();
        row.add_css_class("settings-card-row");

        let hbox = Box::new(Orientation::Horizontal, 12);
        hbox.set_margin_top(12);
        hbox.set_margin_bottom(12);
        hbox.set_margin_start(16);
        hbox.set_margin_end(16);

        // Icon
        let icon = babydra_utils::ui::icon::get_icon(icon_name, 16);
        icon.set_valign(Align::Center);
        icon.add_css_class("settings-row-icon");
        hbox.append(&icon);

        let lbl_desc = Label::builder()
            .label(action_desc)
            .halign(Align::Start)
            .valign(Align::Center)
            .hexpand(true)
            .build();
        lbl_desc.add_css_class("settings-row-title");
        hbox.append(&lbl_desc);

        let lbl_shortcut = Label::builder()
            .label(shortcut)
            .halign(Align::End)
            .valign(Align::Center)
            .build();
        lbl_shortcut.add_css_class("keybind-pill");
        hbox.append(&lbl_shortcut);

        row.set_child(Some(&hbox));
        listbox.append(&row);
    };

    // Add shortcuts
    add_keybind_row(&listbox, "folder", &t("explore.shortcut_open"), "Enter / Double Click");
    add_keybind_row(&listbox, "display", &t("explore.shortcut_split"), "F3");
    add_keybind_row(&listbox, "sidebar", &t("explore.shortcut_preview"), "F4");
    add_keybind_row(&listbox, "eye-off", &t("explore.shortcut_hidden"), "Ctrl + H");
    add_keybind_row(&listbox, "copy", &t("explore.shortcut_copy"), "Ctrl + C");
    add_keybind_row(&listbox, "cut", &t("explore.shortcut_cut"), "Ctrl + X");
    add_keybind_row(&listbox, "paste", &t("explore.shortcut_paste"), "Ctrl + V");

    page
}
