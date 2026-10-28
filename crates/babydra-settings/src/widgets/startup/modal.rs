//! Startup Add Dialog
//! Modal dialog for creating new startup application or command entries.

use std::cell::RefCell;
use std::rc::Rc;

use babydra_core::i18n::trans;
use babydra_core::models::desktop::app::DesktopApp;
use babydra_core::services::apps::find_desktop_apps;
use babydra_ui_kit::components::modals::dialog_builder::{
    create_form_label, create_modern_entry, BadgeVariant, ButtonVariant,
    ModernDialogBuilder,
};
use gtk4::prelude::*;
use gtk4::{Box, Button, DropDown, Entry, Orientation, StringList};

/// Cleans an application exec string for startup scripts.
pub fn clean_app_exec(exec: &str) -> String {
    let mut parts: Vec<&str> = exec.split_whitespace().collect();
    parts.retain(|p| !p.starts_with('%'));
    let clean = parts.join(" ");
    let trimmed = clean.trim();
    if trimmed.is_empty() {
        return exec.to_string();
    }
    if !trimmed.ends_with('&') {
        format!("{} &", trimmed)
    } else {
        trimmed.to_string()
    }
}

/// Modal dialog for adding a startup app or shell command.
#[derive(Clone)]
#[allow(dead_code)]
pub struct StartupAddDialog {
    pub container: Box,
    pub name_entry: Entry,
    pub type_dropdown: DropDown,
    pub app_box: Box,
    pub app_dropdown: DropDown,
    pub cmd_box: Box,
    pub cmd_entry: Entry,
    pub cancel_btn: Button,
    pub add_btn: Button,
    apps: Rc<RefCell<Vec<DesktopApp>>>,
    user_edited_name: Rc<RefCell<bool>>,
}

impl Default for StartupAddDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl StartupAddDialog {
    /// Create a new startup creation dialog.
    pub fn new() -> Self {
        let builder = ModernDialogBuilder::new(450)
            .with_badge("rocket", BadgeVariant::Primary)
            .with_title(trans("settings.startup_modal_title"))
            .with_subtitle(trans("settings.startup_modal_sub"));

        let dialog = builder.build();

        // ── 1. Mnemonic Name (Tên gợi nhớ) ───────────────────────────
        let name_grp = Box::new(Orientation::Vertical, 4);
        let name_lbl = create_form_label(&trans("settings.startup_name_label"));
        let name_entry = create_modern_entry(&trans("settings.startup_name_placeholder"));
        name_entry.set_hexpand(true);
        name_grp.append(&name_lbl);
        name_grp.append(&name_entry);
        dialog.add_child(&name_grp);

        // ── 2. Launch Type Dropdown (Loại khởi chạy) ───────────────────
        let type_grp = Box::new(Orientation::Vertical, 4);
        let type_lbl = create_form_label(&trans("settings.startup_type_label"));

        let type_opt1 = trans("settings.startup_type_app");
        let type_opt2 = trans("settings.startup_type_command");
        let type_strings = StringList::new(&[&type_opt1, &type_opt2]);
        let type_dropdown = DropDown::new(Some(type_strings), None::<gtk4::Expression>);
        type_dropdown.set_selected(0);
        type_dropdown.set_cursor_from_name(Some("pointer"));
        type_dropdown.set_hexpand(true);
        type_grp.append(&type_lbl);
        type_grp.append(&type_dropdown);
        dialog.add_child(&type_grp);

        // ── 3. App Selector Box (for Application mode) ─────────────────
        let app_box = Box::new(Orientation::Vertical, 4);
        let app_lbl = create_form_label(&trans("settings.startup_select_app_label"));

        // Fetch desktop apps and sort alphabetically
        let mut raw_apps = find_desktop_apps();
        raw_apps.retain(|a| !a.is_dependency && !a.name.trim().is_empty() && !a.exec.trim().is_empty());
        raw_apps.sort_by_key(|a| a.name.to_lowercase());
        raw_apps.dedup_by(|a, b| a.name == b.name);

        let app_names: Vec<String> = raw_apps.iter().map(|a| a.name.clone()).collect();
        let app_names_refs: Vec<&str> = app_names.iter().map(|s| s.as_str()).collect();
        let app_strings = StringList::new(&app_names_refs);
        let app_dropdown = DropDown::new(Some(app_strings), None::<gtk4::Expression>);
        app_dropdown.set_cursor_from_name(Some("pointer"));
        app_dropdown.set_hexpand(true);

        app_box.append(&app_lbl);
        app_box.append(&app_dropdown);
        dialog.add_child(&app_box);

        // ── 4. Command Entry Box (for Shell Command mode) ──────────────
        let cmd_box = Box::new(Orientation::Vertical, 4);
        let cmd_lbl = create_form_label(&trans("settings.startup_command_label"));
        let cmd_entry = create_modern_entry(&trans("settings.startup_cmd_entry_placeholder"));
        cmd_entry.set_hexpand(true);
        cmd_box.append(&cmd_lbl);
        cmd_box.append(&cmd_entry);
        cmd_box.set_visible(false); // Hidden by default (App mode is active)
        dialog.add_child(&cmd_box);

        // ── Action Buttons ───────────────────────────────────────────
        let cancel_btn = Button::with_label(&trans("common.cancel"));
        let add_btn = Button::with_label(&trans("settings.startup_add"));
        dialog.add_action_buttons(&[
            (&cancel_btn, ButtonVariant::Cancel),
            (&add_btn, ButtonVariant::Primary),
        ]);

        let apps_rc = Rc::new(RefCell::new(raw_apps));
        let user_edited_name = Rc::new(RefCell::new(false));

        // Wire user typing in name_entry to prevent overriding customized names
        let user_edited_c = user_edited_name.clone();
        name_entry.connect_changed(move |entry| {
            if !entry.text().is_empty() {
                *user_edited_c.borrow_mut() = true;
            }
        });

        // Wire app selection notify to auto-fill mnemonic name if user hasn't customized it
        let apps_sel = apps_rc.clone();
        let name_entry_sel = name_entry.clone();
        let user_edited_sel = user_edited_name.clone();
        app_dropdown.connect_selected_notify(move |dd| {
            let idx = dd.selected() as usize;
            if let Some(app) = apps_sel.borrow().get(idx) {
                if !*user_edited_sel.borrow() || name_entry_sel.text().trim().is_empty() {
                    name_entry_sel.set_text(&app.name);
                    *user_edited_sel.borrow_mut() = false;
                }
            }
        });

        // Wire Type dropdown switching (App vs Command)
        let app_box_switch = app_box.clone();
        let cmd_box_switch = cmd_box.clone();
        let name_entry_switch = name_entry.clone();
        let user_edited_switch = user_edited_name.clone();
        let apps_switch = apps_rc.clone();
        let app_dropdown_switch = app_dropdown.clone();

        type_dropdown.connect_selected_notify(move |dd| {
            let is_app = dd.selected() == 0;
            app_box_switch.set_visible(is_app);
            cmd_box_switch.set_visible(!is_app);

            if is_app {
                let idx = app_dropdown_switch.selected() as usize;
                if let Some(app) = apps_switch.borrow().get(idx) {
                    if !*user_edited_switch.borrow() || name_entry_switch.text().trim().is_empty() {
                        name_entry_switch.set_text(&app.name);
                        *user_edited_switch.borrow_mut() = false;
                    }
                }
            } else if !*user_edited_switch.borrow() {
                name_entry_switch.set_text("");
            }
        });

        // Cancel button closes dialog
        let container_cancel = dialog.container().clone();
        cancel_btn.connect_clicked(move |_| {
            container_cancel.set_visible(false);
        });

        Self {
            container: dialog.container().clone(),
            name_entry,
            type_dropdown,
            app_box,
            app_dropdown,
            cmd_box,
            cmd_entry,
            cancel_btn,
            add_btn,
            apps: apps_rc,
            user_edited_name,
        }
    }

    /// Shows the modal and resets input fields.
    pub fn show(&self) {
        *self.user_edited_name.borrow_mut() = false;
        self.cmd_entry.set_text("");
        self.type_dropdown.set_selected(0);
        self.app_box.set_visible(true);
        self.cmd_box.set_visible(false);

        // Pre-fill name with first app if available
        let apps = self.apps.borrow();
        if !apps.is_empty() {
            self.app_dropdown.set_selected(0);
            self.name_entry.set_text(&apps[0].name);
        } else {
            self.name_entry.set_text("");
        }

        self.container.set_visible(true);
    }

    /// Hides the modal.
    #[allow(dead_code)]
    pub fn hide(&self) {
        self.container.set_visible(false);
    }

    /// Connects the add button callback.
    pub fn connect_add<F: Fn(Option<String>, String) + 'static>(&self, callback: F) {
        let name_entry = self.name_entry.clone();
        let type_dropdown = self.type_dropdown.clone();
        let app_dropdown = self.app_dropdown.clone();
        let cmd_entry = self.cmd_entry.clone();
        let apps = self.apps.clone();
        let container = self.container.clone();

        self.add_btn.connect_clicked(move |_| {
            let is_app = type_dropdown.selected() == 0;
            let typed_name = name_entry.text().trim().to_string();

            let (name_opt, command_str) = if is_app {
                let idx = app_dropdown.selected() as usize;
                let apps_borrow = apps.borrow();
                if let Some(app) = apps_borrow.get(idx) {
                    let cmd = clean_app_exec(&app.exec);
                    let name = if typed_name.is_empty() {
                        app.name.clone()
                    } else {
                        typed_name
                    };
                    (Some(name), cmd)
                } else {
                    return;
                }
            } else {
                let cmd = cmd_entry.text().trim().to_string();
                if cmd.is_empty() {
                    return;
                }
                let name = if typed_name.is_empty() {
                    None
                } else {
                    Some(typed_name)
                };
                (name, cmd)
            };

            container.set_visible(false);
            callback(name_opt, command_str);
        });
    }
}
