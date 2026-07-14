use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align, FlowBoxChild, Image, Picture};
use std::path::PathBuf;
use std::rc::Rc;
use babydra_common::{FileEntry, load_cropped_square_pixbuf};

/// Helper to create flow child elements for grid view
pub fn create_flow_child(
    idx: usize,
    entry: &FileEntry,
    current_path: &PathBuf,
    nav_callback: &Rc<dyn Fn(PathBuf)>,
) -> FlowBoxChild {
    let item_box = Box::new(Orientation::Vertical, 4);
    item_box.set_size_request(100, 110);
    item_box.set_css_classes(&["file-item"]);
    item_box.set_halign(Align::Fill);
    item_box.set_valign(Align::Fill);

    let has_preview = if let Some(ext) = entry.path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        matches!(ext_str.as_str(), "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg")
    } else {
        false
    };

    if has_preview {
        if let Ok(pixbuf) = load_cropped_square_pixbuf(&entry.path, 85) {
            let texture = gdk4::Texture::for_pixbuf(&pixbuf);
            let picture = Picture::for_paintable(&texture);
            picture.set_size_request(85, 85);
            picture.set_halign(Align::Center);
            picture.set_valign(Align::Center);
            picture.set_hexpand(true);
            picture.set_vexpand(true);
            picture.set_content_fit(gtk4::ContentFit::Cover);
            item_box.append(&picture);
        } else {
            let icon = babydra_common::icon::get_system_or_file_icon(&entry.icon_name, "text-x-generic");
            icon.set_pixel_size(52);
            icon.set_halign(Align::Center);
            icon.set_valign(Align::Center);
            icon.set_hexpand(true);
            icon.set_vexpand(true);
            item_box.append(&icon);
        }
    } else {
        let icon = babydra_common::icon::get_system_or_file_icon(&entry.icon_name, "text-x-generic");
        icon.set_pixel_size(52);
        icon.set_halign(Align::Center);
        icon.set_valign(Align::Center);
        icon.set_hexpand(true);
        icon.set_vexpand(true);
        item_box.append(&icon);
    };

    let lbl = Label::builder()
        .label(&entry.display_name)
        .max_width_chars(12)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .halign(Align::Center)
        .hexpand(true)
        .build();
    lbl.add_css_class("file-item-label");

    item_box.append(&lbl);

    // Attach right click gesture to item_box
    let gesture = gtk4::GestureClick::new();
    gesture.set_button(3);
    let target_entry = entry.clone();
    let cp = current_path.clone();
    let widget_clone = item_box.clone();
    let nav = nav_callback.clone();
    gesture.connect_pressed(move |gesture, _, x, y| {
        gesture.set_state(gtk4::EventSequenceState::Claimed);
        crate::widgets::context_menu::show_for_file(
            widget_clone.upcast_ref(),
            x,
            y,
            target_entry.clone(),
            cp.clone(),
            nav.clone(),
        );
    });
    item_box.add_controller(gesture);

    let flow_child = FlowBoxChild::new();
    flow_child.set_child(Some(&item_box));
    flow_child.set_property("name", &format!("{}", idx));
    flow_child
}
