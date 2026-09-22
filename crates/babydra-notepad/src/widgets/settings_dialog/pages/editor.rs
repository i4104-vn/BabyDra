use super::super::row::add_dropdown_row;
use super::{add_switch_setting, page, save_change, SettingsChanged, SettingsState};
use babydra_core::i18n::trans;
use babydra_core::models::notepad::NotepadSettings;
use babydra_core::services::syntax::list_available_themes;
use gtk4::{ListBox, ScrolledWindow};

const TAB_SIZES: &[&str] = &["2 Spaces", "4 Spaces", "8 Spaces"];

pub fn build_editor_page(state: &SettingsState, changed: &SettingsChanged) -> ScrolledWindow {
    let (page, list) = page();
    let cfg = state.borrow().clone();
    let tab_idx = match cfg.tab_size {
        2 => 0,
        8 => 2,
        _ => 1,
    };

    let state_c = state.clone();
    let changed_c = changed.clone();
    add_dropdown_row(
        &list,
        "sidebar",
        &trans("notepad.settings_tab_size"),
        &trans("notepad.settings_tab_size_desc"),
        TAB_SIZES,
        tab_idx,
        move |idx| {
            let tab_size = match idx {
                0 => 2,
                2 => 8,
                _ => 4,
            };
            save_change(&state_c, &changed_c, tab_size, |settings, value| {
                settings.tab_size = value;
            });
        },
    );

    add_switch_setting(
        &list,
        state,
        changed,
        "terminal",
        "indent_spaces",
        cfg.indent_with_spaces,
        |settings, value| settings.indent_with_spaces = value,
    );
    add_switch_setting(
        &list,
        state,
        changed,
        "forward",
        "auto_indent",
        cfg.auto_indent,
        |settings, value| settings.auto_indent = value,
    );

    let themes = list_available_themes();
    let theme_refs: Vec<&str> = themes.iter().map(String::as_str).collect();
    add_theme_setting(&list, state, changed, &cfg, &themes, &theme_refs, true);
    add_theme_setting(&list, state, changed, &cfg, &themes, &theme_refs, false);

    page
}

fn add_theme_setting(
    list: &ListBox,
    state: &SettingsState,
    changed: &SettingsChanged,
    cfg: &NotepadSettings,
    themes: &[String],
    theme_refs: &[&str],
    dark: bool,
) {
    let key = if dark { "dark_theme" } else { "light_theme" };
    let icon = if dark { "dark-mode" } else { "brightness" };
    let current_theme = if dark {
        cfg.dark_theme.as_str()
    } else {
        cfg.light_theme.as_str()
    };
    let selected = theme_refs
        .iter()
        .position(|theme| *theme == current_theme)
        .unwrap_or(0) as u32;

    let state_c = state.clone();
    let changed_c = changed.clone();
    let themes_c = themes.to_vec();
    add_dropdown_row(
        list,
        icon,
        &trans(&format!("notepad.settings_{key}")),
        &trans(&format!("notepad.settings_{key}_desc")),
        theme_refs,
        selected,
        move |idx| {
            if let Some(theme) = themes_c.get(idx as usize) {
                let theme = theme.clone();
                save_change(&state_c, &changed_c, theme, move |settings, value| {
                    if dark {
                        settings.dark_theme = value;
                    } else {
                        settings.light_theme = value;
                    }
                });
            }
        },
    );
}
