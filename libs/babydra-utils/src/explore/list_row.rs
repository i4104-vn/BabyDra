use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align, ListBoxRow};
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use babydra_common::FileEntry;

/// Creates a list row representational component for a file/folder entry.
pub fn create_list_row(
    idx: usize,
    entry: &FileEntry,
    selected_paths: Rc<RefCell<Vec<PathBuf>>>,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    on_right_click: impl Fn(&gtk4::Widget, f64, f64) + 'static,
) -> ListBoxRow {
    let item_box = Box::new(Orientation::Horizontal, 12);
    item_box.set_css_classes(&["list-row"]);
    item_box.set_margin_top(2);
    item_box.set_margin_bottom(2);
    item_box.set_margin_start(6);
    item_box.set_margin_end(6);

    let img = crate::ui::icon::get_system_or_file_icon(&entry.icon_name, "text-x-generic");
    img.set_pixel_size(24);
    item_box.append(&img);

    let lbl_name = Label::builder()
        .label(&entry.display_name)
        .halign(Align::Start)
        .hexpand(true)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .build();
    item_box.append(&lbl_name);

    // File size info
    let size_str = if matches!(entry.file_type, babydra_common::FileType::Directory) {
        "--".to_string()
    } else {
        crate::explore::format_size(entry.size)
    };
    let lbl_size = Label::new(Some(&size_str));
    lbl_size.set_css_classes(&["list-col-meta"]);
    lbl_size.set_size_request(80, -1);
    lbl_size.set_halign(Align::End);
    lbl_size.set_tooltip_text(Some("Size"));
    item_box.append(&lbl_size);

    // Permissions info
    let perm_str = format!("{:o}", entry.permissions & 0o777);
    let lbl_perm = Label::new(Some(&perm_str));
    lbl_perm.set_css_classes(&["list-col-meta"]);
    lbl_perm.set_size_request(80, -1);
    lbl_perm.set_halign(Align::End);
    lbl_perm.set_tooltip_text(Some("Permissions"));
    item_box.append(&lbl_perm);

    // Modified info
    let mod_str = if let Some(mtime) = entry.modified {
        crate::explore::format_date(mtime)
    } else {
        "--".to_string()
    };
    let lbl_date = Label::new(Some(&mod_str));
    lbl_date.set_css_classes(&["list-col-meta"]);
    lbl_date.set_size_request(140, -1);
    lbl_date.set_halign(Align::End);
    lbl_date.set_tooltip_text(Some("Modified Date"));
    item_box.append(&lbl_date);

    // Attach right click gesture to list item_box
    let gesture = gtk4::GestureClick::new();
    gesture.set_button(3);
    let widget_clone = item_box.clone();
    gesture.connect_pressed(move |gesture, _, x, y| {
        gesture.set_state(gtk4::EventSequenceState::Claimed);
        on_right_click(widget_clone.upcast_ref(), x, y);
    });
    item_box.add_controller(gesture);

    // Add Drag Source to item_box
    let drag_source = crate::explore::create_drag_source(&entry.path, &entry.icon_name, selected_paths);
    item_box.add_controller(drag_source);

    // If directory, add Drop Target to item_box
    if matches!(entry.file_type, babydra_common::FileType::Directory) {
        let drop_target = crate::explore::create_dir_drop_target(entry.path.clone());
        item_box.add_controller(drop_target);
    }

    // Dim item if it's currently in the cut clipboard
    let is_cut = crate::explore::CLIPBOARD.with(|cb| {
        cb.borrow().as_ref().map(|(paths, cut)| *cut && paths.contains(&entry.path)).unwrap_or(false)
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
    let is_dir = matches!(entry.file_type, babydra_common::FileType::Directory);
    double_click_gesture.connect_pressed(move |_, n_press, _, _| {
        if n_press == 2 {
            if is_dir {
                nav_callback(target_path.clone());
            } else {
                let uri = format!("file://{}", target_path.to_string_lossy());
                let _ = gtk4::gio::AppInfo::launch_default_for_uri(&uri, gtk4::gio::AppLaunchContext::NONE);
            }
        }
    });
    list_row.add_controller(double_click_gesture);

    list_row
}
