//! Modal dialogs for updating user display name, system hostname, and account password.

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, PasswordEntry};
use std::boxed::Box as StdBox;
use std::rc::Rc;

use crate::components::modals::dialog_builder::{
    ActionButton, BadgeVariant, ButtonVariant, ModernDialogBuilder, create_error_label,
    create_form_label, create_modern_entry, create_modern_password_entry,
};

/// Dialog for updating the user's full / display name.
#[derive(Clone)]
pub struct ChangeNameDialog {
    pub container: Box,
    pub name_entry: Entry,
    pub pwd_entry: PasswordEntry,
    pub error_lbl: Label,
    pub cancel_btn: Button,
    pub confirm_btn: Button,
}

impl ChangeNameDialog {
    pub fn new() -> Self {
        let builder = ModernDialogBuilder::new(420)
            .with_badge("user", BadgeVariant::Primary)
            .with_title(&trans("settings.dialog_change_name_title"))
            .with_subtitle(&trans("settings.dialog_change_name_sub"));

        let dialog = builder.build();

        let error_lbl = create_error_label();
        dialog.add_child(&error_lbl);

        let name_lbl = create_form_label(&trans("settings.new_name_placeholder"));
        dialog.add_child(&name_lbl);

        let name_entry = create_modern_entry(&trans("settings.new_name_placeholder"));
        dialog.add_child(&name_entry);

        let pwd_lbl = create_form_label(&trans("settings.sudo_password_placeholder"));
        dialog.add_child(&pwd_lbl);

        let pwd_entry = create_modern_password_entry(&trans("settings.sudo_password_placeholder"));
        dialog.add_child(&pwd_entry);

        let cancel_btn = Button::with_label(&trans("common.cancel"));
        let confirm_btn = Button::with_label(&trans("settings.save"));

        dialog.add_actions(vec![
ActionButton {
                label: trans("common.cancel"),
                variant: ButtonVariant::Cancel,
                callback: StdBox::new({
                    let c = dialog.container().clone();
                    let n = name_entry.clone();
                    let p = pwd_entry.clone();
                    let e = error_lbl.clone();
                    move || {
                        n.set_text("");
                        p.set_text("");
                        e.set_visible(false);
                        c.set_visible(false);
                    }
                }),
            },
            ActionButton {
                label: trans("settings.save"),
                variant: ButtonVariant::Primary,
                callback: StdBox::new(|| {}),
            },
        ]);

        let s = Self {
            container: dialog.container().clone(),
            name_entry,
            pwd_entry,
            error_lbl,
            cancel_btn,
            confirm_btn,
        };

        s
    }

    pub fn show_for(&self, current_name: &str) {
        self.name_entry.set_text(current_name);
        self.pwd_entry.set_text("");
        self.error_lbl.set_visible(false);
        self.container.set_visible(true);
        self.name_entry.grab_focus();
    }

    pub fn hide(&self) {
        self.name_entry.set_text("");
        self.pwd_entry.set_text("");
        self.error_lbl.set_visible(false);
        self.container.set_visible(false);
    }

    pub fn show_error(&self, error: &str) {
        self.error_lbl.set_text(error);
        self.error_lbl.set_visible(true);
    }

    pub fn connect_submit<F: Fn(String, String) + 'static>(&self, callback: F) {
        let callback_rc = Rc::new(callback);
        let name_c = self.name_entry.clone();
        let pwd_c = self.pwd_entry.clone();

        let cb1 = callback_rc.clone();
        let name1 = name_c.clone();
        let pwd1 = pwd_c.clone();
        self.confirm_btn.connect_clicked(move |_| {
            let name = name1.text().to_string();
            let pwd = pwd1.text().to_string();
            cb1(name, pwd);
        });

        let cb2 = callback_rc;
        self.pwd_entry.connect_activate(move |_| {
            let name = name_c.text().to_string();
            let pwd = pwd_c.text().to_string();
            cb2(name, pwd);
        });
    }
}

/// Dialog for updating the system hostname.
#[derive(Clone)]
pub struct ChangeHostnameDialog {
    pub container: Box,
    pub hostname_entry: Entry,
    pub pwd_entry: PasswordEntry,
    pub error_lbl: Label,
    pub cancel_btn: Button,
    pub confirm_btn: Button,
}

impl ChangeHostnameDialog {
    pub fn new() -> Self {
        let builder = ModernDialogBuilder::new(420)
            .with_badge("desktop", BadgeVariant::Primary)
            .with_title(&trans("settings.dialog_change_hostname_title"))
            .with_subtitle(&trans("settings.dialog_change_hostname_sub"));

        let dialog = builder.build();

        let error_lbl = create_error_label();
        dialog.add_child(&error_lbl);

        let host_lbl = create_form_label(&trans("settings.new_hostname_placeholder"));
        dialog.add_child(&host_lbl);

        let hostname_entry = create_modern_entry(&trans("settings.new_hostname_placeholder"));
        dialog.add_child(&hostname_entry);

        let pwd_lbl = create_form_label(&trans("settings.sudo_password_placeholder"));
        dialog.add_child(&pwd_lbl);

        let pwd_entry = create_modern_password_entry(&trans("settings.sudo_password_placeholder"));
        dialog.add_child(&pwd_entry);

        let cancel_btn = Button::with_label(&trans("common.cancel"));
        let confirm_btn = Button::with_label(&trans("settings.save"));

        dialog.add_actions(vec![
            ActionButton {
                label: trans("common.cancel"),
                variant: ButtonVariant::Cancel,
                callback: StdBox::new({
                    let c = dialog.container().clone();
                    let h = hostname_entry.clone();
                    let p = pwd_entry.clone();
                    let e = error_lbl.clone();
                    move || {
                        h.set_text("");
                        p.set_text("");
                        e.set_visible(false);
                        c.set_visible(false);
                    }
                }),
            },
            ActionButton {
                label: trans("settings.save"),
                variant: ButtonVariant::Primary,
                callback: StdBox::new(|| {}),
            },
        ]);

        let s = Self {
            container: dialog.container().clone(),
            hostname_entry,
            pwd_entry,
            error_lbl,
            cancel_btn,
            confirm_btn,
        };

        s
    }

    pub fn show_for(&self, current_hostname: &str) {
        self.hostname_entry.set_text(current_hostname);
        self.pwd_entry.set_text("");
        self.error_lbl.set_visible(false);
        self.container.set_visible(true);
        self.hostname_entry.grab_focus();
    }

    pub fn hide(&self) {
        self.hostname_entry.set_text("");
        self.pwd_entry.set_text("");
        self.error_lbl.set_visible(false);
        self.container.set_visible(false);
    }

    pub fn show_error(&self, error: &str) {
        self.error_lbl.set_text(error);
        self.error_lbl.set_visible(true);
    }

    pub fn connect_submit<F: Fn(String, String) + 'static>(&self, callback: F) {
        let callback_rc = Rc::new(callback);
        let host_c = self.hostname_entry.clone();
        let pwd_c = self.pwd_entry.clone();

        let cb1 = callback_rc.clone();
        let host1 = host_c.clone();
        let pwd1 = pwd_c.clone();
        self.confirm_btn.connect_clicked(move |_| {
            let host = host1.text().to_string();
            let pwd = pwd1.text().to_string();
            cb1(host, pwd);
        });

        let cb2 = callback_rc;
        self.pwd_entry.connect_activate(move |_| {
            let host = host_c.text().to_string();
            let pwd = pwd_c.text().to_string();
            cb2(host, pwd);
        });
    }
}

/// Dialog for changing the user's password.
#[derive(Clone)]
pub struct ChangePasswordDialog {
    pub container: Box,
    pub current_pwd_entry: PasswordEntry,
    pub new_pwd_entry: PasswordEntry,
    pub confirm_pwd_entry: PasswordEntry,
    pub error_lbl: Label,
    pub cancel_btn: Button,
    pub confirm_btn: Button,
}

impl ChangePasswordDialog {
    pub fn new() -> Self {
        let builder = ModernDialogBuilder::new(420)
            .with_badge("lock", BadgeVariant::Primary)
            .with_title(&trans("settings.dialog_change_password_title"))
            .with_subtitle(&trans("settings.dialog_change_password_sub"));

        let dialog = builder.build();

        let error_lbl = create_error_label();
        dialog.add_child(&error_lbl);

        let cur_lbl = create_form_label(&trans("settings.current_password"));
        dialog.add_child(&cur_lbl);

        let current_pwd_entry = create_modern_password_entry(&trans("settings.current_password"));
        dialog.add_child(&current_pwd_entry);

        let new_lbl = create_form_label(&trans("settings.new_password"));
        dialog.add_child(&new_lbl);

        let new_pwd_entry = create_modern_password_entry(&trans("settings.new_password"));
        dialog.add_child(&new_pwd_entry);

        let conf_lbl = create_form_label(&trans("settings.confirm_password"));
        dialog.add_child(&conf_lbl);

        let confirm_pwd_entry = create_modern_password_entry(&trans("settings.confirm_password"));
        dialog.add_child(&confirm_pwd_entry);

        dialog.add_actions(vec![
            ActionButton {
                label: trans("common.cancel"),
                variant: ButtonVariant::Cancel,
                callback: StdBox::new({
                    let c = dialog.container().clone();
                    let cur = current_pwd_entry.clone();
                    let new = new_pwd_entry.clone();
                    let conf = confirm_pwd_entry.clone();
                    let e = error_lbl.clone();
                    move || {
                        cur.set_text("");
                        new.set_text("");
                        conf.set_text("");
                        e.set_visible(false);
                        c.set_visible(false);
                    }
                }),
            },
ActionButton {
                label: trans("settings.save"),
                variant: ButtonVariant::Primary,
                callback: StdBox::new(|| {}),
            },
        ]);

        let s = Self {
            container: dialog.container().clone(),
            current_pwd_entry,
            new_pwd_entry,
            confirm_pwd_entry,
            error_lbl,
            cancel_btn: Button::new(),
            confirm_btn: Button::new(),
        };

        s
    }

    pub fn show(&self) {
        self.current_pwd_entry.set_text("");
        self.new_pwd_entry.set_text("");
        self.confirm_pwd_entry.set_text("");
        self.error_lbl.set_visible(false);
        self.container.set_visible(true);
        self.current_pwd_entry.grab_focus();
    }

    pub fn hide(&self) {
        self.current_pwd_entry.set_text("");
        self.new_pwd_entry.set_text("");
        self.confirm_pwd_entry.set_text("");
        self.error_lbl.set_visible(false);
        self.container.set_visible(false);
    }

    pub fn show_error(&self, error: &str) {
        self.error_lbl.set_text(error);
        self.error_lbl.set_visible(true);
    }

    pub fn connect_submit<F: Fn(String, String, String) + 'static>(&self, callback: F) {
        let callback_rc = Rc::new(callback);
        let cur_c = self.current_pwd_entry.clone();
        let new_c = self.new_pwd_entry.clone();
        let conf_c = self.confirm_pwd_entry.clone();

        let cb1 = callback_rc.clone();
        let cur1 = cur_c.clone();
        let new1 = new_c.clone();
        let conf1 = conf_c.clone();
        self.confirm_btn.connect_clicked(move |_| {
            let cur = cur1.text().to_string();
            let new = new1.text().to_string();
            let conf = conf1.text().to_string();
            cb1(cur, new, conf);
        });

        let cb2 = callback_rc;
        self.confirm_pwd_entry.connect_activate(move |_| {
            let cur = cur_c.text().to_string();
            let new = new_c.text().to_string();
            let conf = conf_c.text().to_string();
            cb2(cur, new, conf);
        });
    }
}