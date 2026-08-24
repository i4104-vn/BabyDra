//! Shared explore-dialog scaffold. Every explore dialog builds its window
//! frame, content box, hidden error label and action row from here instead of
//! hand-rolling the same widget tree.

use gtk4::prelude::*;
use gtk4::{Align, Box, Button, Entry, Label, Orientation, Window};

pub struct DialogShell {
    pub window: Window,
    pub vbox: Box,
}

impl DialogShell {
    pub fn new(
        title: &str,
        width: i32,
        height: i32,
        spacing: i32,
        parent: Option<&impl IsA<Window>>,
    ) -> Self {
        let window = Window::builder()
            .title(title)
            .icon_name("babydra")
            .modal(true)
            .resizable(false)
            .default_width(width)
            .default_height(height)
            .css_classes(vec!["explore-dialog".to_string()])
            .build();

        if let Some(p) = parent {
            window.set_transient_for(Some(p));
        }

        let vbox = Box::new(Orientation::Vertical, spacing);
        vbox.add_css_class("explore-dialog-box");
        vbox.set_margin_top(16);
        vbox.set_margin_bottom(16);
        vbox.set_margin_start(16);
        vbox.set_margin_end(16);
        window.set_child(Some(&vbox));

        Self { window, vbox }
    }

    /// Appends a left-aligned text label.
    pub fn add_label(&self, text: &str) -> Label {
        let lbl = Label::builder().label(text).halign(Align::Start).build();
        self.vbox.append(&lbl);
        lbl
    }

    /// Appends an expanding entry, optionally pre-filled and/or password-masked.
    pub fn add_entry(&self, initial: Option<&str>, password: bool) -> Entry {
        let entry = Entry::new();
        if let Some(text) = initial {
            entry.set_text(text);
        }
        entry.set_visibility(!password);
        entry.set_hexpand(true);
        self.vbox.append(&entry);
        entry
    }

    /// Appends the hidden error label used together with [`Self::show_error`].
    pub fn add_error_label(&self) -> Label {
        let lbl = Label::builder()
            .halign(Align::Start)
            .visible(false)
            .css_classes(vec!["dialog-error-text".to_string()])
            .wrap(true)
            .max_width_chars(35)
            .build();
        self.vbox.append(&lbl);
        lbl
    }

    /// Appends a markup-capable label for colored status/error messages.
    pub fn add_markup_label(&self) -> Label {
        let lbl = Label::builder()
            .halign(Align::Start)
            .use_markup(true)
            .build();
        self.vbox.append(&lbl);
        lbl
    }

    /// Appends the right-aligned action button row.
    pub fn add_button_row(&self) -> Box {
        let bbox = Box::new(Orientation::Horizontal, 8);
        bbox.set_halign(Align::End);
        self.vbox.append(&bbox);
        bbox
    }

    /// Shows `text` in the given error label and marks the entry as errored.
    pub fn show_error(lbl_error: &Label, entry: &Entry, text: &str) {
        lbl_error.set_text(text);
        lbl_error.set_visible(true);
        entry.add_css_class("error-entry");
    }

    /// Hides `lbl_error` and clears the entry's error styling whenever edited.
    pub fn wire_error_clear(lbl_error: &Label, entry: &Entry) {
        let lbl_err = lbl_error.clone();
        let entry_c = entry.clone();
        entry.connect_changed(move |_| {
            if lbl_err.is_visible() {
                lbl_err.set_visible(false);
                entry_c.remove_css_class("error-entry");
            }
        });
    }

    /// Adds a Cancel button that closes the window when clicked.
    pub fn cancel_button(&self, into: &Box) -> Button {
        let btn_cancel = Button::with_label(&babydra_core::i18n::trans("explore.settings_cancel"));
        let win = self.window.clone();
        btn_cancel.connect_clicked(move |_| {
            win.close();
        });
        into.append(&btn_cancel);
        btn_cancel
    }

    /// Adds a suggested-action button with the given label.
    pub fn action_button(&self, into: &Box, label: &str) -> Button {
        let btn = Button::builder()
            .label(label)
            .css_classes(vec!["suggested-action".to_string()])
            .build();
        into.append(&btn);
        btn
    }

    /// Makes Enter in `entry` trigger `btn` and focuses the entry on present.
    pub fn finish(self, entry: &Entry, btn: &Button) {
        let btn_c = btn.clone();
        entry.connect_activate(move |_| {
            btn_c.emit_clicked();
        });
        self.window.present();
        entry.grab_focus();
    }
}
