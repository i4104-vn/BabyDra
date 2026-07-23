use gtk4::prelude::*;
use gtk4::{Box, Orientation, ListBox};
use babydra_common::i18n::t;

pub mod row;

/// Builds the general settings page, mounting various toggles for file exploration features.
pub fn build_general_page() -> Box {
    let settings = babydra_common::load_explore_settings();
    let tab_general = Box::new(Orientation::Vertical, 10);
    tab_general.set_margin_top(8);
    tab_general.set_margin_bottom(8);

    let listbox = ListBox::new();
    listbox.set_selection_mode(gtk4::SelectionMode::None);
    listbox.add_css_class("settings-card");
    tab_general.append(&listbox);

    // 1. Show hidden files
    row::add_switch_row(
        &listbox,
        "eye-off",
        &t("explore.toggle_hidden"),
        &t("explore.settings_toggle_hidden_desc"),
        settings.show_hidden,
        std::boxed::Box::new(|state| {
            let mut s = babydra_common::load_explore_settings();
            s.show_hidden = state;
            babydra_common::save_explore_settings(&s);
        }),
    );

    // 2. Preview Visible
    row::add_switch_row(
        &listbox,
        "sidebar",
        &t("explore.toggle_preview"),
        &t("explore.settings_toggle_preview_desc"),
        settings.preview_visible,
        std::boxed::Box::new(|state| {
            let mut s = babydra_common::load_explore_settings();
            s.preview_visible = state;
            babydra_common::save_explore_settings(&s);
        }),
    );

    // 3. Double click to open
    row::add_switch_row(
        &listbox,
        "activity",
        &t("explore.settings_double_click"),
        &t("explore.settings_double_click_desc"),
        settings.double_click_to_open,
        std::boxed::Box::new(|state| {
            let mut s = babydra_common::load_explore_settings();
            s.double_click_to_open = state;
            babydra_common::save_explore_settings(&s);
        }),
    );

    // 4. Permanent delete
    row::add_switch_row(
        &listbox,
        "trash",
        &t("explore.settings_permanent_delete"),
        &t("explore.settings_permanent_delete_desc"),
        settings.permanent_delete,
        std::boxed::Box::new(|state| {
            let mut s = babydra_common::load_explore_settings();
            s.permanent_delete = state;
            babydra_common::save_explore_settings(&s);
        }),
    );

    // 5. Calculate folder sizes
    row::add_switch_row(
        &listbox,
        "info",
        &t("explore.settings_calculate_size"),
        &t("explore.settings_calculate_size_desc"),
        settings.calculate_dir_size,
        std::boxed::Box::new(|state| {
            let mut s = babydra_common::load_explore_settings();
            s.calculate_dir_size = state;
            babydra_common::save_explore_settings(&s);
        }),
    );

    tab_general
}
