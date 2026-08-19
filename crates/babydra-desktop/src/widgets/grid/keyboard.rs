//! Desktop keyboard shortcuts controller.

use crate::state::DesktopState;
use crate::widgets::icon::launch_entry;
use crate::widgets::selection::update_icon_sel;
use babydra_ui_kit::components::explore::prelude::*;
use gtk4::prelude::*;
use gtk4::{EventControllerKey, Fixed};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub fn wire_keyboard(
    fixed: &Fixed,
    state: Rc<RefCell<DesktopState>>,
    rubberband: gtk4::Box,
    refresh_fn: Rc<dyn Fn()>,
) {
    let key_controller = EventControllerKey::new();
    let state_key = state.clone();
    let fixed_key = fixed.clone();
    let ref_cb_key = refresh_fn.clone();
    let rubberband_key = rubberband.clone();

    key_controller.connect_key_pressed(move |_, keyval, _, mod_state| {
        let has_ctrl = mod_state.contains(gtk4::gdk::ModifierType::CONTROL_MASK);
        let has_shift = mod_state.contains(gtk4::gdk::ModifierType::SHIFT_MASK);

        match keyval {
            gtk4::gdk::Key::Return | gtk4::gdk::Key::KP_Enter => {
                let state_ref = state_key.borrow();
                for entry in &state_ref.entries {
                    if state_ref.is_selected(&entry.path) {
                        launch_entry(entry);
                    }
                }
                glib::Propagation::Stop
            }
            gtk4::gdk::Key::A | gtk4::gdk::Key::a if has_ctrl => {
                state_key.borrow_mut().select_all();
                update_icon_sel(&fixed_key, &state_key, &rubberband_key);
                glib::Propagation::Stop
            }
            gtk4::gdk::Key::F5 | gtk4::gdk::Key::R | gtk4::gdk::Key::r
                if has_ctrl || keyval == gtk4::gdk::Key::F5 =>
            {
                ref_cb_key();
                glib::Propagation::Stop
            }
            gtk4::gdk::Key::C | gtk4::gdk::Key::c if has_ctrl => {
                let state_ref = state_key.borrow();
                let selected: Vec<PathBuf> = state_ref.selected_paths.iter().cloned().collect();
                if !selected.is_empty() {
                    CLIPBOARD.with(|cb| cb.replace(Some((selected.clone(), false))));
                    set_clipboard_files(&selected, false);
                }
                glib::Propagation::Stop
            }
            gtk4::gdk::Key::X | gtk4::gdk::Key::x if has_ctrl => {
                let state_ref = state_key.borrow();
                let selected: Vec<PathBuf> = state_ref.selected_paths.iter().cloned().collect();
                if !selected.is_empty() {
                    CLIPBOARD.with(|cb| cb.replace(Some((selected.clone(), true))));
                    set_clipboard_files(&selected, true);
                    apply_cut_everywhere(&selected);
                }
                glib::Propagation::Stop
            }
            gtk4::gdk::Key::V | gtk4::gdk::Key::v if has_ctrl => {
                let ddir = DesktopState::desktop_dir();
                let clipboard_data = CLIPBOARD.with(|cb| cb.borrow().clone());
                if let Some((sources, is_cut)) = clipboard_data {
                    let nav_cb = crate::widgets::context_menu::refresh_nav_cb(ref_cb_key.clone());
                    execute_paste(sources, ddir.clone(), is_cut, ddir.clone(), nav_cb);
                }
                glib::Propagation::Stop
            }
            gtk4::gdk::Key::Delete | gtk4::gdk::Key::KP_Delete => {
                let state_ref = state_key.borrow();
                let selected: Vec<PathBuf> = state_ref.selected_paths.iter().cloned().collect();
                if !selected.is_empty() {
                    let ref_cb_inner = ref_cb_key.clone();
                    glib::spawn_future_local(async move {
                        for path in selected {
                            if has_shift {
                                let _ = babydra_core::delete_path(path).await;
                            } else {
                                let _ = babydra_core::send_to_trash(path).await;
                            }
                        }
                        ref_cb_inner();
                    });
                }
                glib::Propagation::Stop
            }
            _ => glib::Propagation::Proceed,
        }
    });

    fixed.add_controller(key_controller);
}
