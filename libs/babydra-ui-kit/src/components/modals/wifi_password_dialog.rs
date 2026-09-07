//! WiFi Password Dialog

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, Orientation, PasswordEntry};
use std::boxed::Box as StdBox;
use std::rc::Rc;

use crate::components::modals::dialog_builder::{
    ActionButton, BadgeVariant, ButtonVariant, ModernDialogBuilder, create_error_label,
    create_form_label, create_modern_entry, create_modern_password_entry,
};

pub struct WifiPasswordDialog {
    pub container: Box,
    pub ssid_lbl: Label,
    pub sub_lbl: Label,
    pub username_box: Box,
    pub username_entry: Entry,
    pub password_entry: PasswordEntry,
    pub error_lbl: Label,
    pub cancel_btn: Button,
    pub connect_btn: Button,
}

impl WifiPasswordDialog {
    pub fn new() -> Self {
        let builder = ModernDialogBuilder::new(420)
            .with_badge("wifi", BadgeVariant::Primary)
            .with_title(&trans("wifi.connect_to").replace("{}", "Wi-Fi"))
            .with_subtitle(&trans("wifi.requires_password"))
            .with_card_spacing(16);

        let dialog = builder.build();

        // Username Entry (hidden unless 802.1X Enterprise)
        let username_box = Box::new(Orientation::Vertical, 4);
        username_box.set_visible(false);

        let user_lbl = create_form_label(&trans("wifi.username_identity"));
        let username_entry = create_modern_entry(&trans("wifi.enter_username"));

        username_box.append(&user_lbl);
        username_box.append(&username_entry);
        dialog.add_child(&username_box);

        // Password Entry
        let pwd_lbl = create_form_label(&trans("common.password"));
        let password_entry = create_modern_password_entry(&trans("wifi.enter_password"));

        dialog.add_child(&pwd_lbl);
        dialog.add_child(&password_entry);

        // Error message label
        let error_lbl = create_error_label();
        error_lbl.add_css_class("wifi-error-hint");
        dialog.add_child(&error_lbl);

        // Get title/subtitle labels for dynamic updates
        let ssid_lbl = dialog.title_label().clone();
        let sub_lbl = dialog.subtitle_label().cloned().unwrap_or_else(|| Label::new(None));

        let cancel_btn = Button::with_label(&trans("common.cancel"));
        let connect_btn = Button::with_label(&trans("common.connect"));

        dialog.add_actions(vec![
            ActionButton {
                label: trans("common.cancel"),
                variant: ButtonVariant::Cancel,
                callback: StdBox::new({
                    let entry = password_entry.clone();
                    let container = dialog.container().clone();
                    move || {
                        entry.set_text("");
                        container.set_visible(false);
                    }
                }),
            },
            ActionButton {
                label: trans("common.connect"),
                variant: ButtonVariant::Primary,
                callback: StdBox::new(|| {}),
            },
        ]);

        let s = Self {
            container: dialog.container().clone(),
            ssid_lbl,
            sub_lbl,
            username_box,
            username_entry,
            password_entry,
            error_lbl,
            cancel_btn,
            connect_btn,
        };

        let entry_c = s.password_entry.clone();
        let box_c = s.container.clone();
        s.cancel_btn.connect_clicked(move |_| {
            entry_c.set_text("");
            box_c.set_visible(false);
        });

        s
    }

    pub fn show_for(&self, ssid: &str, security: &str) {
        self.ssid_lbl
            .set_text(&trans("wifi.connect_to").replace("{}", ssid));
        self.password_entry.set_text("");
        self.username_entry.set_text("");
        self.error_lbl.set_visible(false);

        if security == "8021x" {
            self.sub_lbl.set_text(&trans("wifi.enterprise_requires"));
            self.username_box.set_visible(true);
            self.username_entry.grab_focus();
        } else {
            self.sub_lbl.set_text(&trans("wifi.requires_password"));
            self.username_box.set_visible(false);
            self.password_entry.grab_focus();
        }

        self.container.set_visible(true);
    }

    pub fn set_error(&self, msg: Option<&str>) {
        if let Some(err) = msg {
            self.error_lbl.set_text(err);
            self.error_lbl.set_visible(true);
        } else {
            self.error_lbl.set_visible(false);
        }
    }

    pub fn hide(&self) {
        self.password_entry.set_text("");
        self.username_entry.set_text("");
        self.error_lbl.set_visible(false);
        self.container.set_visible(false);
    }

    pub fn connect_submit<F: Fn(String, Option<String>) + 'static>(&self, callback: F) {
        let pwd_entry = self.password_entry.clone();
        let user_entry = self.username_entry.clone();
        let is_user_vis = self.username_box.clone();
        let container = self.container.clone();
        let cb_rc = Rc::new(callback);

        let cb1 = cb_rc.clone();
        let p1 = pwd_entry.clone();
        let u1 = user_entry.clone();
        let v1 = is_user_vis.clone();
        let c1 = container.clone();
        self.connect_btn.connect_clicked(move |_| {
            let pwd = p1.text().to_string();
            let user = if v1.is_visible() && !u1.text().to_string().trim().is_empty() {
                Some(u1.text().to_string())
            } else {
                None
            };
            p1.set_text("");
            u1.set_text("");
            c1.set_visible(false);
            cb1(pwd, user);
        });

        let cb2 = cb_rc;
        let p2 = pwd_entry;
        let u2 = user_entry;
        let v2 = is_user_vis;
        let c2 = container;
        self.password_entry.connect_activate(move |_| {
            let pwd = p2.text().to_string();
            let user = if v2.is_visible() && !u2.text().to_string().trim().is_empty() {
                Some(u2.text().to_string())
            } else {
                None
            };
            p2.set_text("");
            u2.set_text("");
            c2.set_visible(false);
            cb2(pwd, user);
        });
    }
}