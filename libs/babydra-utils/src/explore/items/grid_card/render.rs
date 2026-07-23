use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align, Picture};
use babydra_common::{FileEntry, load_cropped_square_pixbuf};

/// Builds the visual GTK layout box for a grid item card.
pub fn build_grid_card_ui(entry: &FileEntry) -> Box {
    let item_box = Box::new(Orientation::Vertical, 4);
    item_box.set_size_request(114, 114);
    item_box.set_css_classes(&["file-item"]);
    item_box.set_halign(Align::Center);
    item_box.set_valign(Align::Center);
    item_box.set_hexpand(false);
    item_box.set_vexpand(false);

    let icon_frame = Box::new(Orientation::Vertical, 0);
    icon_frame.set_size_request(68, 68);
    icon_frame.set_halign(Align::Center);
    icon_frame.set_valign(Align::Center);
    icon_frame.set_hexpand(false);
    icon_frame.set_vexpand(false);

    let has_preview = if let Some(ext) = entry.path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        matches!(ext_str.as_str(), "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg")
    } else {
        false
    };

    if has_preview {
        let overlay = gtk4::Overlay::new();
        overlay.set_size_request(68, 68);
        overlay.set_halign(Align::Center);
        overlay.set_valign(Align::Center);
        
        let temp_icon = crate::ui::icon::get_system_or_file_icon(&entry.icon_name, "text-x-generic");
        temp_icon.set_pixel_size(58);
        temp_icon.set_halign(Align::Center);
        temp_icon.set_valign(Align::Center);
        
        overlay.set_child(Some(&temp_icon));
        icon_frame.append(&overlay);

        struct SendWrapper<T>(T);
        unsafe impl<T> Send for SendWrapper<T> {}

        let path_clone = entry.path.clone();
        let overlay_c = overlay.clone();
        
        glib::spawn_future_local(async move {
            let res = tokio::task::spawn_blocking(move || {
                load_cropped_square_pixbuf(&path_clone, 68).map(|pb| SendWrapper(pb))
            }).await;

            if let Ok(Ok(wrapper)) = res {
                let pixbuf = wrapper.0;
                let texture = gtk4::gdk::Texture::for_pixbuf(&pixbuf);
                let picture = Picture::for_paintable(&texture);
                picture.set_size_request(68, 68);
                picture.set_halign(Align::Center);
                picture.set_valign(Align::Center);
                picture.set_content_fit(gtk4::ContentFit::Cover);
                overlay_c.set_child(Some(&picture));
            }
        });
    } else {
        let icon = crate::ui::icon::get_system_or_file_icon(&entry.icon_name, "text-x-generic");
        icon.set_pixel_size(58);
        icon.set_halign(Align::Center);
        icon.set_valign(Align::Center);
        icon_frame.append(&icon);
    };

    item_box.append(&icon_frame);

    let lbl = Label::builder()
        .label(&entry.display_name)
        .max_width_chars(11)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .halign(Align::Center)
        .valign(Align::Center)
        .hexpand(false)
        .vexpand(false)
        .build();
    lbl.add_css_class("file-item-label");

    item_box.append(&lbl);
    item_box
}
