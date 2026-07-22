use gtk4::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use babydra_common::ContentViewWidgets;

/// Wires background click gestures, context menus, drag select, and drag drop to the view container
pub fn wire_background_controllers(
    widgets: &ContentViewWidgets,
    current_path: Rc<RefCell<PathBuf>>,
    nav_cb: Rc<dyn Fn(PathBuf)>,
) {
    // 1. Drag-to-select for Grid overlay
    if let Some(grid_overlay) = widgets.grid_fixed.parent() {
        babydra_utils::explore::wire_rubberband_grid(
            &grid_overlay,
            widgets.grid_container.clone(),
            widgets.grid_fixed.clone(),
            widgets.grid_rubberband.clone(),
        );
    }

    // 2. Right click context menu on empty space
    {
        let cp = current_path.clone();
        let nav = nav_cb.clone();
        let container_widget = widgets.container.clone();
        let gesture = gtk4::GestureClick::new();
        gesture.set_button(3);
        gesture.connect_pressed(move |gesture, _, x, y| {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            let path = cp.borrow().clone();
            if let Some(win) = container_widget.root().and_then(|r| r.downcast::<gtk4::Window>().ok()) {
                babydra_utils::explore::context_menu::show_for_empty(
                    container_widget.upcast_ref(),
                    x,
                    y,
                    path,
                    nav.clone(),
                    &win,
                );
            }
        });
        widgets.container.add_controller(gesture);
    }

    // 3. Drop target to background
    {
        let drop_target = babydra_utils::explore::create_background_drop_target(current_path.clone());
        widgets.container.add_controller(drop_target);
    }
}
