use babydra_core::FileEntry;
use gtk4::prelude::*;
use gtk4::FlowBoxChild;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// Creates an explore-specific flowbox child grid cell representing a single file/folder.
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
    let flow_child = babydra_ui_kit::components::explore::create_grid_file(
        idx,
        entry,
        selected_paths,
        move |widget, x, y| {
            let mut target_paths = sel_paths.borrow().clone();
            if !target_paths.contains(&target_entry.path) {
                target_paths = vec![target_entry.path.clone()];
            }

            if let Some(win) = widget
                .root()
                .and_then(|r| r.downcast::<gtk4::Window>().ok())
            {
                babydra_ui_kit::components::explore::context_menu::show_for_file(
                    widget,
                    x,
                    y,
                    target_paths,
                    cp.clone(),
                    nav.clone(),
                    &win,
                );
            }
        },
    );

    // Dim item if it's currently in the cut clipboard
    let is_cut = babydra_ui_kit::components::explore::CLIPBOARD.with(|cb| {
        cb.borrow()
            .as_ref()
            .map(|(paths, cut)| *cut && paths.contains(&entry.path))
            .unwrap_or(false)
    });
    if is_cut {
        if let Some(child_widget) = flow_child.child() {
            child_widget.add_css_class("cut-item");
        }
    }

    let double_click_gesture = gtk4::GestureClick::new();
    double_click_gesture.set_button(1);
    let target_path = entry.path.clone();
    let is_dir = matches!(entry.file_type, babydra_core::FileType::Directory);
    let nav_c = nav_callback.clone();
    double_click_gesture.connect_pressed(move |_, n_press, _, _| {
        let settings = babydra_core::load_explore_cfg();
        let trigger = if settings.double_click_to_open {
            n_press == 2
        } else {
            n_press == 1
        };
        if trigger {
            if is_dir {
                nav_c(target_path.clone());
            } else {
                babydra_ui_kit::components::explore::dialogs::launch_file_or_open_with(
                    &target_path,
                    None::<&gtk4::Window>,
                );
            }
        }
    });
    flow_child.add_controller(double_click_gesture);

    flow_child
}
