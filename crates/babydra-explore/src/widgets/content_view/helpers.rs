use gtk4::prelude::*;
use gtk4::FlowBoxChild;
use std::path::PathBuf;
use std::rc::Rc;
use babydra_common::FileEntry;
use std::cell::RefCell;

pub fn create_flow_child(
    idx: usize,
    entry: &FileEntry,
    current_path: &PathBuf,
    nav_callback: &Rc<dyn Fn(PathBuf)>,
    selected_paths: Rc<RefCell<Vec<PathBuf>>>,
) -> FlowBoxChild {
    let target_entry = entry.clone();
    let cp = current_path.clone();
    let nav = nav_callback.clone();

    let sel_paths = selected_paths.clone();
    let flow_child = baby_utils::components::create_grid_file_item(
        idx,
        entry,
        selected_paths,
        move |widget, x, y| {
            let mut target_paths = sel_paths.borrow().clone();
            if !target_paths.contains(&target_entry.path) {
                target_paths = vec![target_entry.path.clone()];
            }

            crate::widgets::context_menu::show_for_file(
                widget,
                x,
                y,
                target_paths,
                cp.clone(),
                nav.clone(),
            );
        },
    );

    // Dim item if it's currently in the cut clipboard
    let is_cut = baby_utils::explore::CLIPBOARD.with(|cb| {
        cb.borrow().as_ref().map(|(paths, cut)| *cut && paths.contains(&entry.path)).unwrap_or(false)
    });
    if is_cut {
        if let Some(child_widget) = flow_child.child() {
            child_widget.add_css_class("cut-item");
        }
    }

    let double_click_gesture = gtk4::GestureClick::new();
    double_click_gesture.set_button(1);
    let target_path = entry.path.clone();
    let is_dir = matches!(entry.file_type, babydra_common::FileType::Directory);
    let nav_c = nav_callback.clone();
    double_click_gesture.connect_pressed(move |_, n_press, _, _| {
        if n_press == 2 {
            if is_dir {
                nav_c(target_path.clone());
            } else {
                let uri = format!("file://{}", target_path.to_string_lossy());
                let _ = gtk4::gio::AppInfo::launch_default_for_uri(&uri, gtk4::gio::AppLaunchContext::NONE);
            }
        }
    });
    flow_child.add_controller(double_click_gesture);

    flow_child
}
