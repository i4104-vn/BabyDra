use babydra_core::FileEntry;
use gtk4::prelude::*;
use gtk4::ListBoxRow;
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

mod render;

pub fn create_list_row(
    idx: usize,
    entry: &FileEntry,
    selected_paths: Rc<RefCell<Vec<PathBuf>>>,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    on_right_click: impl Fn(&gtk4::Widget, f64, f64) + 'static,
) -> ListBoxRow {
    let item_box = render::build_list_row_ui(entry);

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

    let is_cut = crate::components::explore::CLIPBOARD.with(|cb| {
        cb.borrow()
            .as_ref()
            .map(|(paths, cut)| *cut && paths.contains(&entry.path))
            .unwrap_or(false)
    });

    if is_cut {
        item_box.add_css_class("cut-item");
    }

    let double_click_gesture = gtk4::GestureClick::new();
    double_click_gesture.set_button(1);

    let target_path = entry.path.clone();
    let is_dir = matches!(entry.file_type, babydra_core::FileType::Directory);

    double_click_gesture.connect_pressed(move |_, n_press, _, _| {
        if n_press == 2 {
            if is_dir {
                nav_callback(target_path.clone());
            } else {
                let uri = format!("file://{}", target_path.to_string_lossy());
                let _ = gtk4::gio::AppInfo::launch_default_for_uri(
                    &uri,
                    gtk4::gio::AppLaunchContext::NONE,
                );
            }
        }
    });

    let list_row = ListBoxRow::new();
    list_row.set_child(Some(&item_box));
    list_row.set_property("name", &format!("{}", idx));
    list_row.set_widget_name(&entry.path.to_string_lossy());
    list_row.add_controller(double_click_gesture);

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
    item_box.add_controller(saver);

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
    item_box.add_controller(drag_source);

    list_row
}
