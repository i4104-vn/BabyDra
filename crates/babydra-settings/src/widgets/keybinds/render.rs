use crate::widgets::state::KeybindsWidget;
use babydra_core::models::shortcut::Shortcut;
use gtk4::prelude::*;
use gtk4::{glib, Box, Button, Entry, Label, Orientation, Window};
use std::cell::RefCell;
use std::rc::Rc;

/// CSS class marking editable data rows of the shortcuts table.
pub const DATA_ROW_CSS_CLASS: &str = "keybind-data-row";

/// Canonical modifier letters in serialization order (labwc syntax).
const MOD_LETTERS: [&str; 4] = ["W", "C", "A", "S"];
/// Display names matching [`MOD_LETTERS`].
const MOD_NAMES: [&str; 4] = ["Super", "Ctrl", "Alt", "Shift"];

/// Build the shortcuts page: header + table of shortcut rows.
pub fn build(shortcuts: &[Shortcut]) -> KeybindsWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // Header
    let header_box = Box::new(Orientation::Horizontal, 12);
    let title_label = Label::new(Some(&babydra_core::i18n::trans(
        "settings.keybinds_title_page",
    )));
    title_label.add_css_class("settings-page-title");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);

    let refresh_btn = Button::with_label(&babydra_core::i18n::trans("settings.refresh"));
    refresh_btn.add_css_class("connect-pill-btn");

    let add_btn = Button::with_label(&babydra_core::i18n::trans("settings.startup_add_new"));
    add_btn.add_css_class("connect-pill-btn");

    let save_btn = Button::with_label(&babydra_core::i18n::trans("settings.save_changes"));
    save_btn.add_css_class("suggested-action");

    header_box.append(&title_label);
    header_box.append(&refresh_btn);
    header_box.append(&add_btn);
    header_box.append(&save_btn);
    container.append(&header_box);

    let glass_card = Box::new(Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");
    glass_card.set_vexpand(true);
    glass_card.set_valign(gtk4::Align::Fill);

    // Table Container Card
    let table_card = Box::new(Orientation::Vertical, 4);
    table_card.add_css_class("keybinds-table");

    for sc in shortcuts {
        let row = create_shortcut_row(sc, table_card.clone());
        table_card.append(&row);
    }

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&table_card));

    glass_card.append(&scroll);
    container.append(&glass_card);

    KeybindsWidget {
        container,
        table_box: table_card,
        add_btn,
        refresh_btn,
        save_btn,
    }
}

/// Creates an editable row: shortcut button + command entry.
///
/// The shortcut button shows the current combo and opens a capture dialog
/// when clicked — the user presses the desired key combination instead of
/// typing it.
///
/// Children are laid out in a fixed order so `read_combo` / `read_command`
/// can extract their values when saving: `[combo_btn, command_entry,
/// delete_btn]`.
pub fn create_shortcut_row(sc: &Shortcut, parent: Box) -> Box {
    let row = Box::new(Orientation::Horizontal, 12);
    row.add_css_class("settings-card-row");
    row.add_css_class(DATA_ROW_CSS_CLASS);

    let combo_btn = Button::new();
    combo_btn.set_width_request(170);
    combo_btn.add_css_class("shortcut-combo-btn");
    combo_btn.set_halign(gtk4::Align::Start);
    set_combo_button(&combo_btn, &sc.modifiers, &sc.key);
    {
        let combo_btn = combo_btn.clone();
        combo_btn.connect_clicked(move |btn| {
            let Some(parent_window) = btn.root().and_then(|root| root.downcast::<Window>().ok())
            else {
                return;
            };
            show_capture_dialog(&parent_window, btn);
        });
    }

    let cmd_entry = Entry::new();
    cmd_entry.set_text(&sc.command);
    cmd_entry.set_hexpand(true);
    cmd_entry.set_placeholder_text(Some("~/.local/bin/babydra-app"));
    cmd_entry.add_css_class("sidebar-search-entry");

    let delete_btn = Button::new();
    delete_btn.add_css_class("icon-btn");
    delete_btn.add_css_class("circular");
    delete_btn.add_css_class("delete-btn");
    delete_btn.set_valign(gtk4::Align::Center);
    let del_icon = babydra_ui_kit::ui::icon::get_icon("edit-delete", 16);
    del_icon.set_pixel_size(16);
    delete_btn.set_child(Some(&del_icon));

    let row_copy = row.clone();
    delete_btn.connect_clicked(move |_| {
        parent.remove(&row_copy);
    });

    row.append(&combo_btn);
    row.append(&cmd_entry);
    row.append(&delete_btn);

    row
}

/// Opens a modal dialog asking the user to press a key combination.
///
/// While the dialog is open every key press is captured (modifiers are
/// tracked via the event state), shown live in the dialog, and applied to
/// `combo_btn` when the user confirms.  `Escape` closes without changes.
fn show_capture_dialog(parent: &Window, combo_btn: &Button) {
    // Suppress global shortcuts while recording so pressed keys never fire
    // their real commands; resumed when the dialog closes for any reason.
    babydra_core::services::system::keymap::pause_shortcuts();

    let window = Window::builder()
        .title(&babydra_core::i18n::trans("settings.keybinds_title_page"))
        .transient_for(parent)
        .modal(true)
        .resizable(false)
        .default_width(360)
        .default_height(200)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 16);
    vbox.set_margin_top(24);
    vbox.set_margin_bottom(24);
    vbox.set_margin_start(24);
    vbox.set_margin_end(24);
    vbox.set_valign(gtk4::Align::Center);
    vbox.add_css_class("explore-dialog-box");
    window.set_child(Some(&vbox));

    let lbl_desc = Label::builder()
        .label(&babydra_core::i18n::trans("settings.keybind_press"))
        .halign(gtk4::Align::Center)
        .justify(gtk4::Justification::Center)
        .build();
    lbl_desc.add_css_class("settings-row-title");
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

    let btn_save = Button::with_label(&babydra_core::i18n::trans("settings.save"));
    btn_save.add_css_class("baby-button");

    let btn_cancel = Button::with_label(&babydra_core::i18n::trans("settings.keybind_cancel"));
    btn_cancel.add_css_class("baby-button");

    bbox.append(&btn_save);
    bbox.append(&btn_cancel);

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

/// Updates the shortcut button label from a `W-C-A-S` modifier string + key.
///
/// An empty key shows the "click to record" placeholder.
fn set_combo_button(btn: &Button, modifiers: &str, key: &str) {
    if key.is_empty() {
        btn.set_label(&babydra_core::i18n::trans("settings.keybind_record"));
        btn.set_tooltip_text(None::<&str>);
    } else {
        btn.set_label(&pretty_combo(modifiers, key));
        btn.set_tooltip_text(Some(&combo_string(modifiers, key)));
    }
}

/// Joins modifier letters and a key into the labwc combo string, e.g.
/// `"W-Tab"`.
fn combo_string(modifiers: &str, key: &str) -> String {
    if modifiers.is_empty() {
        key.to_string()
    } else {
        format!("{}-{}", modifiers, key)
    }
}

/// Formats a combo for display, e.g. `"Super + Tab"`.
fn pretty_combo(modifiers: &str, key: &str) -> String {
    let mut parts: Vec<String> = modifiers
        .split('-')
        .filter(|m| !m.is_empty())
        .filter_map(|m| {
            MOD_LETTERS
                .iter()
                .position(|l| *l == m)
                .map(|i| MOD_NAMES[i].to_string())
        })
        .collect();
    parts.push(pretty_key(key));
    parts.join(" + ")
}

/// Human friendly key name: single letters upper-cased, common aliases.
fn pretty_key(key: &str) -> String {
    match key {
        "Return" => "Enter".to_string(),
        "space" => "Space".to_string(),
        _ => {
            let mut chars = key.chars();
            if let (Some(c), None) = (chars.next(), chars.next()) {
                c.to_uppercase().to_string()
            } else {
                key.to_string()
            }
        }
    }
}

/// Extracts `(modifiers, key)` from a data row created by
/// [`create_shortcut_row`].  Returns `None` while the combo is unset.
///
/// The combo is recovered by parsing the button label back into canonical
/// `W-C-A-S` letters, so saving stays independent of display order.
pub fn read_combo(row: &Box) -> Option<(String, String)> {
    let children = row.observe_children();
    let combo_btn = children.item(0)?.downcast::<Button>().ok()?;
    let tooltip = combo_btn.tooltip_text()?;
    let combo = tooltip.to_string();
    let (modifiers, key) = match combo.rsplit_once('-') {
        Some((mods, key)) => (mods, key),
        None => ("", combo.as_str()),
    };

    let valid = modifiers
        .split('-')
        .filter(|m| !m.is_empty())
        .all(|m| MOD_LETTERS.contains(&m));
    if !valid || key.is_empty() {
        return None;
    }

    Some((modifiers.to_string(), key.to_string()))
}

/// Extracts the shell command from a data row created by
/// [`create_shortcut_row`].
pub fn read_command(row: &Box) -> Option<String> {
    let children = row.observe_children();
    let cmd_entry = children.item(1)?.downcast::<Entry>().ok()?;
    let command = cmd_entry.text().trim().to_string();
    if command.is_empty() {
        None
    } else {
        Some(command)
    }
}

/// Reads held modifiers from the event state into canonical `W-C-A-S` order.
fn state_modifiers(state: &gtk4::gdk::ModifierType) -> String {
    let checks = [
        (gtk4::gdk::ModifierType::SUPER_MASK, "W"),
        (gtk4::gdk::ModifierType::CONTROL_MASK, "C"),
        (gtk4::gdk::ModifierType::ALT_MASK, "A"),
        (gtk4::gdk::ModifierType::SHIFT_MASK, "S"),
    ];
    checks
        .iter()
        .filter(|(mask, _)| state.contains(*mask))
        .map(|(_, letter)| letter.to_string())
        .collect::<Vec<_>>()
        .join("-")
}
