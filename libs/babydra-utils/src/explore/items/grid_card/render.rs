use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align, Picture};
use babydra_common::{FileEntry, load_cropped_square_pixbuf};

/// Builds the visual GTK layout box for a grid item card.
pub fn build_grid_card_ui(entry: &FileEntry) -> Box {
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
        let overlay = gtk4::Overlay::new();
        
        let temp_icon = crate::ui::icon::get_system_or_file_icon(&entry.icon_name, "text-x-generic");
        temp_icon.set_pixel_size(52);
        temp_icon.set_halign(Align::Center);
        temp_icon.set_valign(Align::Center);
        temp_icon.set_hexpand(true);
        temp_icon.set_vexpand(true);
        
        overlay.set_child(Some(&temp_icon));
        item_box.append(&overlay);

        struct SendWrapper<T>(T);
        unsafe impl<T> Send for SendWrapper<T> {}

        let path_clone = entry.path.clone();
        let overlay_c = overlay.clone();
        
        glib::spawn_future_local(async move {
            let res = tokio::task::spawn_blocking(move || {
                load_cropped_square_pixbuf(&path_clone, 85).map(|pb| SendWrapper(pb))
            }).await;

            if let Ok(Ok(wrapper)) = res {
                let pixbuf = wrapper.0;
                let texture = gtk4::gdk::Texture::for_pixbuf(&pixbuf);
                let picture = Picture::for_paintable(&texture);
                picture.set_size_request(85, 85);
                picture.set_halign(Align::Center);
                picture.set_valign(Align::Center);
                picture.set_hexpand(true);
                picture.set_vexpand(true);
                picture.set_content_fit(gtk4::ContentFit::Cover);
                overlay_c.set_child(Some(&picture));
            }
        });
    } else {
        let icon = crate::ui::icon::get_system_or_file_icon(&entry.icon_name, "text-x-generic");
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
    item_box
}
