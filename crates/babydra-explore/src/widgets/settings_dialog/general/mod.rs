//! General settings tab builder for file explorer options.

use babydra_core::i18n::trans;
use babydra_ui_kit::components::create_switch_row;
use gtk4::prelude::*;
use gtk4::{Align, Box, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow};

/// Builds the general settings page, mounting various toggles for file exploration features.
pub fn build_general_page() -> ScrolledWindow {
    let settings = babydra_core::load_explore_cfg();

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .hexpand(true)
        .build();

    let content_box = Box::new(Orientation::Vertical, 12);
    content_box.set_margin_top(20);
    content_box.set_margin_bottom(20);
    content_box.set_margin_start(24);
    content_box.set_margin_end(24);
    content_box.set_vexpand(true);
    content_box.set_hexpand(true);

    // Page Header matching standard settings layout
    let header_box = Box::new(Orientation::Vertical, 4);
    header_box.set_margin_bottom(6);

    let title_lbl = Label::new(Some(&trans("explore.settings_general")));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(Align::Start);
    header_box.append(&title_lbl);

    let subtitle_lbl = Label::new(Some(&trans("explore.settings_behavior")));
    subtitle_lbl.add_css_class("settings-page-subtitle");
    subtitle_lbl.set_halign(Align::Start);
    header_box.append(&subtitle_lbl);

    content_box.append(&header_box);

    let listbox = ListBox::new();
    listbox.set_selection_mode(gtk4::SelectionMode::None);
    listbox.add_css_class("settings-card");
    content_box.append(&listbox);

    let append_toggle = |list: &ListBox, icon: &str, title: &str, desc: &str, initial: bool, on_change: fn(bool)| {
        let row_widget = ListBoxRow::new();
        row_widget.add_css_class("settings-card-row");
        let (row, switch) = create_switch_row(icon, title, desc, initial);
        switch.connect_state_set(move |state| {
            on_change(state);
        });
        row_widget.set_child(Some(&row));
        list.append(&row_widget);
    };

    // 1. Show hidden files
    append_toggle(
        &listbox,
        "eye-off",
        &trans("explore.toggle_hidden"),
        &trans("explore.settings_toggle_hidden_desc"),
        settings.show_hidden,
        |state| {
            let mut s = babydra_core::load_explore_cfg();
            s.show_hidden = state;
            babydra_core::save_explore_cfg(&s);
        },
    );

    // 2. Preview Visible
    append_toggle(
        &listbox,
        "sidebar",
        &trans("explore.toggle_preview"),
        &trans("explore.settings_toggle_preview_desc"),
        settings.preview_visible,
        |state| {
            let mut s = babydra_core::load_explore_cfg();
            s.preview_visible = state;
            babydra_core::save_explore_cfg(&s);
        },
    );

    // 3. Double click to open
    append_toggle(
        &listbox,
        "activity",
        &trans("explore.settings_double_click"),
        &trans("explore.settings_double_click_desc"),
        settings.double_click_to_open,
        |state| {
            let mut s = babydra_core::load_explore_cfg();
            s.double_click_to_open = state;
            babydra_core::save_explore_cfg(&s);
        },
    );

    // 4. Permanent delete
    append_toggle(
        &listbox,
        "trash",
        &trans("explore.settings_permanent_delete"),
        &trans("explore.settings_permanent_delete_desc"),
        settings.permanent_delete,
        |state| {
            let mut s = babydra_core::load_explore_cfg();
            s.permanent_delete = state;
            babydra_core::save_explore_cfg(&s);
        },
    );

    // 5. Calculate folder sizes
    append_toggle(
        &listbox,
        "info",
        &trans("explore.settings_calculate_size"),
        &trans("explore.settings_calculate_size_desc"),
        settings.calculate_dir_size,
        |state| {
            let mut s = babydra_core::load_explore_cfg();
            s.calculate_dir_size = state;
            babydra_core::save_explore_cfg(&s);
        },
    );

    scroll.set_child(Some(&content_box));
    scroll
}
