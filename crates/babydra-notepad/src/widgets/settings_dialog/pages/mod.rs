//! Pages and shared helpers for the Notepad settings dialog.

mod editor;
mod font;
mod saving;

use super::row::add_switch_row;
use babydra_core::i18n::trans;
use babydra_core::models::notepad::{save_notepad_cfg, NotepadSettings};
use gtk4::prelude::*;
use gtk4::{Box, ListBox, Orientation, ScrolledWindow};
use std::cell::RefCell;
use std::rc::Rc;

pub use editor::build_editor_page;
pub use font::build_font_page;
pub use saving::build_saving_page;

pub(super) type SettingsState = Rc<RefCell<NotepadSettings>>;
pub(super) type SettingsChanged = Rc<dyn Fn(NotepadSettings)>;

pub(super) fn page() -> (ScrolledWindow, ListBox) {
    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .hexpand(true)
        .vexpand(true)
        .build();
    let container = Box::new(Orientation::Vertical, 10);
    container.set_hexpand(true);
    container.set_vexpand(true);

    let list = ListBox::new();
    list.set_selection_mode(gtk4::SelectionMode::None);
    list.add_css_class("settings-card");
    container.append(&list);
    scroll.set_child(Some(&container));

    (scroll, list)
}

pub(super) fn save_change<T>(
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

pub(super) fn add_switch_setting(
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
