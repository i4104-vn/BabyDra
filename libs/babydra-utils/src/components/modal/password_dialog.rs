use babydra_common::i18n::t;
use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, PasswordEntry};

#[derive(Clone)]
pub struct PasswordDialog {
    pub container: Box,
    pub password_entry: PasswordEntry,
    pub confirm_btn: Button,
    pub cancel_btn: Button,
    pub title_lbl: Label,
    pub sub_lbl: Label,
}

impl PasswordDialog {
    pub fn new(title: &str, subtitle: &str) -> Self {
        let container = Box::new(Orientation::Vertical, 16);
        container.add_css_class("auth-dialog-card");
        container.set_halign(gtk4::Align::Center);
        container.set_valign(gtk4::Align::Center);
        container.set_visible(false);

        let header_box = Box::new(Orientation::Horizontal, 12);
        let lock_icon = crate::ui::icon::get_icon("lock", 24);
        lock_icon.set_pixel_size(24);
        header_box.append(&lock_icon);

        let title_box = Box::new(Orientation::Vertical, 2);
        let title_lbl = Label::new(Some(title));
        title_lbl.add_css_class("settings-row-title");
        title_lbl.set_halign(gtk4::Align::Start);

        let sub_lbl = Label::new(Some(subtitle));
        sub_lbl.add_css_class("settings-row-desc");
        sub_lbl.set_halign(gtk4::Align::Start);

        title_box.append(&title_lbl);
        title_box.append(&sub_lbl);
        header_box.append(&title_box);
        container.append(&header_box);

        let password_entry = PasswordEntry::new();
        password_entry.add_css_class("sidebar-search-entry");
        password_entry.set_placeholder_text(Some(&t("common.password_placeholder")));
        container.append(&password_entry);

        let actions_box = Box::new(Orientation::Horizontal, 8);
        actions_box.set_halign(gtk4::Align::End);

        let cancel_btn = Button::with_label(&t("common.cancel"));
        cancel_btn.add_css_class("connect-pill-btn");
        cancel_btn.set_cursor_from_name(Some("pointer"));

        let confirm_btn = Button::with_label(&t("common.confirm"));
        confirm_btn.add_css_class("suggested-action");
        confirm_btn.set_cursor_from_name(Some("pointer"));

        actions_box.append(&cancel_btn);
        actions_box.append(&confirm_btn);
        container.append(&actions_box);

        let dialog = Self {
            container,
            password_entry,
            confirm_btn,
            cancel_btn,
            title_lbl,
            sub_lbl,
        };

        // Wire cancel button
        let entry_c = dialog.password_entry.clone();
        let box_c = dialog.container.clone();
        dialog.cancel_btn.connect_clicked(move |_| {
            entry_c.set_text("");
            box_c.set_visible(false);
        });

        dialog
    }

    pub fn show_for(&self, prompt_title: &str, prompt_sub: &str) {
        self.title_lbl.set_text(prompt_title);
        self.sub_lbl.set_text(prompt_sub);
        self.password_entry.set_text("");
        self.container.set_visible(true);
        self.password_entry.grab_focus();
    }

    pub fn hide(&self) {
        self.password_entry.set_text("");
        self.container.set_visible(false);
    }

    pub fn connect_submit<F: Fn(Option<String>) + 'static>(&self, callback: F) {
        let entry = self.password_entry.clone();
        let container = self.container.clone();
        let callback_rc = std::rc::Rc::new(callback);

        let cb1 = callback_rc.clone();
        let entry1 = entry.clone();
        let container1 = container.clone();
        self.confirm_btn.connect_clicked(move |_| {
            let pwd = entry1.text().to_string();
            entry1.set_text("");
            container1.set_visible(false);
            let opt = if pwd.trim().is_empty() {
                None
            } else {
                Some(pwd)
            };
            cb1(opt);
        });

        let cb2 = callback_rc;
        let entry2 = entry;
        let container2 = container;
        self.password_entry.connect_activate(move |_| {
            let pwd = entry2.text().to_string();
            entry2.set_text("");
            container2.set_visible(false);
            let opt = if pwd.trim().is_empty() {
                None
            } else {
                Some(pwd)
            };
            cb2(opt);
        });
    }
}
