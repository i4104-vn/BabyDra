//! Modal dialogs for updating user display name, system hostname, and account password.

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, Orientation, PasswordEntry};
use std::rc::Rc;

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
        let container = Box::new(Orientation::Vertical, 14);
        container.add_css_class("auth-dialog-card");
        container.set_halign(gtk4::Align::Center);
        container.set_valign(gtk4::Align::Center);
        container.set_visible(false);
        container.set_width_request(380);

        // Header
        let header_box = Box::new(Orientation::Horizontal, 12);
        let user_icon = crate::ui::icon::get_icon("user", 24);
        user_icon.set_pixel_size(24);
        user_icon.set_valign(gtk4::Align::Center);
        header_box.append(&user_icon);

        let title_box = Box::new(Orientation::Vertical, 2);
        let title_lbl = Label::new(Some(&trans("settings.dialog_change_name_title")));
        title_lbl.add_css_class("settings-row-title");
        title_lbl.set_halign(gtk4::Align::Start);

        let sub_lbl = Label::new(Some(&trans("settings.dialog_change_name_sub")));
        sub_lbl.add_css_class("settings-row-desc");
        sub_lbl.set_halign(gtk4::Align::Start);
        sub_lbl.set_wrap(true);

        title_box.append(&title_lbl);
        title_box.append(&sub_lbl);
        header_box.append(&title_box);
        container.append(&header_box);

        // Error message
        let error_lbl = Label::new(None);
        error_lbl.add_css_class("dialog-error-text");
        error_lbl.set_halign(gtk4::Align::Start);
        error_lbl.set_visible(false);
        container.append(&error_lbl);

        // New name field
        let name_lbl = Label::new(Some(&trans("settings.new_name_placeholder")));
        name_lbl.add_css_class("settings-row-desc");
        name_lbl.set_halign(gtk4::Align::Start);
        container.append(&name_lbl);

        let name_entry = Entry::new();
        name_entry.add_css_class("sidebar-search-entry");
        name_entry.set_placeholder_text(Some(&trans("settings.new_name_placeholder")));
        container.append(&name_entry);

        // Sudo password field
        let pwd_lbl = Label::new(Some(&trans("settings.sudo_password_placeholder")));
        pwd_lbl.add_css_class("settings-row-desc");
        pwd_lbl.set_halign(gtk4::Align::Start);
        container.append(&pwd_lbl);

        let pwd_entry = PasswordEntry::new();
        pwd_entry.add_css_class("sidebar-search-entry");
        pwd_entry.set_placeholder_text(Some(&trans("settings.sudo_password_placeholder")));
        container.append(&pwd_entry);

        // Action buttons
        let actions_box = Box::new(Orientation::Horizontal, 8);
        actions_box.set_halign(gtk4::Align::End);
        actions_box.set_margin_top(6);

        let cancel_btn = Button::with_label(&trans("common.cancel"));
        cancel_btn.add_css_class("connect-pill-btn");
        cancel_btn.set_cursor_from_name(Some("pointer"));

        let confirm_btn = Button::with_label(&trans("settings.save"));
        confirm_btn.add_css_class("suggested-action");
        confirm_btn.set_cursor_from_name(Some("pointer"));

        actions_box.append(&cancel_btn);
        actions_box.append(&confirm_btn);
        container.append(&actions_box);

        let dialog = Self {
            container,
            name_entry,
            pwd_entry,
            error_lbl,
            cancel_btn,
            confirm_btn,
        };

        // Wire cancel
        let box_c = dialog.container.clone();
        let name_c = dialog.name_entry.clone();
        let pwd_c = dialog.pwd_entry.clone();
        let err_c = dialog.error_lbl.clone();
        dialog.cancel_btn.connect_clicked(move |_| {
            name_c.set_text("");
            pwd_c.set_text("");
            err_c.set_visible(false);
            box_c.set_visible(false);
        });

        dialog
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
        let container = Box::new(Orientation::Vertical, 14);
        container.add_css_class("auth-dialog-card");
        container.set_halign(gtk4::Align::Center);
        container.set_valign(gtk4::Align::Center);
        container.set_visible(false);
        container.set_width_request(380);

        // Header
        let header_box = Box::new(Orientation::Horizontal, 12);
        let host_icon = crate::ui::icon::get_icon("desktop", 24);
        host_icon.set_pixel_size(24);
        host_icon.set_valign(gtk4::Align::Center);
        header_box.append(&host_icon);

        let title_box = Box::new(Orientation::Vertical, 2);
        let title_lbl = Label::new(Some(&trans("settings.dialog_change_hostname_title")));
        title_lbl.add_css_class("settings-row-title");
        title_lbl.set_halign(gtk4::Align::Start);

        let sub_lbl = Label::new(Some(&trans("settings.dialog_change_hostname_sub")));
        sub_lbl.add_css_class("settings-row-desc");
        sub_lbl.set_halign(gtk4::Align::Start);
        sub_lbl.set_wrap(true);

        title_box.append(&title_lbl);
        title_box.append(&sub_lbl);
        header_box.append(&title_box);
        container.append(&header_box);

        // Error message
        let error_lbl = Label::new(None);
        error_lbl.add_css_class("dialog-error-text");
        error_lbl.set_halign(gtk4::Align::Start);
        error_lbl.set_visible(false);
        container.append(&error_lbl);

        // New hostname field
        let host_lbl = Label::new(Some(&trans("settings.new_hostname_placeholder")));
        host_lbl.add_css_class("settings-row-desc");
        host_lbl.set_halign(gtk4::Align::Start);
        container.append(&host_lbl);

        let hostname_entry = Entry::new();
        hostname_entry.add_css_class("sidebar-search-entry");
        hostname_entry.set_placeholder_text(Some(&trans("settings.new_hostname_placeholder")));
        container.append(&hostname_entry);

        // Sudo password field
        let pwd_lbl = Label::new(Some(&trans("settings.sudo_password_placeholder")));
        pwd_lbl.add_css_class("settings-row-desc");
        pwd_lbl.set_halign(gtk4::Align::Start);
        container.append(&pwd_lbl);

        let pwd_entry = PasswordEntry::new();
        pwd_entry.add_css_class("sidebar-search-entry");
        pwd_entry.set_placeholder_text(Some(&trans("settings.sudo_password_placeholder")));
        container.append(&pwd_entry);

        // Action buttons
        let actions_box = Box::new(Orientation::Horizontal, 8);
        actions_box.set_halign(gtk4::Align::End);
        actions_box.set_margin_top(6);

        let cancel_btn = Button::with_label(&trans("common.cancel"));
        cancel_btn.add_css_class("connect-pill-btn");
        cancel_btn.set_cursor_from_name(Some("pointer"));

        let confirm_btn = Button::with_label(&trans("settings.save"));
        confirm_btn.add_css_class("suggested-action");
        confirm_btn.set_cursor_from_name(Some("pointer"));

        actions_box.append(&cancel_btn);
        actions_box.append(&confirm_btn);
        container.append(&actions_box);

        let dialog = Self {
            container,
            hostname_entry,
            pwd_entry,
            error_lbl,
            cancel_btn,
            confirm_btn,
        };

        // Wire cancel
        let box_c = dialog.container.clone();
        let host_c = dialog.hostname_entry.clone();
        let pwd_c = dialog.pwd_entry.clone();
        let err_c = dialog.error_lbl.clone();
        dialog.cancel_btn.connect_clicked(move |_| {
            host_c.set_text("");
            pwd_c.set_text("");
            err_c.set_visible(false);
            box_c.set_visible(false);
        });

        dialog
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
        let container = Box::new(Orientation::Vertical, 14);
        container.add_css_class("auth-dialog-card");
        container.set_halign(gtk4::Align::Center);
        container.set_valign(gtk4::Align::Center);
        container.set_visible(false);
        container.set_width_request(380);

        // Header
        let header_box = Box::new(Orientation::Horizontal, 12);
        let lock_icon = crate::ui::icon::get_icon("lock", 24);
        lock_icon.set_pixel_size(24);
        lock_icon.set_valign(gtk4::Align::Center);
        header_box.append(&lock_icon);

        let title_box = Box::new(Orientation::Vertical, 2);
        let title_lbl = Label::new(Some(&trans("settings.dialog_change_password_title")));
        title_lbl.add_css_class("settings-row-title");
        title_lbl.set_halign(gtk4::Align::Start);

        let sub_lbl = Label::new(Some(&trans("settings.dialog_change_password_sub")));
        sub_lbl.add_css_class("settings-row-desc");
        sub_lbl.set_halign(gtk4::Align::Start);
        sub_lbl.set_wrap(true);

        title_box.append(&title_lbl);
        title_box.append(&sub_lbl);
        header_box.append(&title_box);
        container.append(&header_box);

        // Error message
        let error_lbl = Label::new(None);
        error_lbl.add_css_class("dialog-error-text");
        error_lbl.set_halign(gtk4::Align::Start);
        error_lbl.set_visible(false);
        container.append(&error_lbl);

        // Current password field
        let cur_lbl = Label::new(Some(&trans("settings.current_password")));
        cur_lbl.add_css_class("settings-row-desc");
        cur_lbl.set_halign(gtk4::Align::Start);
        container.append(&cur_lbl);

        let current_pwd_entry = PasswordEntry::new();
        current_pwd_entry.add_css_class("sidebar-search-entry");
        current_pwd_entry.set_placeholder_text(Some(&trans("settings.current_password")));
        container.append(&current_pwd_entry);

        // New password field
        let new_lbl = Label::new(Some(&trans("settings.new_password")));
        new_lbl.add_css_class("settings-row-desc");
        new_lbl.set_halign(gtk4::Align::Start);
        container.append(&new_lbl);

        let new_pwd_entry = PasswordEntry::new();
        new_pwd_entry.add_css_class("sidebar-search-entry");
        new_pwd_entry.set_placeholder_text(Some(&trans("settings.new_password")));
        container.append(&new_pwd_entry);

        // Confirm new password field
        let conf_lbl = Label::new(Some(&trans("settings.confirm_password")));
        conf_lbl.add_css_class("settings-row-desc");
        conf_lbl.set_halign(gtk4::Align::Start);
        container.append(&conf_lbl);

        let confirm_pwd_entry = PasswordEntry::new();
        confirm_pwd_entry.add_css_class("sidebar-search-entry");
        confirm_pwd_entry.set_placeholder_text(Some(&trans("settings.confirm_password")));
        container.append(&confirm_pwd_entry);

        // Action buttons
        let actions_box = Box::new(Orientation::Horizontal, 8);
        actions_box.set_halign(gtk4::Align::End);
        actions_box.set_margin_top(6);

        let cancel_btn = Button::with_label(&trans("common.cancel"));
        cancel_btn.add_css_class("connect-pill-btn");
        cancel_btn.set_cursor_from_name(Some("pointer"));

        let confirm_btn = Button::with_label(&trans("settings.change_password"));
        confirm_btn.add_css_class("suggested-action");
        confirm_btn.set_cursor_from_name(Some("pointer"));

        actions_box.append(&cancel_btn);
        actions_box.append(&confirm_btn);
        container.append(&actions_box);

        let dialog = Self {
            container,
            current_pwd_entry,
            new_pwd_entry,
            confirm_pwd_entry,
            error_lbl,
            cancel_btn,
            confirm_btn,
        };

        // Wire cancel
        let box_c = dialog.container.clone();
        let cur_c = dialog.current_pwd_entry.clone();
        let new_c = dialog.new_pwd_entry.clone();
        let conf_c = dialog.confirm_pwd_entry.clone();
        let err_c = dialog.error_lbl.clone();
        dialog.cancel_btn.connect_clicked(move |_| {
            cur_c.set_text("");
            new_c.set_text("");
            conf_c.set_text("");
            err_c.set_visible(false);
            box_c.set_visible(false);
        });

        dialog
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
