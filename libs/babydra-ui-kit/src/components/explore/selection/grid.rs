use gtk4::prelude::*;
use gtk4::{Box, Fixed, FlowBox, FlowBoxChild, GestureDrag};
use std::cell::RefCell;
use std::rc::Rc;

/// Disables the GtkFlowBox built-in rubberband selection gesture.
///
/// GTK's internal drag gesture (CAPTURE phase on the FlowBox) claims the
/// pointer sequence after ~32px even when the press started on a child item,
/// which cancels the child's DragSource and turns item drags into multi-select.
/// Disabling it lets items be drag-and-dropped normally; empty-space rubberband
/// selection remains handled by the custom overlay gesture (`wire_rubberband_grid`).
pub fn disable_native_rubberband(flowbox: &FlowBox) {
    let controllers = flowbox.observe_controllers();
    for i in 0..controllers.n_items() {
        if let Some(controller) = controllers.item(i).and_then(|o| o.downcast::<gtk4::GestureDrag>().ok()) {
            controller.set_propagation_phase(gtk4::PropagationPhase::None);
        }
    }
}

/// Wires rubberband select gesture for a Grid view container.
pub fn wire_rubberband_grid(
    grid_overlay: &gtk4::Widget,
    grid_container: Box,
    grid_fixed: Fixed,
    grid_rubberband: Box,
    selected_paths: Rc<RefCell<Vec<std::path::PathBuf>>>,
) {
    type ItemRect = (FlowBox, FlowBoxChild, f64, f64, f64, f64);

    let drag_gesture = GestureDrag::new();
    drag_gesture.set_button(1);
    drag_gesture.set_propagation_phase(gtk4::PropagationPhase::Capture);

    let start_pos = Rc::new(RefCell::new(None::<(f64, f64)>));
    let start_pos_c = start_pos.clone();
    let drag_select_active = Rc::new(RefCell::new(false));
    // Item rects are snapshotted once at drag-begin; computing them per
    // mouse-move event via translate_coordinates is O(n) layout traversal
    let item_rects: Rc<RefCell<Vec<ItemRect>>> = Rc::new(RefCell::new(Vec::new()));

    let drag_select_active_begin = drag_select_active.clone();
    let grid_overlay_begin = grid_overlay.clone();
    let gc_begin = grid_container.clone();
    let rects_begin = item_rects.clone();

    drag_gesture.connect_drag_begin(move |gesture, x, y| {
        let picked = grid_overlay_begin.pick(x, y, gtk4::PickFlags::empty());
        let mut is_item = false;
        let mut curr = picked;
        while let Some(w) = curr {
            if let Some(_fb_child) = w.downcast_ref::<FlowBoxChild>() {
                is_item = true;
                break;
            }
            if w == grid_overlay_begin {
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

            // Snapshot item positions once for the whole rubberband gesture
            let mut rects = Vec::new();
            let mut sibling = gc_begin.first_child();
            while let Some(child) = sibling {
                if let Some(fb) = child.downcast_ref::<FlowBox>() {
                    let mut item_child = fb.first_child();
                    while let Some(c) = item_child {
                        if let Some(fb_child) = c.downcast_ref::<FlowBoxChild>() {
                            if let Some((cx, cy)) =
                                c.translate_coordinates(&grid_overlay_begin, 0.0, 0.0)
                            {
                                rects.push((
                                    fb.clone(),
                                    fb_child.clone(),
                                    cx,
                                    cy,
                                    c.width() as f64,
                                    c.height() as f64,
                                ));
                            }
                        }
                        item_child = c.next_sibling();
                    }
                }
                sibling = child.next_sibling();
            }
            *rects_begin.borrow_mut() = rects;

            gesture.set_state(gtk4::EventSequenceState::Claimed);
        }
    });

    let start_pos_update = start_pos.clone();
    let drag_select_active_update = drag_select_active.clone();
    let gf_update = grid_fixed.clone();
    let gr_update = grid_rubberband.clone();
    let grid_overlay_update = grid_overlay.clone();
    let rects_update = item_rects.clone();

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

            if !gr_update.is_visible() {
                gr_update.set_visible(true);
            }

            // Translate min_x, min_y from grid_overlay to grid_fixed for positioning the rubberband widget
            let fixed_pos = grid_overlay_update.translate_coordinates(&gf_update, min_x, min_y);
            if let Some((fx, fy)) = fixed_pos {
                gf_update.move_(&gr_update, fx, fy);
            }
            gr_update.set_size_request(width as i32, height as i32);

            for (fb, fb_child, cx, cy, cw, ch) in rects_update.borrow().iter() {
                let intersects =
                    !(cx > &max_x || cx + cw < min_x || cy > &max_y || cy + ch < min_y);
                if intersects {
                    fb.select_child(fb_child);
                } else {
                    fb.unselect_child(fb_child);
                }
            }
        }
    });

    let rb_end = grid_rubberband.clone();
    let drag_select_active_end = drag_select_active.clone();
    let rects_end = item_rects.clone();
    drag_gesture.connect_drag_end(move |_, _, _| {
        if *drag_select_active_end.borrow() {
            rb_end.set_visible(false);
            *drag_select_active_end.borrow_mut() = false;
            rects_end.borrow_mut().clear();
        }
    });

    grid_overlay.add_controller(drag_gesture);
}
