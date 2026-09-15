//! Key capture modal dialog for recording new shortcut combinations.

pub mod format;

use format::{combo_string, pretty_combo, state_modifiers};
use gtk4::prelude::*;
use gtk4::{glib, Box, Button, Label, Orientation, Window};
use std::cell::RefCell;
use std::rc::Rc;

/// Opens a modal dialog asking the user to press a key combination.
///
/// While the dialog is open every key press is captured (modifiers are
/// tracked via the event state), shown live in the dialog, and applied to
/// `combo_btn` when the user confirms. `Escape` closes without changes.
pub fn show_capture_dialog(parent: &Window, combo_btn: &Button) {
    // Suppress global shortcuts while recording so pressed keys never fire
    // their real commands; resumed when the dialog closes for any reason.
    babydra_core::services::system::keymap::pause_shortcuts();

    let window = Window::builder()
        .title(&babydra_core::i18n::trans("settings.keybinds_title_page"))
        .transient_for(parent)
        .modal(true)
        .resizable(false)
        .default_width(380)
        .default_height(230)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 14);
    vbox.set_margin_top(22);
    vbox.set_margin_bottom(22);
    vbox.set_margin_start(24);
    vbox.set_margin_end(24);
    vbox.set_valign(gtk4::Align::Center);
    vbox.add_css_class("explore-dialog-box");
    window.set_child(Some(&vbox));

    let badge = Box::builder()
        .width_request(46)
        .height_request(46)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .css_classes(vec![
            "modern-dialog-badge".to_string(),
            "badge-primary".to_string(),
        ])
        .build();
    let icon_badge = babydra_ui_kit::ui::icon::get_icon("terminal", 22);
    icon_badge.set_pixel_size(22);
    icon_badge.set_vexpand(true);
    icon_badge.set_hexpand(true);
    icon_badge.set_valign(gtk4::Align::Center);
    icon_badge.set_halign(gtk4::Align::Center);
    badge.append(&icon_badge);
    vbox.append(&badge);

    let lbl_desc = Label::builder()
        .label(&babydra_core::i18n::trans("settings.keybind_press"))
        .halign(gtk4::Align::Center)
        .justify(gtk4::Justification::Center)
        .build();
    lbl_desc.add_css_class("modern-dialog-title");
    vbox.append(&lbl_desc);

    let lbl_shortcut = Label::new(Some("…"));
    lbl_shortcut.add_css_class("keybind-pill");
    lbl_shortcut.add_css_class("settings-item-command");
    vbox.append(&lbl_shortcut);

    let captured = Rc::new(RefCell::new(None::<(String, String)>));

    let key_controller = gtk4::EventControllerKey::new();
    {
        let lbl_shortcut = lbl_shortcut.clone();
        let captured = captured.clone();
        let window = window.clone();
        key_controller.connect_key_pressed(move |_, keyval, _, state| {
            if keyval == gtk4::gdk::Key::Escape {
                window.close();
                return glib::Propagation::Stop;
            }

            // Ignore bare modifier presses so combos can be built up.
            let is_modifier_only = matches!(
                keyval,
                gtk4::gdk::Key::Super_L
                    | gtk4::gdk::Key::Super_R
                    | gtk4::gdk::Key::Control_L
                    | gtk4::gdk::Key::Control_R
                    | gtk4::gdk::Key::Alt_L
                    | gtk4::gdk::Key::Alt_R
                    | gtk4::gdk::Key::Shift_L
                    | gtk4::gdk::Key::Shift_R
            );
            if is_modifier_only {
                return glib::Propagation::Stop;
            }

            let modifiers = state_modifiers(&state);
            let Some(key) = keyval.name() else {
                return glib::Propagation::Stop;
            };
            let key = key.trim_end().to_string();

            lbl_shortcut.set_text(&pretty_combo(&modifiers, &key));
            captured.replace(Some((modifiers, key)));
            glib::Propagation::Stop
        });
    }
    window.add_controller(key_controller);

    // Resume global shortcuts whenever the dialog closes (Save, Cancel,
    // Escape or the window close button).
    window.connect_close_request(|_| {
        babydra_core::services::system::keymap::resume_shortcuts();
        glib::Propagation::Proceed
    });

    let bbox = Box::new(Orientation::Horizontal, 12);
    bbox.set_halign(gtk4::Align::Center);
    vbox.append(&bbox);

    let btn_cancel = Button::with_label(&babydra_core::i18n::trans("settings.keybind_cancel"));
    btn_cancel.add_css_class("modern-dialog-cancel-btn");
    btn_cancel.set_cursor_from_name(Some("pointer"));

    let btn_save = Button::with_label(&babydra_core::i18n::trans("settings.save"));
    btn_save.add_css_class("modern-dialog-primary-btn");
    btn_save.set_cursor_from_name(Some("pointer"));

    bbox.append(&btn_cancel);
    bbox.append(&btn_save);

    let window_cancel = window.clone();
    btn_cancel.connect_clicked(move |_| {
        window_cancel.close();
    });

    {
        let captured = captured.clone();
        let combo_btn = combo_btn.clone();
        let window_save = window.clone();
        btn_save.connect_clicked(move |_| {
            if let Some((modifiers, key)) = captured.borrow().clone() {
                combo_btn.set_label(&pretty_combo(&modifiers, &key));
                combo_btn.set_tooltip_text(Some(&combo_string(&modifiers, &key)));
            }
            window_save.close();
        });
    }

    window.present();
}
