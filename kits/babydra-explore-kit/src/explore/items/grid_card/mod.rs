use babydra_core::FileEntry;
use gtk4::prelude::*;
use gtk4::FlowBoxChild;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

mod render;

/// Creates a generic grid card representation for a file or directory.
pub fn create_grid_file_item(
    idx: usize,
    entry: &FileEntry,
    selected_paths: Rc<RefCell<Vec<PathBuf>>>,
    on_right_click: impl Fn(&gtk4::Widget, f64, f64) + 'static,
) -> FlowBoxChild {
    let item_box = render::build_grid_card_ui(entry);

    // Attach right click gesture to item_box
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

    let flow_child = FlowBoxChild::new();
    flow_child.set_size_request(114, 114);
    flow_child.set_hexpand(false);
    flow_child.set_vexpand(false);
    flow_child.set_halign(gtk4::Align::Center);
    flow_child.set_valign(gtk4::Align::Center);
    flow_child.set_child(Some(&item_box));
    flow_child.set_property("name", &format!("{}", idx));
    flow_child.set_widget_name(&entry.path.to_string_lossy());
    flow_child
}
