use babydra_core::FileEntry;
use gtk4::prelude::*;
use gtk4::FlowBoxChild;
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

mod render;

pub fn create_grid_file(
    idx: usize,
    entry: &FileEntry,
    selected_paths: Rc<RefCell<Vec<PathBuf>>>,
    on_right_click: impl Fn(&gtk4::Widget, f64, f64) + 'static,
) -> FlowBoxChild {
    let item_box = render::build_grid_card_ui(entry);

    let rc_gesture = gtk4::GestureClick::new();
    rc_gesture.set_button(3);
    let item_box_clone = item_box.clone();
    rc_gesture.connect_pressed(move |gesture, _, x, y| {
        gesture.set_state(gtk4::EventSequenceState::Claimed);
        on_right_click(item_box_clone.upcast_ref(), x, y);
    });
    item_box.add_controller(rc_gesture);

    if matches!(entry.file_type, babydra_core::FileType::Directory) {
        let drop_target = crate::components::explore::create_drop_target(entry.path.clone());
        item_box.add_controller(drop_target);
    }

    let is_dragging = Rc::new(Cell::new(false));
    let snapshot: Rc<RefCell<Vec<PathBuf>>> = Rc::new(RefCell::new(Vec::new()));

    let saver = gtk4::GestureClick::new();
    saver.set_button(1);
    saver.set_propagation_phase(gtk4::PropagationPhase::Capture);

    let sel_save = selected_paths.clone();
    let path_save = entry.path.clone();
    let snap_save = snapshot.clone();

    saver.connect_pressed(move |_, n_press, _, _| {
        if n_press == 1 {
            let s = sel_save.borrow();
            if s.contains(&path_save) && s.len() > 1 {
                *snap_save.borrow_mut() = s.clone();
            } else {
                snap_save.borrow_mut().clear();
            }
        }
    });

    let snap_rel = snapshot.clone();
    let is_drag_rel = is_dragging.clone();
    saver.connect_released(move |_, _, _, _| {
        if !is_drag_rel.get() {
            snap_rel.borrow_mut().clear();
        }
    });

    let path_drag = entry.path.clone();
    let sel_drag = selected_paths.clone();
    let snap_drag = snapshot.clone();

    let drag_source = crate::components::explore::create_drag_source(
        &entry.path,
        &entry.icon_name,
        is_dragging,
        move || {
            let snap = snap_drag.borrow();
            if !snap.is_empty() {
                snap.clone()
            } else {
                let s = sel_drag.borrow();
                if s.contains(&path_drag) {
                    s.clone()
                } else {
                    vec![path_drag.clone()]
                }
            }
        },
    );
    drag_source.set_propagation_phase(gtk4::PropagationPhase::Capture);

    let flow_child = FlowBoxChild::new();
    flow_child.set_size_request(114, 114);
    flow_child.set_hexpand(false);
    flow_child.set_vexpand(false);
    flow_child.set_halign(gtk4::Align::Center);
    flow_child.set_valign(gtk4::Align::Center);
    flow_child.set_child(Some(&item_box));
    flow_child.set_property("name", &format!("{}", idx));
    flow_child.set_widget_name(&entry.path.to_string_lossy());

    flow_child.add_controller(saver);
    flow_child.add_controller(drag_source);

    flow_child
}
