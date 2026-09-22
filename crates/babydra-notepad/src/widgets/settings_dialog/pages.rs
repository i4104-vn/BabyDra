//! Settings pages and their configuration bindings.

use super::row::{add_dropdown_row, add_spin_row, add_switch_row};
use babydra_core::i18n::trans;
use babydra_core::models::notepad::{save_notepad_cfg, NotepadSettings};
use babydra_core::services::syntax::list_available_themes;
use gtk4::prelude::*;
use gtk4::{Box, ListBox, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

const FONT_FAMILIES: &[&str] = &[
    "Monospace",
    "JetBrains Mono",
    "Fira Code",
    "Source Code Pro",
    "Cascadia Code",
    "Courier New",
    "Cantarell",
];
const TAB_SIZES: &[&str] = &["2 Spaces", "4 Spaces", "8 Spaces"];

type SettingsState = Rc<RefCell<NotepadSettings>>;
type SettingsChanged = Rc<dyn Fn(NotepadSettings)>;

fn page() -> (Box, ListBox) {
    let container = Box::new(Orientation::Vertical, 10);
    let list = ListBox::new();
    list.set_selection_mode(gtk4::SelectionMode::None);
    list.add_css_class("settings-card");
    container.append(&list);
    (container, list)
}

fn save_change<T>(
    state: &SettingsState,
    changed: &SettingsChanged,
    value: T,
    update: impl Fn(&mut NotepadSettings, T),
) {
    let mut settings = state.borrow_mut();
    update(&mut settings, value);
    save_notepad_cfg(&settings);
    changed(settings.clone());
}

pub fn build_font_page(state: &SettingsState, changed: &SettingsChanged) -> Box {
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
        "type",
        &trans("notepad.settings_font_family"),
        &trans("notepad.settings_font_family_desc"),
        FONT_FAMILIES,
        font_idx,
        move |idx| {
            let font = FONT_FAMILIES.get(idx as usize).unwrap_or(&"Monospace");
            save_change(&state_c, &changed_c, font.to_string(), |s, value| {
                s.font_family = value
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
        move |value| save_change(&state_c, &changed_c, value, |s, value| s.font_size = value),
    );
    add_switch_setting(
        &list,
        state,
        changed,
        "list",
        "show_line_numbers",
        cfg.show_line_numbers,
        |s, v| s.show_line_numbers = v,
    );
    add_switch_setting(
        &list,
        state,
        changed,
        "align-left",
        "word_wrap",
        cfg.word_wrap,
        |s, v| s.word_wrap = v,
    );
    page
}

pub fn build_editor_page(state: &SettingsState, changed: &SettingsChanged) -> Box {
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
            save_change(
                &state_c,
                &changed_c,
                match idx {
                    0 => 2,
                    2 => 8,
                    _ => 4,
                },
                |s, v| s.tab_size = v,
            )
        },
    );
    add_switch_setting(
        &list,
        state,
        changed,
        "code",
        "indent_spaces",
        cfg.indent_with_spaces,
        |s, v| s.indent_with_spaces = v,
    );
    add_switch_setting(
        &list,
        state,
        changed,
        "corner-down-right",
        "auto_indent",
        cfg.auto_indent,
        |s, v| s.auto_indent = v,
    );
    let themes = list_available_themes();
    let theme_refs: Vec<&str> = themes.iter().map(String::as_str).collect();
    add_theme_setting(&list, state, changed, &cfg, &themes, &theme_refs, true);
    add_theme_setting(&list, state, changed, &cfg, &themes, &theme_refs, false);
    page
}

pub fn build_saving_page(state: &SettingsState, changed: &SettingsChanged) -> Box {
    let (page, list) = page();
    let cfg = state.borrow().clone();
    add_switch_setting(
        &list,
        state,
        changed,
        "clock",
        "auto_save",
        cfg.auto_save,
        |s, v| s.auto_save = v,
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
            save_change(&state_c, &changed_c, value, |s, v| {
                s.auto_save_delay_seconds = v
            })
        },
    );
    add_switch_setting(
        &list,
        state,
        changed,
        "delete",
        "trim_whitespace",
        cfg.trim_trailing_whitespace,
        |s, v| s.trim_trailing_whitespace = v,
    );
    add_switch_setting(
        &list,
        state,
        changed,
        "check-circle",
        "insert_newline",
        cfg.insert_final_newline,
        |s, v| s.insert_final_newline = v,
    );
    page
}

fn add_switch_setting(
    list: &ListBox,
    state: &SettingsState,
    changed: &SettingsChanged,
    icon: &str,
    key: &str,
    value: bool,
    update: impl Fn(&mut NotepadSettings, bool) + 'static,
) {
    let state_c = state.clone();
    let changed_c = changed.clone();
    add_switch_row(
        list,
        icon,
        &trans(&format!("notepad.settings_{key}")),
        &trans(&format!("notepad.settings_{key}_desc")),
        value,
        move |value| save_change(&state_c, &changed_c, value, &update),
    );
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
    let icon = if dark { "moon" } else { "sun" };
    let selected = theme_refs
        .iter()
        .position(|theme| {
            *theme
                == if dark {
                    cfg.dark_theme.as_str()
                } else {
                    cfg.light_theme.as_str()
                }
        })
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
                save_change(&state_c, &changed_c, theme, move |s, value| {
                    if dark {
                        s.dark_theme = value
                    } else {
                        s.light_theme = value
                    }
                });
            }
        },
    );
}
