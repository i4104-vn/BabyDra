use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align, FlowBoxChild, Picture};
use babydra_common::{FileEntry, load_cropped_square_pixbuf};

/// Creates a generic grid card representation for a file or directory.
pub fn create_grid_file_item(
    idx: usize,
    entry: &FileEntry,
    on_right_click: impl Fn(&gtk4::Widget, f64, f64) + 'static,
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
            let texture = gtk4::gdk::Texture::for_pixbuf(&pixbuf);
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
    let widget_clone = item_box.clone();
    gesture.connect_pressed(move |gesture, _, x, y| {
        gesture.set_state(gtk4::EventSequenceState::Claimed);
        on_right_click(widget_clone.upcast_ref(), x, y);
    });
    item_box.add_controller(gesture);

    // Add Drag Source to item_box
    let drag_source = gtk4::DragSource::new();
    drag_source.set_actions(gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY);
    let path_clone = entry.path.clone();
    drag_source.connect_prepare(move |_, _, _| {
        let file = gtk4::gio::File::for_path(&path_clone);
        Some(gtk4::gdk::ContentProvider::for_value(&file.to_value()))
    });

    if has_preview {
        if let Ok(pixbuf) = load_cropped_square_pixbuf(&entry.path, 85) {
            let texture = gtk4::gdk::Texture::for_pixbuf(&pixbuf);
            drag_source.set_icon(Some(&texture), 42, 42);
        } else {
            let display = gtk4::gdk::Display::default().unwrap();
            let icon_theme = gtk4::IconTheme::for_display(&display);
            let paintable = icon_theme.lookup_icon(
                &entry.icon_name,
                &[],
                48,
                1,
                gtk4::TextDirection::Ltr,
                gtk4::IconLookupFlags::empty(),
            );
            drag_source.set_icon(Some(&paintable), 24, 24);
        }
    } else {
        let display = gtk4::gdk::Display::default().unwrap();
        let icon_theme = gtk4::IconTheme::for_display(&display);
        let paintable = icon_theme.lookup_icon(
            &entry.icon_name,
            &[],
            48,
            1,
            gtk4::TextDirection::Ltr,
            gtk4::IconLookupFlags::empty(),
        );
        drag_source.set_icon(Some(&paintable), 24, 24);
    }

    item_box.add_controller(drag_source);

    // If directory, add Drop Target to item_box
    if matches!(entry.file_type, babydra_common::FileType::Directory) {
        let drop_target = gtk4::DropTarget::new(
            gtk4::gio::File::static_type(),
            gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY,
        );
        let dest_path = entry.path.clone();
        drop_target.connect_drop(move |_, value, _, _| {
            if let Ok(file) = value.get::<gtk4::gio::File>() {
                if let Some(src_path) = file.path() {
                    let dest = dest_path.join(src_path.file_name().unwrap());
                    if src_path != dest {
                        let _ = std::fs::rename(&src_path, &dest);
                    }
                }
                return true;
            }
            false
        });
        item_box.add_controller(drop_target);
    }

    let flow_child = FlowBoxChild::new();
    flow_child.set_child(Some(&item_box));
    flow_child.set_property("name", &format!("{}", idx));
    flow_child
}
