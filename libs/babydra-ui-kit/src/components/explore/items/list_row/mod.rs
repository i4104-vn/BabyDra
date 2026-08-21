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
    if selected_paths.borrow().contains(&entry.path) {
        item_box.add_css_class("selected");
    }

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
                crate::components::explore::dialogs::launch_file_or_open_with(
                    &target_path,
                    None::<&gtk4::Window>,
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
    let path_drag = entry.path.clone();
    let sel_drag = selected_paths.clone();

    let drag_source = crate::components::explore::create_drag_source(
        &entry.path,
        &entry.icon_name,
        is_dragging,
        move || {
            let s = sel_drag.borrow();
            if s.contains(&path_drag) && s.len() > 1 {
                s.clone()
            } else {
                vec![path_drag.clone()]
            }
        },
    );
    drag_source.set_propagation_phase(gtk4::PropagationPhase::Capture);

    list_row.add_controller(drag_source);

    list_row
}
