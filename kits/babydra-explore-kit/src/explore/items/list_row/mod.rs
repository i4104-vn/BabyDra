use babydra_core::FileEntry;
use gtk4::prelude::*;
use gtk4::ListBoxRow;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

mod render;

/// Creates a list row representational component for a file/folder entry.
pub fn create_list_row(
    idx: usize,
    entry: &FileEntry,
    selected_paths: Rc<RefCell<Vec<PathBuf>>>,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    on_right_click: impl Fn(&gtk4::Widget, f64, f64) + 'static,
) -> ListBoxRow {
    let item_box = render::build_list_row_ui(entry);

    // Attach right click gesture to list item_box
    let gesture = gtk4::GestureClick::new();
    gesture.set_button(3);
    let widget_clone = item_box.clone();
    gesture.connect_pressed(move |gesture, _, x, y| {
        gesture.set_state(gtk4::EventSequenceState::Claimed);
        on_right_click(widget_clone.upcast_ref(), x, y);
    });
    item_box.add_controller(gesture);

    let drag_source =
        crate::explore::create_drag_source(&entry.path, &entry.icon_name, selected_paths);
    item_box.add_controller(drag_source);

    // If directory, add Drop Target to item_box
    if matches!(entry.file_type, babydra_core::FileType::Directory) {
        let drop_target = crate::explore::create_dir_drop_target(entry.path.clone());
        item_box.add_controller(drop_target);
    }

    // Dim item if it's currently in the cut clipboard
    let is_cut = crate::explore::CLIPBOARD.with(|cb| {
        cb.borrow()
            .as_ref()
            .map(|(paths, cut)| *cut && paths.contains(&entry.path))
            .unwrap_or(false)
    });
    if is_cut {
        item_box.add_css_class("cut-item");
    }

    let list_row = ListBoxRow::new();
    list_row.set_child(Some(&item_box));
    list_row.set_property("name", &format!("{}", idx));
    list_row.set_widget_name(&entry.path.to_string_lossy());

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
    list_row.add_controller(double_click_gesture);

    list_row
}
