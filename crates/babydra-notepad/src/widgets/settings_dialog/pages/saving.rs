use super::super::row::add_spin_row;
use super::{add_switch_setting, page, save_change, SettingsChanged, SettingsState};
use babydra_core::i18n::trans;
use gtk4::ScrolledWindow;

pub fn build_saving_page(state: &SettingsState, changed: &SettingsChanged) -> ScrolledWindow {
    let (page, list) = page();
    let cfg = state.borrow().clone();

    add_switch_setting(
        &list,
        state,
        changed,
        "clock",
        "auto_save",
        cfg.auto_save,
        |settings, value| settings.auto_save = value,
    );

    let state_c = state.clone();
    let changed_c = changed.clone();
    add_spin_row(
        &list,
        "activity",
        &trans("notepad.settings_auto_save_delay"),
        &trans("notepad.settings_auto_save_delay_desc"),
        cfg.auto_save_delay_seconds,
        1.0,
        60.0,
        1.0,
        move |value| {
            save_change(&state_c, &changed_c, value, |settings, value| {
                settings.auto_save_delay_seconds = value;
            });
        },
    );

    add_switch_setting(
        &list,
        state,
        changed,
        "broom",
        "trim_whitespace",
        cfg.trim_trailing_whitespace,
        |settings, value| settings.trim_trailing_whitespace = value,
    );
    add_switch_setting(
        &list,
        state,
        changed,
        "check",
        "insert_newline",
        cfg.insert_final_newline,
        |settings, value| settings.insert_final_newline = value,
    );

    page
}
