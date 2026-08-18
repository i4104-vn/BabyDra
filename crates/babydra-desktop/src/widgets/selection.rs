//! Rubberband (lasso) selection gesture and icon selection state manager for desktop grid.

use crate::state::DesktopState;
use gtk4::prelude::*;
use gtk4::{Box, Fixed, GestureDrag, PickFlags};
use std::cell::RefCell;
use std::rc::Rc;

/// Updates CSS classes (`selected`) on all icon widgets inside `Fixed` based on current `DesktopState`.
pub fn update_icons_selection_state(
    fixed: &Fixed,
    state: &Rc<RefCell<DesktopState>>,
    rubberband: &Box,
) {
    let st = state.borrow();
    let mut child_opt = fixed.first_child();
    while let Some(child) = child_opt {
        if child != rubberband.clone().upcast::<gtk4::Widget>() && child.has_css_class("desktop-icon") {
            if let Some(name) = child.widget_name().as_str().into() {
                let p = std::path::PathBuf::from(name);
                if st.is_selected(&p) {
                    child.add_css_class("selected");
                } else {
                    child.remove_css_class("selected");
                }
            }
        }
        child_opt = child.next_sibling();
    }
}

/// Attaches the rubberband lasso selection controller to the desktop fixed container.
pub fn attach_rubberband_controller(
    desktop_fixed: &Fixed,
    state: Rc<RefCell<DesktopState>>,
    rubberband: Box,
) {
    let drag_gesture = GestureDrag::new();
    drag_gesture.set_button(1);

    let start_pos = Rc::new(RefCell::new(None::<(f64, f64)>));
    let start_pos_begin = start_pos.clone();
    let drag_active = Rc::new(RefCell::new(false));
    let drag_active_begin = drag_active.clone();

    let fixed_begin = desktop_fixed.clone();
    let rb_begin = rubberband.clone();
    let state_begin = state.clone();

    drag_gesture.connect_drag_begin(move |gesture, x, y| {
        let picked = fixed_begin.pick(x, y, PickFlags::empty());
        let mut is_icon = false;
        let mut curr = picked;
        while let Some(w) = curr {
            if w.has_css_class("desktop-icon") {
                is_icon = true;
                break;
            }
            if w == fixed_begin.clone().upcast::<gtk4::Widget>() {
                break;
            }
            curr = w.parent();
        }

        if !is_icon {
            let event = gesture.current_event();
            let is_ctrl = event
                .as_ref()
                .map(|e| e.modifier_state().contains(gtk4::gdk::ModifierType::CONTROL_MASK))
                .unwrap_or(false);

            if !is_ctrl {
                state_begin.borrow_mut().clear_selection();
                update_icons_selection_state(&fixed_begin, &state_begin, &rb_begin);
            }

            drag_active_begin.replace(true);
            start_pos_begin.replace(Some((x, y)));
            fixed_begin.move_(&rb_begin, x, y);
            rb_begin.set_size_request(0, 0);
            rb_begin.set_visible(true);
        } else {
            drag_active_begin.replace(false);
        }
    });

    let start_pos_update = start_pos.clone();
    let drag_active_update = drag_active.clone();
    let fixed_update = desktop_fixed.clone();
    let rb_update = rubberband.clone();
    let state_update = state.clone();

    drag_gesture.connect_drag_update(move |gesture, offset_x, offset_y| {
        if !*drag_active_update.borrow() {
            return;
        }

        if let Some((start_x, start_y)) = *start_pos_update.borrow() {
            let current_x = start_x + offset_x;
            let current_y = start_y + offset_y;
            let min_x = start_x.min(current_x);
            let max_x = start_x.max(current_x);
            let min_y = start_y.min(current_y);
            let max_y = start_y.max(current_y);
            let width = (max_x - min_x).max(0.0);
            let height = (max_y - min_y).max(0.0);

            fixed_update.move_(&rb_update, min_x, min_y);
            rb_update.set_size_request(width as i32, height as i32);

            let event = gesture.current_event();
            let is_ctrl = event
                .as_ref()
                .map(|e| e.modifier_state().contains(gtk4::gdk::ModifierType::CONTROL_MASK))
                .unwrap_or(false);

            let mut st = state_update.borrow_mut();
            if !is_ctrl {
                st.clear_selection();
            }

            let mut child_opt = fixed_update.first_child();
            while let Some(child) = child_opt {
                if child != rb_update.clone().upcast::<gtk4::Widget>()
                    && child.has_css_class("desktop-icon")
                {
                    if let Some((cx, cy)) = child.translate_coordinates(&fixed_update, 0.0, 0.0) {
                        let cw = child.width() as f64;
                        let ch = child.height() as f64;

                        let intersects = !(cx > max_x || cx + cw < min_x || cy > max_y || cy + ch < min_y);
                        if intersects {
                            if let Some(file_path_str) = child.widget_name().as_str().into() {
                                if !file_path_str.is_empty() {
                                    st.select(std::path::PathBuf::from(file_path_str), true, false);
                                }
                            }
                        }
                    }
                }
                child_opt = child.next_sibling();
            }
            drop(st);
            update_icons_selection_state(&fixed_update, &state_update, &rb_update);
        }
    });

    let drag_active_end = drag_active.clone();
    let rb_end = rubberband.clone();
    drag_gesture.connect_drag_end(move |_, _, _| {
        if *drag_active_end.borrow() {
            rb_end.set_visible(false);
            *drag_active_end.borrow_mut() = false;
        }
    });

    desktop_fixed.add_controller(drag_gesture);
}
