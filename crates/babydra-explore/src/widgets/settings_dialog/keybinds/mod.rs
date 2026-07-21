mod dialog;

use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align, ListBox, ListBoxRow};
use babydra_common::i18n::t;
use std::rc::Rc;

/// Builds the keyboard shortcuts list page inside the Settings Dialog.
pub fn build_keybinds_page(
    parent_win: &gtk4::Window,
    on_keybinds_changed: impl Fn() + 'static,
) -> Box {
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

    // Load settings to fetch active shortcuts
    let settings = babydra_common::load_explore_settings();

    // Keep track of shortcut labels so we can update them on capture
    let on_changed = Rc::new(on_keybinds_changed);

    let add_keybind_row = |icon_name: &str, action_id: &str, action_desc: &str| {
        let row = ListBoxRow::new();
        row.add_css_class("settings-card-row");
        row.set_cursor_from_name(Some("pointer"));

        let hbox = Box::new(Orientation::Horizontal, 12);
        hbox.set_margin_top(12);
        hbox.set_margin_bottom(12);
        hbox.set_margin_start(16);
        hbox.set_margin_end(16);

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

        // Get keybind value from settings
        let shortcut_val = settings.get_keybind(action_id);
        let lbl_shortcut = Label::builder()
            .label(&shortcut_val)
            .halign(Align::End)
            .valign(Align::Center)
            .build();
        lbl_shortcut.add_css_class("keybind-pill");
        hbox.append(&lbl_shortcut);

        row.set_child(Some(&hbox));
        listbox.append(&row);

        // Row activation action
        let parent_win_c = parent_win.clone();
        let action_id_c = action_id.to_string();
        let action_desc_c = action_desc.to_string();
        let lbl_shortcut_c = lbl_shortcut.clone();
        let on_changed_c = on_changed.clone();

        row.connect_activate(move |_| {
            let action_id_inner = action_id_c.clone();
            let lbl_inner = lbl_shortcut_c.clone();
            let on_changed_inner = on_changed_c.clone();
            dialog::show_capture_dialog(&parent_win_c, &action_desc_c, move |new_shortcut| {
                // Update settings
                let mut current_settings = babydra_common::load_explore_settings();
                current_settings.keybinds.insert(action_id_inner.clone(), new_shortcut.clone());
                babydra_common::save_explore_settings(&current_settings);

                // Update UI Label
                lbl_inner.set_text(&new_shortcut);

                // Trigger callback
                on_changed_inner();
            });
        });
    };

    // Add dynamic keybind rows
    add_keybind_row("display", "toggle_split", &t("explore.shortcut_split"));
    add_keybind_row("sidebar", "toggle_preview", &t("explore.shortcut_preview"));
    add_keybind_row("eye-off", "toggle_hidden", &t("explore.shortcut_hidden"));
    add_keybind_row("copy", "copy", &t("explore.shortcut_copy"));
    add_keybind_row("cut", "cut", &t("explore.shortcut_cut"));
    add_keybind_row("paste", "paste", &t("explore.shortcut_paste"));
    add_keybind_row("refresh", "undo", "Undo");

    page
}
