//! Fixed icon grid container and interaction manager for the desktop surface.

pub mod dnd;

pub use dnd::{create_desktop_drop_target, create_folder_drop_target, create_icon_drag_source};

use crate::state::DesktopState;
use crate::widgets::context_menu::{show_desktop_empty_menu, show_desktop_file_menu};
use crate::widgets::icon::{create_desktop_icon_widget, launch_entry};
use crate::widgets::selection::{
    attach_rubberband_controller, update_icons_selection_state,
};
use babydra_core::models::explore::FileType;
use babydra_ui_kit::components::explore::prelude::*;
use gtk4::prelude::*;
use gtk4::{Box, EventControllerKey, Fixed, GestureClick};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// Builds a refresh callback that reloads desktop entries and rebuilds the icon grid.
fn make_refresh_cb(
    fixed: &Fixed,
    state: &Rc<RefCell<DesktopState>>,
    parent_window: &gtk4::ApplicationWindow,
    rubberband: &Box,
) -> Rc<dyn Fn()> {
    let fixed_c = fixed.clone();
    let state_c = state.clone();
    let parent_win_c = parent_window.clone();
    let rubberband_c = rubberband.clone();

    Rc::new(move || {
        let f = fixed_c.clone();
        let s = state_c.clone();
        let p = parent_win_c.clone();
        let r = rubberband_c.clone();

        glib::spawn_future_local(async move {
            s.borrow_mut().reload_entries().await;
            rebuild_grid_icons(&f, &s, &p, &r);
        });
    })
}

/// Builds the desktop grid fixed layout, attaching gestures, keyboard shortcuts, and file watching.
pub fn create_desktop_grid(
    parent_window: &gtk4::ApplicationWindow,
) -> (Fixed, Rc<RefCell<DesktopState>>, Rc<dyn Fn()>) {
    let fixed = Fixed::new();
    fixed.set_hexpand(true);
    fixed.set_vexpand(true);
    fixed.set_focusable(true);
    fixed.set_can_focus(true);
    fixed.add_css_class("desktop-grid");

    let state = Rc::new(RefCell::new(DesktopState::new()));

    // 1. Rubberband Selection Widget
    let rubberband = Box::new(gtk4::Orientation::Vertical, 0);
    rubberband.add_css_class("desktop-rubberband");
    rubberband.set_visible(false);
    fixed.put(&rubberband, 0.0, 0.0);

    // 2. Refresh callback
    let refresh_fn = make_refresh_cb(&fixed, &state, parent_window, &rubberband);

    // 3. Desktop Background Click Gesture (Deselect / Focus)
    let bg_click = GestureClick::new();
    bg_click.set_button(1); // Left click
    let state_bg = state.clone();
    let fixed_bg = fixed.clone();
    let rubberband_bg = rubberband.clone();

    bg_click.connect_pressed(move |_, _, _, _| {
        fixed_bg.grab_focus();
        state_bg.borrow_mut().clear_selection();
        update_icons_selection_state(&fixed_bg, &state_bg, &rubberband_bg);
    });
    fixed.add_controller(bg_click);

    // 4. Desktop Background Right-Click Gesture (Empty Desktop Context Menu)
    let bg_right_click = GestureClick::new();
    bg_right_click.set_button(3); // Right click
    let parent_win_rc = parent_window.clone();
    let fixed_right = fixed.clone();
    let ref_cb_right = refresh_fn.clone();

    bg_right_click.connect_pressed(move |_, _, x, y| {
        fixed_right.grab_focus();
        show_desktop_empty_menu(
            fixed_right.upcast_ref::<gtk4::Widget>(),
            x,
            y,
            ref_cb_right.clone(),
            &parent_win_rc,
        );
    });
    fixed.add_controller(bg_right_click);

    // 5. Desktop Background DropTarget (Ingestion + Repositioning)
    let drop_target = create_desktop_drop_target(state.clone(), refresh_fn.clone());
    fixed.add_controller(drop_target);

    // 6. Rubberband Lasso Selection Controller
    attach_rubberband_controller(&fixed, state.clone(), rubberband.clone());

    // 7. Desktop Keyboard Shortcuts
    let key_controller = EventControllerKey::new();
    let state_key = state.clone();
    let fixed_key = fixed.clone();
    let ref_cb_key = refresh_fn.clone();
    let rubberband_key = rubberband.clone();

    key_controller.connect_key_pressed(move |_, keyval, _, mod_state| {
        let has_ctrl = mod_state.contains(gtk4::gdk::ModifierType::CONTROL_MASK);
        let has_shift = mod_state.contains(gtk4::gdk::ModifierType::SHIFT_MASK);

        match keyval {
            // Enter / Return: Launch all selected items
            gtk4::gdk::Key::Return | gtk4::gdk::Key::KP_Enter => {
                let st = state_key.borrow();
                for entry in &st.entries {
                    if st.is_selected(&entry.path) {
                        launch_entry(entry);
                    }
                }
                glib::Propagation::Stop
            }
            // Ctrl + A: Select All
            gtk4::gdk::Key::A | gtk4::gdk::Key::a if has_ctrl => {
                state_key.borrow_mut().select_all();
                update_icons_selection_state(&fixed_key, &state_key, &rubberband_key);
                glib::Propagation::Stop
            }
            // F5 / Ctrl + R: Refresh
            gtk4::gdk::Key::F5 | gtk4::gdk::Key::R | gtk4::gdk::Key::r if has_ctrl || keyval == gtk4::gdk::Key::F5 => {
                ref_cb_key();
                glib::Propagation::Stop
            }
            // Ctrl + C: Copy selected
            gtk4::gdk::Key::C | gtk4::gdk::Key::c if has_ctrl => {
                let st = state_key.borrow();
                let selected: Vec<PathBuf> = st.selected_paths.iter().cloned().collect();
                if !selected.is_empty() {
                    CLIPBOARD.with(|cb| cb.replace(Some((selected.clone(), false))));
                    set_system_clipboard_files(&selected, false);
                }
                glib::Propagation::Stop
            }
            // Ctrl + X: Cut selected
            gtk4::gdk::Key::X | gtk4::gdk::Key::x if has_ctrl => {
                let st = state_key.borrow();
                let selected: Vec<PathBuf> = st.selected_paths.iter().cloned().collect();
                if !selected.is_empty() {
                    CLIPBOARD.with(|cb| cb.replace(Some((selected.clone(), true))));
                    set_system_clipboard_files(&selected, true);
                    apply_cut_dimming_global(&selected);
                }
                glib::Propagation::Stop
            }
            // Ctrl + V: Paste from clipboard
            gtk4::gdk::Key::V | gtk4::gdk::Key::v if has_ctrl => {
                let ddir = DesktopState::desktop_dir();
                let clipboard_data = CLIPBOARD.with(|cb| cb.borrow().clone());
                if let Some((sources, is_cut)) = clipboard_data {
                    let nav_cb = crate::widgets::context_menu::refresh_nav_cb(ref_cb_key.clone());
                    execute_paste(
                        sources,
                        ddir.clone(),
                        is_cut,
                        ddir.clone(),
                        nav_cb,
                    );
                }
                glib::Propagation::Stop
            }
            // Delete: Move selected to trash
            gtk4::gdk::Key::Delete | gtk4::gdk::Key::KP_Delete => {
                let st = state_key.borrow();
                let selected: Vec<PathBuf> = st.selected_paths.iter().cloned().collect();
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

    // Initial load
    refresh_fn();

    // Start FileWatcher daemon on ~/Desktop
    let desktop_path = DesktopState::desktop_dir();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();
    let ref_cb_watch = refresh_fn.clone();

    glib::spawn_future_local(async move {
        while rx.recv().await.is_some() {
            ref_cb_watch();
        }
    });

    if let Ok(_watcher) = babydra_core::FileWatcher::new(desktop_path, move |_event| {
        let _ = tx.send(());
    }) {
        std::mem::forget(_watcher);
    }

    (fixed, state, refresh_fn)
}

/// Rebuilds all icon widgets inside the Fixed layout based on the current DesktopState.
pub fn rebuild_grid_icons(
    fixed: &Fixed,
    state: &Rc<RefCell<DesktopState>>,
    parent_window: &gtk4::ApplicationWindow,
    rubberband: &Box,
) {
    // 1. Remove all old icon widgets (preserve rubberband)
    let mut to_remove = Vec::new();
    let mut child_opt = fixed.first_child();
    while let Some(child) = child_opt {
        if child != rubberband.clone().upcast::<gtk4::Widget>() {
            to_remove.push(child.clone());
        }
        child_opt = child.next_sibling();
    }
    for w in to_remove {
        fixed.remove(&w);
    }

    let st = state.borrow();
    let icon_size = st.config.icon_size;
    let entries = st.entries.clone();
    let selected_paths = st.selected_paths.clone();
    drop(st);

    for (index, entry) in entries.iter().enumerate() {
        let is_sel = selected_paths.contains(&entry.path);
        let icon_widget = create_desktop_icon_widget(entry, icon_size, is_sel);
        icon_widget.set_widget_name(&entry.path.to_string_lossy());

        let file_name = entry.name.to_string_lossy().to_string();
        let (pos_x, pos_y) = state.borrow().get_entry_position(&file_name, index);

        // 1. Single click / double click gesture
        let click_gesture = GestureClick::new();
        click_gesture.set_button(1); // Left click

        let entry_click = entry.clone();
        let state_click = state.clone();
        let fixed_click = fixed.clone();
        let rubberband_click = rubberband.clone();

        click_gesture.connect_pressed(move |gesture, n_press, _, _| {
            if n_press == 1 {
                let event = gesture.current_event();
                let is_ctrl = event
                    .as_ref()
                    .map(|e| e.modifier_state().contains(gtk4::gdk::ModifierType::CONTROL_MASK))
                    .unwrap_or(false);

                state_click
                    .borrow_mut()
                    .select(entry_click.path.clone(), is_ctrl, is_ctrl);
                update_icons_selection_state(&fixed_click, &state_click, &rubberband_click);
            } else if n_press == 2 {
                launch_entry(&entry_click);
            }
        });
        icon_widget.add_controller(click_gesture);

        // 2. Right click gesture (File context menu)
        let right_click = GestureClick::new();
        right_click.set_button(3); // Right click
        let entry_rc = entry.clone();
        let parent_win_rc = parent_window.clone();
        let fixed_rc = fixed.clone();
        let state_rc = state.clone();
        let rubberband_rc = rubberband.clone();

        right_click.connect_pressed(move |_, _, x, y| {
            state_rc.borrow_mut().select(entry_rc.path.clone(), false, false);
            update_icons_selection_state(&fixed_rc, &state_rc, &rubberband_rc);

            let fixed_ref = fixed_rc.clone();
            let state_ref = state_rc.clone();
            let parent_win_ref = parent_win_rc.clone();
            let rubberband_ref = rubberband_rc.clone();

            let refresh_cb = make_refresh_cb(&fixed_ref, &state_ref, &parent_win_ref, &rubberband_ref);

            show_desktop_file_menu(
                fixed_rc.upcast_ref::<gtk4::Widget>(),
                pos_x as f64 + x,
                pos_y as f64 + y,
                &entry_rc,
                refresh_cb,
                &parent_win_rc,
            );
        });
        icon_widget.add_controller(right_click);

        // 3. Drag Source (DND for repositioning, moving to folders, or dragging to external apps)
        let sel_paths_rc = Rc::new(RefCell::new(
            selected_paths.iter().cloned().collect::<Vec<PathBuf>>(),
        ));
        let drag_src = create_icon_drag_source(&entry.path, &entry.icon_name, sel_paths_rc);
        icon_widget.add_controller(drag_src);

        // 4. Folder Drop Target (If entry is a directory, accept dropping files into it)
        if entry.file_type == FileType::Directory {
            let fixed_f = fixed.clone();
            let state_f = state.clone();
            let parent_f = parent_window.clone();
            let rubber_f = rubberband.clone();

            let ref_folder_cb = make_refresh_cb(&fixed_f, &state_f, &parent_f, &rubber_f);

            let folder_drop = create_folder_drop_target(entry.path.clone(), icon_widget.clone(), ref_folder_cb);
            icon_widget.add_controller(folder_drop);
        }

        fixed.put(&icon_widget, pos_x as f64, pos_y as f64);
    }
}
