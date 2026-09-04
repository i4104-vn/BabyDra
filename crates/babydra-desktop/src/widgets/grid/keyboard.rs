//! Desktop keyboard shortcuts controller matching babydra-explore shortcuts.

use crate::state::DesktopState;
use crate::widgets::icon::launch_entry;
use crate::widgets::selection::update_icon_sel;
use babydra_ui_kit::components::explore::context_menu::clipboard::{
    paste_from_clipboard, set_clipboard_files,
};
use babydra_ui_kit::components::explore::*;
use gtk4::prelude::*;
use gtk4::{EventControllerKey, Fixed};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// Parses a shortcut string like "Ctrl+Shift+T" or "Delete" into a (Key, ModifierType) pair.
pub fn parse_shortcut(shortcut_str: &str) -> Option<(gtk4::gdk::Key, gtk4::gdk::ModifierType)> {
    let parts: Vec<&str> = shortcut_str.split('+').map(|s| s.trim()).collect();
    let mut modifiers = gtk4::gdk::ModifierType::empty();
    let mut key = None;

    for part in parts {
        let part_lower = part.to_lowercase();
        if part_lower == "ctrl" || part_lower == "control" {
            modifiers |= gtk4::gdk::ModifierType::CONTROL_MASK;
        } else if part_lower == "shift" {
            modifiers |= gtk4::gdk::ModifierType::SHIFT_MASK;
        } else if part_lower == "alt" {
            modifiers |= gtk4::gdk::ModifierType::ALT_MASK;
        } else {
            let k = match part_lower.as_str() {
                "f1" => Some(gtk4::gdk::Key::F1),
                "f2" => Some(gtk4::gdk::Key::F2),
                "f3" => Some(gtk4::gdk::Key::F3),
                "f4" => Some(gtk4::gdk::Key::F4),
                "f5" => Some(gtk4::gdk::Key::F5),
                "f6" => Some(gtk4::gdk::Key::F6),
                "f7" => Some(gtk4::gdk::Key::F7),
                "f8" => Some(gtk4::gdk::Key::F8),
                "f9" => Some(gtk4::gdk::Key::F9),
                "f10" => Some(gtk4::gdk::Key::F10),
                "f11" => Some(gtk4::gdk::Key::F11),
                "f12" => Some(gtk4::gdk::Key::F12),
                "enter" => Some(gtk4::gdk::Key::Return),
                "space" => Some(gtk4::gdk::Key::space),
                "escape" | "esc" => Some(gtk4::gdk::Key::Escape),
                "delete" | "del" => Some(gtk4::gdk::Key::Delete),
                "backspace" => Some(gtk4::gdk::Key::BackSpace),
                s if s.len() == 1 => {
                    let c = s.chars().next().unwrap();
                    gtk4::gdk::Key::from_name(c.to_string())
                }
                s => gtk4::gdk::Key::from_name(s),
            };
            if let Some(keyval) = k {
                key = Some(keyval);
            }
        }
    }

    key.map(|k| (k, modifiers))
}

fn matches_key(
    keyval: gtk4::gdk::Key,
    clean_mod: gtk4::gdk::ModifierType,
    target: (gtk4::gdk::Key, gtk4::gdk::ModifierType),
) -> bool {
    let (target_key, target_mod) = target;
    if clean_mod != target_mod {
        return false;
    }
    keyval == target_key
        || keyval.to_lower() == target_key.to_lower()
        || (target_key == gtk4::gdk::Key::Delete && keyval == gtk4::gdk::Key::KP_Delete)
        || (target_key == gtk4::gdk::Key::Return && keyval == gtk4::gdk::Key::KP_Enter)
}

pub fn wire_keyboard(
    parent_window: &gtk4::ApplicationWindow,
    fixed: &Fixed,
    state: Rc<RefCell<DesktopState>>,
    rubberband: gtk4::Box,
    refresh_fn: Rc<dyn Fn()>,
) {
    let key_controller = EventControllerKey::new();
    key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);

    let state_key = state.clone();
    let fixed_key = fixed.clone();
    let ref_cb_key = refresh_fn.clone();
    let rubberband_key = rubberband.clone();

    key_controller.connect_key_pressed(move |_, keyval, _, mod_state| {
        let clean_mod = mod_state
            & (gtk4::gdk::ModifierType::CONTROL_MASK
                | gtk4::gdk::ModifierType::SHIFT_MASK
                | gtk4::gdk::ModifierType::ALT_MASK);

        let cfg = babydra_core::load_explore_cfg();

        // 1. Permanent Delete (Hard delete / bypass Trash)
        let perm_del_target = parse_shortcut(&cfg.get_keybind("permanent_delete"))
            .unwrap_or((gtk4::gdk::Key::Delete, gtk4::gdk::ModifierType::SHIFT_MASK));
        let is_perm_del = matches_key(keyval, clean_mod, perm_del_target)
            || ((keyval == gtk4::gdk::Key::Delete || keyval == gtk4::gdk::Key::KP_Delete)
                && clean_mod == gtk4::gdk::ModifierType::SHIFT_MASK);

        if is_perm_del {
            let selected: Vec<PathBuf> = {
                let mut s = state_key.borrow_mut();
                let list = s.selected_paths.iter().cloned().collect();
                s.clear_selection();
                list
            };
            if !selected.is_empty() {
                update_icon_sel(&fixed_key, &state_key, &rubberband_key);
                let ref_cb = ref_cb_key.clone();
                glib::spawn_future_local(async move {
                    for path in selected {
                        let _ = babydra_core::delete_path(path).await;
                    }
                    ref_cb();
                });
            }
            return glib::Propagation::Stop;
        }

        // 2. Soft Delete (Send to Trash)
        let del_target = parse_shortcut(&cfg.get_keybind("delete"))
            .unwrap_or((gtk4::gdk::Key::Delete, gtk4::gdk::ModifierType::empty()));
        let is_del = matches_key(keyval, clean_mod, del_target)
            || ((keyval == gtk4::gdk::Key::Delete || keyval == gtk4::gdk::Key::KP_Delete)
                && clean_mod.is_empty());

        if is_del {
            let selected: Vec<PathBuf> = {
                let mut s = state_key.borrow_mut();
                let list = s.selected_paths.iter().cloned().collect();
                s.clear_selection();
                list
            };
            if !selected.is_empty() {
                update_icon_sel(&fixed_key, &state_key, &rubberband_key);
                let ref_cb = ref_cb_key.clone();
                glib::spawn_future_local(async move {
                    for path in selected {
                        let _ = babydra_core::send_to_trash(path).await;
                    }
                    ref_cb();
                });
            }
            return glib::Propagation::Stop;
        }

        // 3. Cut (Ctrl+X)
        let cut_target = parse_shortcut(&cfg.get_keybind("cut"))
            .unwrap_or((gtk4::gdk::Key::x, gtk4::gdk::ModifierType::CONTROL_MASK));
        let is_cut = matches_key(keyval, clean_mod, cut_target)
            || ((keyval == gtk4::gdk::Key::x || keyval == gtk4::gdk::Key::X)
                && clean_mod == gtk4::gdk::ModifierType::CONTROL_MASK);

        if is_cut {
            let selected: Vec<PathBuf> = state_key.borrow().selected_paths.iter().cloned().collect();
            if !selected.is_empty() {
                set_clipboard_files(&selected, true);
                CLIPBOARD.with(|cb| cb.replace(Some((selected.clone(), true))));
                apply_cut_everywhere(&selected);
            }
            return glib::Propagation::Stop;
        }

        // 4. Copy (Ctrl+C)
        let copy_target = parse_shortcut(&cfg.get_keybind("copy"))
            .unwrap_or((gtk4::gdk::Key::c, gtk4::gdk::ModifierType::CONTROL_MASK));
        let is_copy = matches_key(keyval, clean_mod, copy_target)
            || ((keyval == gtk4::gdk::Key::c || keyval == gtk4::gdk::Key::C)
                && clean_mod == gtk4::gdk::ModifierType::CONTROL_MASK);

        if is_copy {
            let selected: Vec<PathBuf> = state_key.borrow().selected_paths.iter().cloned().collect();
            if !selected.is_empty() {
                set_clipboard_files(&selected, false);
                CLIPBOARD.with(|cb| cb.replace(Some((selected.clone(), false))));
                apply_cut_everywhere(&[]);
            }
            return glib::Propagation::Stop;
        }

        // 5. Paste (Ctrl+V)
        let paste_target = parse_shortcut(&cfg.get_keybind("paste"))
            .unwrap_or((gtk4::gdk::Key::v, gtk4::gdk::ModifierType::CONTROL_MASK));
        let is_paste = matches_key(keyval, clean_mod, paste_target)
            || ((keyval == gtk4::gdk::Key::v || keyval == gtk4::gdk::Key::V)
                && clean_mod == gtk4::gdk::ModifierType::CONTROL_MASK);

        if is_paste {
            let ddir = DesktopState::desktop_dir();
            let nav_cb = crate::widgets::context_menu::refresh_nav_cb(ref_cb_key.clone());
            paste_from_clipboard(ddir.clone(), ddir, nav_cb);
            return glib::Propagation::Stop;
        }

        // 6. Select All (Ctrl+A)
        let sel_all_target = parse_shortcut(&cfg.get_keybind("select_all"))
            .unwrap_or((gtk4::gdk::Key::a, gtk4::gdk::ModifierType::CONTROL_MASK));
        let is_sel_all = matches_key(keyval, clean_mod, sel_all_target)
            || ((keyval == gtk4::gdk::Key::a || keyval == gtk4::gdk::Key::A)
                && clean_mod == gtk4::gdk::ModifierType::CONTROL_MASK);

        if is_sel_all {
            state_key.borrow_mut().select_all();
            update_icon_sel(&fixed_key, &state_key, &rubberband_key);
            return glib::Propagation::Stop;
        }

        // 7. Launch / Open (Return / Enter)
        if keyval == gtk4::gdk::Key::Return || keyval == gtk4::gdk::Key::KP_Enter {
            let state_ref = state_key.borrow();
            for entry in &state_ref.entries {
                if state_ref.is_selected(&entry.path) {
                    launch_entry(entry);
                }
            }
            return glib::Propagation::Stop;
        }

        // 8. Refresh (F5 / Ctrl+R)
        if keyval == gtk4::gdk::Key::F5
            || ((keyval == gtk4::gdk::Key::r || keyval == gtk4::gdk::Key::R)
                && clean_mod == gtk4::gdk::ModifierType::CONTROL_MASK)
        {
            ref_cb_key();
            return glib::Propagation::Stop;
        }

        glib::Propagation::Proceed
    });

    parent_window.add_controller(key_controller.clone());
    fixed.add_controller(key_controller);
}
