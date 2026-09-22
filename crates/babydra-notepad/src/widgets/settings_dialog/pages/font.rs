use super::super::row::{add_dropdown_row, add_spin_row};
use super::{add_switch_setting, page, save_change, SettingsChanged, SettingsState};
use babydra_core::i18n::trans;
use gtk4::ScrolledWindow;

const FONT_FAMILIES: &[&str] = &[
    "Monospace",
    "JetBrains Mono",
    "Fira Code",
    "Source Code Pro",
    "Cascadia Code",
    "Courier New",
    "Cantarell",
];

pub fn build_font_page(state: &SettingsState, changed: &SettingsChanged) -> ScrolledWindow {
    let (page, list) = page();
    let cfg = state.borrow().clone();
    let font_idx = FONT_FAMILIES
        .iter()
        .position(|font| font.eq_ignore_ascii_case(&cfg.font_family))
        .unwrap_or(0) as u32;

    let state_c = state.clone();
    let changed_c = changed.clone();
    add_dropdown_row(
        &list,
        "text",
        &trans("notepad.settings_font_family"),
        &trans("notepad.settings_font_family_desc"),
        FONT_FAMILIES,
        font_idx,
        move |idx| {
            let font = FONT_FAMILIES.get(idx as usize).unwrap_or(&"Monospace");
            save_change(&state_c, &changed_c, font.to_string(), |settings, value| {
                settings.font_family = value;
            });
        },
    );

    let state_c = state.clone();
    let changed_c = changed.clone();
    add_spin_row(
        &list,
        "zoom-in",
        &trans("notepad.settings_font_size"),
        &trans("notepad.settings_font_size_desc"),
        cfg.font_size,
        8.0,
        48.0,
        1.0,
        move |value| {
            save_change(&state_c, &changed_c, value, |settings, value| {
                settings.font_size = value;
            })
        },
    );

    add_switch_setting(
        &list,
        state,
        changed,
        "view-list",
        "show_line_numbers",
        cfg.show_line_numbers,
        |settings, value| settings.show_line_numbers = value,
    );
    add_switch_setting(
        &list,
        state,
        changed,
        "text",
        "word_wrap",
        cfg.word_wrap,
        |settings, value| settings.word_wrap = value,
    );

    page
}
