use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Box, ListBox, Orientation};

pub mod row;

/// Builds the general settings page, mounting various toggles for file exploration features.
pub fn build_general_page() -> Box {
    let settings = babydra_core::load_explore_cfg();
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
        &trans("explore.toggle_hidden"),
        &trans("explore.settings_toggle_hidden_desc"),
        settings.show_hidden,
        std::boxed::Box::new(|state| {
            let mut s = babydra_core::load_explore_cfg();
            s.show_hidden = state;
            babydra_core::save_explore_cfg(&s);
        }),
    );

    // 2. Preview Visible
    row::add_switch_row(
        &listbox,
        "sidebar",
        &trans("explore.toggle_preview"),
        &trans("explore.settings_toggle_preview_desc"),
        settings.preview_visible,
        std::boxed::Box::new(|state| {
            let mut s = babydra_core::load_explore_cfg();
            s.preview_visible = state;
            babydra_core::save_explore_cfg(&s);
        }),
    );

    // 3. Double click to open
    row::add_switch_row(
        &listbox,
        "activity",
        &trans("explore.settings_double_click"),
        &trans("explore.settings_double_click_desc"),
        settings.double_click_to_open,
        std::boxed::Box::new(|state| {
            let mut s = babydra_core::load_explore_cfg();
            s.double_click_to_open = state;
            babydra_core::save_explore_cfg(&s);
        }),
    );

    // 4. Permanent delete
    row::add_switch_row(
        &listbox,
        "trash",
        &trans("explore.settings_permanent_delete"),
        &trans("explore.settings_permanent_delete_desc"),
        settings.permanent_delete,
        std::boxed::Box::new(|state| {
            let mut s = babydra_core::load_explore_cfg();
            s.permanent_delete = state;
            babydra_core::save_explore_cfg(&s);
        }),
    );

    // 5. Calculate folder sizes
    row::add_switch_row(
        &listbox,
        "info",
        &trans("explore.settings_calculate_size"),
        &trans("explore.settings_calculate_size_desc"),
        settings.calculate_dir_size,
        std::boxed::Box::new(|state| {
            let mut s = babydra_core::load_explore_cfg();
            s.calculate_dir_size = state;
            babydra_core::save_explore_cfg(&s);
        }),
    );

    tab_general
}
