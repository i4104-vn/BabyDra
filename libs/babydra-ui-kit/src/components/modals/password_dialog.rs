//! Generic password prompt dialog.

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Box, Button, PasswordEntry};
use std::boxed::Box as StdBox;
use std::rc::Rc;

use crate::components::modals::dialog_builder::{
    ActionButton, BadgeVariant, ButtonVariant, ModernDialogBuilder, create_modern_password_entry,
};

#[derive(Clone)]
pub struct PasswordDialog {
    pub container: Box,
    pub password_entry: PasswordEntry,
    pub confirm_btn: Button,
    pub cancel_btn: Button,
}

impl PasswordDialog {
    pub fn new(title: &str, subtitle: &str) -> Self {
        let builder = ModernDialogBuilder::new(420)
            .with_badge("lock", BadgeVariant::Primary)
            .with_title(title)
            .with_subtitle(subtitle);

        let dialog = builder.build();

        let password_entry = create_modern_password_entry(&trans("common.password_placeholder"));
        dialog.add_child(&password_entry);

        let cancel_btn = Button::with_label(&trans("common.cancel"));
        let confirm_btn = Button::with_label(&trans("common.confirm"));

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
                label: trans("common.confirm"),
                variant: ButtonVariant::Primary,
                callback: StdBox::new(|| {}),
            },
        ]);

        let s = Self {
            container: dialog.container().clone(),
            password_entry,
            confirm_btn,
            cancel_btn,
        };

        // Wire cancel button
        let entry_c = s.password_entry.clone();
        let box_c = s.container.clone();
        s.cancel_btn.connect_clicked(move |_| {
            entry_c.set_text("");
            box_c.set_visible(false);
        });

        s
    }

    pub fn show_for(&self, _prompt_title: &str, _prompt_sub: &str) {
        // Note: Title/subtitle are set at build time in new implementation.
        // For dynamic titles, we'd need to expose them from the dialog.
        // For now, just show the dialog.
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
        let callback_rc = Rc::new(callback);

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