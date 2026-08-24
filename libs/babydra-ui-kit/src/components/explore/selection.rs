//! Rubberband selection shared by the grid and list views, plus the FlowBox
//! native-rubberband opt-out.

use gtk4::prelude::*;
use gtk4::{Box, Fixed, FlowBox, FlowBoxChild, GestureDrag, ListBox, ListBoxRow};
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
        if let Some(controller) = controllers
            .item(i)
            .and_then(|o| o.downcast::<gtk4::GestureDrag>().ok())
        {
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
) {
    wire_rubberband_gesture(
        grid_overlay,
        GridView(grid_container),
        grid_fixed,
        grid_rubberband,
    );
}

/// Wires rubberband select gesture for a ListBox view overlay.
pub fn wire_rubberband(
    list_overlay: &gtk4::Widget,
    listbox: ListBox,
    list_fixed: Fixed,
    list_rubberband: Box,
) {
    wire_rubberband_gesture(list_overlay, ListView(listbox), list_fixed, list_rubberband);
}

/// View-agnostic access to a container's selectable children so one gesture
/// implementation serves both the grid (FlowBox) and list (ListBox) views.
trait RubberbandItems: 'static {
    type Child: Clone + 'static;
    /// True when `picked` (or an ancestor below the overlay) is an item.
    fn hits_child(&self, picked: Option<gtk4::Widget>) -> bool;
    /// Snapshot every selectable child with its position relative to `overlay`.
    fn children(&self, overlay: &gtk4::Widget) -> Vec<(Self::Child, f64, f64, f64, f64)>;
    fn set_selected(&self, child: &Self::Child, selected: bool);
}

struct GridView(Box);

impl RubberbandItems for GridView {
    type Child = (FlowBox, FlowBoxChild);

    fn hits_child(&self, picked: Option<gtk4::Widget>) -> bool {
        let mut curr = picked;
        while let Some(w) = curr {
            if w.downcast_ref::<FlowBoxChild>().is_some() {
                return true;
            }
            curr = w.parent();
        }
        false
    }

    fn children(&self, overlay: &gtk4::Widget) -> Vec<(Self::Child, f64, f64, f64, f64)> {
        let mut rects = Vec::new();
        let mut sibling = self.0.first_child();
        while let Some(child) = sibling {
            if let Some(fb) = child.downcast_ref::<FlowBox>() {
                let mut item_child = fb.first_child();
                while let Some(c) = item_child {
                    if let Some(fb_child) = c.downcast_ref::<FlowBoxChild>() {
                        if let Some((cx, cy)) = c.translate_coordinates(overlay, 0.0, 0.0) {
                            rects.push((
                                (fb.clone(), fb_child.clone()),
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
        rects
    }

    fn set_selected(&self, child: &Self::Child, selected: bool) {
        if selected {
            child.0.select_child(&child.1);
        } else {
            child.0.unselect_child(&child.1);
        }
    }
}

struct ListView(ListBox);

impl RubberbandItems for ListView {
    type Child = ListBoxRow;

    fn hits_child(&self, picked: Option<gtk4::Widget>) -> bool {
        let mut curr = picked;
        while let Some(w) = curr {
            if w.downcast_ref::<ListBoxRow>().is_some() {
                return true;
            }
            curr = w.parent();
        }
        false
    }

    fn children(&self, overlay: &gtk4::Widget) -> Vec<(Self::Child, f64, f64, f64, f64)> {
        let mut rects = Vec::new();
        let mut child = self.0.first_child();
        while let Some(c) = child {
            if let Some(row) = c.downcast_ref::<ListBoxRow>() {
                if let Some((cx, cy)) = c.translate_coordinates(overlay, 0.0, 0.0) {
                    rects.push((row.clone(), cx, cy, c.width() as f64, c.height() as f64));
                }
            }
            child = c.next_sibling();
        }
        rects
    }

    fn set_selected(&self, child: &Self::Child, selected: bool) {
        if selected {
            self.0.select_row(Some(child));
        } else {
            self.0.unselect_row(child);
        }
    }
}

/// One drag gesture driving rectangle selection over any [`RubberbandItems`]
/// view. Item rects are snapshotted at drag-begin; recomputing them per
/// mouse-move via translate_coordinates would be O(n) layout traversal.
fn wire_rubberband_gesture<V: RubberbandItems>(
    overlay: &gtk4::Widget,
    view: V,
    fixed: Fixed,
    rubberband: Box,
) {
    let drag_gesture = GestureDrag::new();
    drag_gesture.set_button(1);
    drag_gesture.set_propagation_phase(gtk4::PropagationPhase::Capture);

    let start_pos: Rc<RefCell<Option<(f64, f64)>>> = Rc::new(RefCell::new(None));
    let active: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
    let view = Rc::new(view);
    let rects: Rc<RefCell<Vec<(V::Child, f64, f64, f64, f64)>>> = Rc::new(RefCell::new(Vec::new()));

    // --- drag begin: deny when starting on an item (that means item DnD) ---
    {
        let active_begin = active.clone();
        let start_pos_begin = start_pos.clone();
        let overlay_begin = overlay.clone();
        let view_begin = view.clone();
        let rects_begin = rects.clone();

        drag_gesture.connect_drag_begin(move |gesture, x, y| {
            let picked = overlay_begin.pick(x, y, gtk4::PickFlags::empty());
            if view_begin.hits_child(picked) {
                active_begin.replace(false);
                gesture.set_state(gtk4::EventSequenceState::Denied);
            } else {
                active_begin.replace(true);
                start_pos_begin.replace(Some((x, y)));
                *rects_begin.borrow_mut() = view_begin.children(&overlay_begin);
                gesture.set_state(gtk4::EventSequenceState::Claimed);
            }
        });
    }

    // --- drag update: move the rubberband and toggle intersecting items ---
    {
        let start_pos_update = start_pos.clone();
        let active_update = active.clone();
        let fixed_update = fixed.clone();
        let rubberband_update = rubberband.clone();
        let overlay_update = overlay.clone();
        let view_update = view.clone();
        let rects_update = rects.clone();

        drag_gesture.connect_drag_update(move |_, offset_x, offset_y| {
            if !*active_update.borrow() {
                return;
            }
            let Some((start_x, start_y)) = *start_pos_update.borrow() else {
                return;
            };
            let current_x = start_x + offset_x;
            let current_y = start_y + offset_y;
            let min_x = start_x.min(current_x);
            let max_x = start_x.max(current_x);
            let min_y = start_y.min(current_y);
            let max_y = start_y.max(current_y);
            let width = (max_x - min_x).max(0.0);
            let height = (max_y - min_y).max(0.0);

            if !rubberband_update.is_visible() {
                rubberband_update.set_visible(true);
            }

            if let Some((fx, fy)) =
                overlay_update.translate_coordinates(&fixed_update, min_x, min_y)
            {
                fixed_update.move_(&rubberband_update, fx, fy);
            }
            rubberband_update.set_size_request(width as i32, height as i32);

            for (child, cx, cy, cw, ch) in rects_update.borrow().iter() {
                let intersects =
                    !(cx > &max_x || cx + cw < min_x || cy > &max_y || cy + ch < min_y);
                view_update.set_selected(child, intersects);
            }
        });
    }

    // --- drag end ---
    {
        let rb_end = rubberband.clone();
        let active_end = active.clone();
        let rects_end = rects.clone();

        drag_gesture.connect_drag_end(move |_, _, _| {
            if *active_end.borrow() {
                rb_end.set_visible(false);
                *active_end.borrow_mut() = false;
                rects_end.borrow_mut().clear();
            }
        });
    }

    overlay.add_controller(drag_gesture);
}
