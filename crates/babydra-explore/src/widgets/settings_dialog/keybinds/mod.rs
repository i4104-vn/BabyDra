mod render;

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box, Label, ListBox, ListBoxRow, Orientation};
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

    // Load settings to fetch the currently active shortcuts
    let settings = babydra_core::load_explore_cfg();

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

        let icon = babydra_ui_kit::ui::icon::get_icon(icon_name, 16);
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

        // Row click gesture action
        let click_gesture = gtk4::GestureClick::new();
        let parent_win_c = parent_win.clone();
        let action_id_c = action_id.to_string();
        let action_desc_c = action_desc.to_string();
        let lbl_shortcut_c = lbl_shortcut.clone();
        let on_changed_c = on_changed.clone();

        click_gesture.connect_pressed(move |_, _, _, _| {
            let action_id_inner = action_id_c.clone();
            let lbl_inner = lbl_shortcut_c.clone();
            let on_changed_inner = on_changed_c.clone();
            render::show_capture_dialog(&parent_win_c, &action_desc_c, move |new_shortcut| {
                let mut current_settings = babydra_core::load_explore_cfg();
                current_settings
                    .keybinds
                    .insert(action_id_inner.clone(), new_shortcut.clone());
                babydra_core::save_explore_cfg(&current_settings);

                lbl_inner.set_text(&new_shortcut);

                on_changed_inner();
            });
        });
        row.add_controller(click_gesture);
    };

    add_keybind_row("display", "toggle_split", &trans("explore.shortcut_split"));
    add_keybind_row(
        "sidebar",
        "toggle_preview",
        &trans("explore.shortcut_preview"),
    );
    add_keybind_row(
        "eye-off",
        "toggle_hidden",
        &trans("explore.shortcut_hidden"),
    );
    add_keybind_row("copy", "copy", &trans("explore.shortcut_copy"));
    add_keybind_row("cut", "cut", &trans("explore.shortcut_cut"));
    add_keybind_row("paste", "paste", &trans("explore.shortcut_paste"));
    add_keybind_row("refresh", "undo", &trans("explore.shortcut_undo"));
    add_keybind_row("trash", "delete", &trans("explore.shortcut_delete"));
    add_keybind_row("trash", "permanent_delete", &trans("explore.shortcut_perm_delete"));
    add_keybind_row("check", "select_all", &trans("explore.shortcut_select_all"));
    add_keybind_row("plus", "new_tab", &trans("explore.shortcut_new_tab"));
    add_keybind_row("close", "close_tab", &trans("explore.shortcut_close_tab"));

    page
}
