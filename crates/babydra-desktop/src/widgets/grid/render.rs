//! Desktop icon grid rendering: builds icon widgets and attaches per-icon gestures.

use super::dnd::{create_folder_drop, create_icon_drag};
use super::make_refresh_cb;
use crate::state::DesktopState;
use crate::widgets::context_menu::show_file_menu;
use crate::widgets::icon::{create_desktop_icon, launch_entry};
use crate::widgets::selection::update_icon_sel;
use babydra_core::models::explore::FileType;
use gtk4::prelude::*;
use gtk4::{Box, Fixed, GestureClick};
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

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

    let state_ref = state.borrow();
    let icon_size = state_ref.config.icon_size;
    let entries = state_ref.entries.clone();
    let selected_paths = state_ref.selected_paths.clone();
    let positions = state_ref.compute_all_positions();
    drop(state_ref);

    for entry in &entries {
        let is_sel = selected_paths.contains(&entry.path);
        let icon_widget = create_desktop_icon(entry, icon_size, is_sel);
        icon_widget.set_widget_name(&entry.path.to_string_lossy());

        let file_name = entry.name.to_string_lossy().to_string();
        let (pos_x, pos_y) = positions.get(&file_name).copied().unwrap_or((16, 48));

        let is_dragging = Rc::new(Cell::new(false));

        // 1. Single click / double click gesture
        let click_gesture = GestureClick::new();
        click_gesture.set_button(1); // Left click
        click_gesture.set_propagation_phase(gtk4::PropagationPhase::Bubble);

        let entry_click = entry.clone();
        let state_click = state.clone();
        let fixed_click = fixed.clone();
        let rubberband_click = rubberband.clone();

        click_gesture.connect_pressed(move |gesture, n_press, _, _| {
            let event = gesture.current_event();
            let is_ctrl = event
                .as_ref()
                .map(|e| {
                    e.modifier_state()
                        .contains(gtk4::gdk::ModifierType::CONTROL_MASK)
                })
                .unwrap_or(false);

            if n_press == 1 {
                let mut s = state_click.borrow_mut();
                // If it's already selected and we aren't using Ctrl, do NOT clear selection yet.
                // We'll clear it on Release if it was just a click (no drag).
                if !s.is_selected(&entry_click.path) || is_ctrl {
                    s.select(entry_click.path.clone(), is_ctrl, is_ctrl);
                    drop(s);
                    update_icon_sel(&fixed_click, &state_click, &rubberband_click);
                }
            } else if n_press == 2 {
                launch_entry(&entry_click);
            }
        });
        icon_widget.add_controller(click_gesture);

        let release_gesture = GestureClick::new();
        release_gesture.set_button(1);
        release_gesture.set_propagation_phase(gtk4::PropagationPhase::Bubble);
        let entry_rel = entry.clone();
        let state_rel = state.clone();
        let fixed_rel = fixed.clone();
        let rubberband_rel = rubberband.clone();
        let is_drag_rel = is_dragging.clone();

        release_gesture.connect_released(move |gesture, n_press, _, _| {
            let event = gesture.current_event();
            let is_ctrl = event
                .as_ref()
                .map(|e| {
                    e.modifier_state()
                        .contains(gtk4::gdk::ModifierType::CONTROL_MASK)
                })
                .unwrap_or(false);

            if n_press == 1 && !is_ctrl && !is_drag_rel.get() {
                let mut s = state_rel.borrow_mut();
                if s.is_selected(&entry_rel.path) && s.selected_paths.len() > 1 {
                    s.select(entry_rel.path.clone(), false, false);
                    drop(s);
                    update_icon_sel(&fixed_rel, &state_rel, &rubberband_rel);
                }
            }
        });
        icon_widget.add_controller(release_gesture);

        // 2. Right click gesture (File context menu)
        let right_click = GestureClick::new();
        right_click.set_button(3); // Right click
        let entry_rc = entry.clone();
        let parent_win_rc = parent_window.clone();
        let fixed_rc = fixed.clone();
        let state_rc = state.clone();
        let rubberband_rc = rubberband.clone();

        right_click.connect_pressed(move |_, _, x, y| {
            state_rc
                .borrow_mut()
                .select(entry_rc.path.clone(), false, false);
            update_icon_sel(&fixed_rc, &state_rc, &rubberband_rc);

            let fixed_ref = fixed_rc.clone();
            let state_ref = state_rc.clone();
            let parent_win_ref = parent_win_rc.clone();
            let rubberband_ref = rubberband_rc.clone();

            let refresh_cb =
                make_refresh_cb(&fixed_ref, &state_ref, &parent_win_ref, &rubberband_ref);

            show_file_menu(
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
        let drag_src = create_icon_drag(
            &entry.path,
            &entry.icon_name,
            state.clone(),
            is_dragging.clone(),
        );
        drag_src.set_propagation_phase(gtk4::PropagationPhase::Capture);
        icon_widget.add_controller(drag_src);

        // 4. Folder Drop Target (If entry is a directory, accept dropping files into it)
        if entry.file_type == FileType::Directory {
            let fixed_f = fixed.clone();
            let state_f = state.clone();
            let parent_f = parent_window.clone();
            let rubber_f = rubberband.clone();

            let ref_folder_cb = make_refresh_cb(&fixed_f, &state_f, &parent_f, &rubber_f);

            let folder_drop =
                create_folder_drop(entry.path.clone(), icon_widget.clone(), ref_folder_cb);
            icon_widget.add_controller(folder_drop);
        }

        fixed.put(&icon_widget, pos_x as f64, pos_y as f64);
    }
}
