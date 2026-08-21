use gtk4::prelude::*;
use gtk4::{Box, Fixed, GestureDrag, ListBox, ListBoxRow};
use std::cell::RefCell;
use std::rc::Rc;

/// Wires rubberband select gesture for a ListBox view overlay.
pub fn wire_rubberband(
    list_overlay: &gtk4::Widget,
    listbox: ListBox,
    list_fixed: Fixed,
    list_rubberband: Box,
    selected_paths: Rc<RefCell<Vec<std::path::PathBuf>>>,
) {
    let drag_gesture = GestureDrag::new();
    drag_gesture.set_button(1);
    drag_gesture.set_propagation_phase(gtk4::PropagationPhase::Capture);

    let start_pos = Rc::new(RefCell::new(None::<(f64, f64)>));
    let start_pos_c = start_pos.clone();
    let drag_select_active = Rc::new(RefCell::new(false));

    let drag_select_active_begin = drag_select_active.clone();
    let list_overlay_begin = list_overlay.clone();

    drag_gesture.connect_drag_begin(move |gesture, x, y| {
        let picked = list_overlay_begin.pick(x, y, gtk4::PickFlags::empty());
        let mut is_item = false;
        let mut curr = picked;
        while let Some(w) = curr {
            if let Some(_row) = w.downcast_ref::<ListBoxRow>() {
                is_item = true;
                break;
            }
            if w == list_overlay_begin {
                break;
            }
            curr = w.parent();
        }

        if is_item {
            // User clicked and dragged an item -> Treat as MOVE (Drag and Drop)
            drag_select_active_begin.replace(false);
            gesture.set_state(gtk4::EventSequenceState::Denied);
        } else {
            // User clicked on empty space or unselected item and dragged -> Treat as SELECT (Rubberband)
            drag_select_active_begin.replace(true);
            start_pos_c.replace(Some((x, y)));
            gesture.set_state(gtk4::EventSequenceState::Claimed);
        }
    });

    let start_pos_update = start_pos.clone();
    let drag_select_active_update = drag_select_active.clone();
    let lb_update = listbox.clone();
    let lf_update = list_fixed.clone();
    let lr_update = list_rubberband.clone();
    let list_overlay_update = list_overlay.clone();

    drag_gesture.connect_drag_update(move |_, offset_x, offset_y| {
        if !*drag_select_active_update.borrow() {
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

            if !lr_update.is_visible() {
                lr_update.set_visible(true);
            }

            let fixed_pos = list_overlay_update.translate_coordinates(&lf_update, min_x, min_y);
            if let Some((fx, fy)) = fixed_pos {
                lf_update.move_(&lr_update, fx, fy);
            }
            lr_update.set_size_request(width as i32, height as i32);

            let mut child = lb_update.first_child();
            while let Some(c) = child {
                if let Some((cx, cy)) = c.translate_coordinates(&list_overlay_update, 0.0, 0.0) {
                    let cw = c.width() as f64;
                    let ch = c.height() as f64;

                    let intersects =
                        !(cx > max_x || cx + cw < min_x || cy > max_y || cy + ch < min_y);
                    if let Some(row) = c.downcast_ref::<ListBoxRow>() {
                        if intersects {
                            lb_update.select_row(Some(row));
                        } else {
                            lb_update.unselect_row(row);
                        }
                    }
                }
                child = c.next_sibling();
            }
        }
    });

    let rb_end = list_rubberband.clone();
    let drag_select_active_end = drag_select_active.clone();
    drag_gesture.connect_drag_end(move |_, _, _| {
        if *drag_select_active_end.borrow() {
            rb_end.set_visible(false);
            *drag_select_active_end.borrow_mut() = false;
        }
    });

    list_overlay.add_controller(drag_gesture);
}
