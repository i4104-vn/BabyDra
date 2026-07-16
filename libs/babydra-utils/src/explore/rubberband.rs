use gtk4::prelude::*;
use gtk4::{ListBox, Box, Fixed, ListBoxRow, FlowBox, FlowBoxChild, GestureDrag, PickFlags};
use std::rc::Rc;
use std::cell::RefCell;

/// Wires rubberband select gesture for a ListBox view overlay.
pub fn wire_rubberband_listbox(
    list_overlay: &gtk4::Widget,
    listbox: ListBox,
    list_fixed: Fixed,
    list_rubberband: Box,
) {
    let drag_gesture = GestureDrag::new();
    drag_gesture.set_button(1);
    
    let start_pos = Rc::new(RefCell::new(None::<(f64, f64)>));
    let start_pos_c = start_pos.clone();
    let drag_select_active = Rc::new(RefCell::new(false));
    
    let rb_begin = list_rubberband.clone();
    let drag_select_active_begin = drag_select_active.clone();
    let lf_parent = list_fixed.parent().map(|p| p.clone());
    drag_gesture.connect_drag_begin(move |_, x, y| {
        let mut is_item = false;
        if let Some(ref parent) = lf_parent {
            let picked = parent.pick(x, y, PickFlags::empty());
            let mut next = picked;
            while let Some(w) = next {
                if w.downcast_ref::<FlowBoxChild>().is_some() || w.downcast_ref::<ListBoxRow>().is_some() {
                    is_item = true;
                    break;
                }
                next = w.parent();
            }
        }

        if !is_item {
            drag_select_active_begin.replace(true);
            start_pos_c.replace(Some((x, y)));
            rb_begin.set_visible(true);
            rb_begin.set_size_request(0, 0);
        } else {
            drag_select_active_begin.replace(false);
        }
    });

    let start_pos_update = start_pos.clone();
    let drag_select_active_update = drag_select_active.clone();
    let lb_update = listbox.clone();
    let lf_update = list_fixed.clone();
    let lr_update = list_rubberband.clone();
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
            let width = max_x - min_x;
            let height = max_y - min_y;

            lf_update.move_(&lr_update, min_x, min_y);
            lr_update.set_size_request(width as i32, height as i32);

            let mut child = lb_update.first_child();
            while let Some(c) = child {
                if let Some((cx, cy)) = c.translate_coordinates(&lb_update, 0.0, 0.0) {
                    let cw = c.width() as f64;
                    let ch = c.height() as f64;
                    
                    let intersects = !(cx > max_x || cx + cw < min_x || cy > max_y || cy + ch < min_y);
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
        }
    });

    list_overlay.add_controller(drag_gesture);
}

/// Wires rubberband select gesture for a Grid view container.
pub fn wire_rubberband_grid(
    grid_overlay: &gtk4::Widget,
    grid_container: Box,
    grid_fixed: Fixed,
    grid_rubberband: Box,
) {
    let drag_gesture = GestureDrag::new();
    drag_gesture.set_button(1);
    
    let start_pos = Rc::new(RefCell::new(None::<(f64, f64)>));
    let start_pos_c = start_pos.clone();
    let drag_select_active = Rc::new(RefCell::new(false));
    
    let rb_begin = grid_rubberband.clone();
    let drag_select_active_begin = drag_select_active.clone();
    let gf_parent = grid_fixed.parent().map(|p| p.clone());
    drag_gesture.connect_drag_begin(move |_, x, y| {
        let mut is_item = false;
        if let Some(ref parent) = gf_parent {
            let picked = parent.pick(x, y, PickFlags::empty());
            let mut next = picked;
            while let Some(w) = next {
                if w.downcast_ref::<FlowBoxChild>().is_some() || w.downcast_ref::<ListBoxRow>().is_some() {
                    is_item = true;
                    break;
                }
                next = w.parent();
            }
        }

        if !is_item {
            drag_select_active_begin.replace(true);
            start_pos_c.replace(Some((x, y)));
            rb_begin.set_visible(true);
            rb_begin.set_size_request(0, 0);
        } else {
            drag_select_active_begin.replace(false);
        }
    });

    let start_pos_update = start_pos.clone();
    let drag_select_active_update = drag_select_active.clone();
    let gc_update = grid_container.clone();
    let gf_update = grid_fixed.clone();
    let gr_update = grid_rubberband.clone();
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
            let width = max_x - min_x;
            let height = max_y - min_y;

            gf_update.move_(&gr_update, min_x, min_y);
            gr_update.set_size_request(width as i32, height as i32);

            let mut sibling = gc_update.first_child();
            while let Some(child) = sibling {
                if let Some(fb) = child.downcast_ref::<FlowBox>() {
                    let mut item_child = fb.first_child();
                    while let Some(c) = item_child {
                        if let Some((cx, cy)) = c.translate_coordinates(&gc_update, 0.0, 0.0) {
                            let cw = c.width() as f64;
                            let ch = c.height() as f64;
                            
                            let intersects = !(cx > max_x || cx + cw < min_x || cy > max_y || cy + ch < min_y);
                            if let Some(fb_child) = c.downcast_ref::<FlowBoxChild>() {
                                if intersects {
                                    fb.select_child(fb_child);
                                } else {
                                    fb.unselect_child(fb_child);
                                }
                            }
                        }
                        item_child = c.next_sibling();
                    }
                }
                sibling = child.next_sibling();
            }
        }
    });

    let rb_end = grid_rubberband.clone();
    let drag_select_active_end = drag_select_active.clone();
    drag_gesture.connect_drag_end(move |_, _, _| {
        if *drag_select_active_end.borrow() {
            rb_end.set_visible(false);
        }
    });

    grid_overlay.add_controller(drag_gesture);
}
