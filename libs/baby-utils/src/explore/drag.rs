use gtk4::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use babydra_common::load_cropped_square_pixbuf;

/// Creates a DragSource for a file path, setting preview thumbnails or fallback icons.
pub fn create_drag_source(path: &PathBuf, icon_name: &str) -> gtk4::DragSource {
    let drag_source = gtk4::DragSource::new();
    drag_source.set_actions(gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY);
    let path_clone = path.clone();
    drag_source.connect_prepare(move |_, _, _| {
        let file = gtk4::gio::File::for_path(&path_clone);
        Some(gtk4::gdk::ContentProvider::for_value(&file.to_value()))
    });

    let has_preview = if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        matches!(ext_str.as_str(), "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg")
    } else {
        false
    };

    if has_preview {
        if let Ok(pixbuf) = load_cropped_square_pixbuf(path, 85) {
            let texture = gtk4::gdk::Texture::for_pixbuf(&pixbuf);
            drag_source.set_icon(Some(&texture), 42, 42);
            return drag_source;
        }
    }

    let display = gtk4::gdk::Display::default().unwrap();
    let icon_theme = gtk4::IconTheme::for_display(&display);
    let paintable = icon_theme.lookup_icon(
        icon_name,
        &[],
        48,
        1,
        gtk4::TextDirection::Ltr,
        gtk4::IconLookupFlags::empty(),
    );
    drag_source.set_icon(Some(&paintable), 24, 24);

    drag_source
}

/// Creates a DropTarget for dropping files/folders into a target directory.
pub fn create_dir_drop_target(dest_path: PathBuf) -> gtk4::DropTarget {
    let drop_target = gtk4::DropTarget::new(
        gtk4::gio::File::static_type(),
        gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY,
    );
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
    drop_target
}

/// Creates a DropTarget for dropping files/folders into a dynamic background path.
pub fn create_background_drop_target(current_path: Rc<RefCell<PathBuf>>) -> gtk4::DropTarget {
    let drop_target = gtk4::DropTarget::new(
        gtk4::gio::File::static_type(),
        gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY,
    );
    drop_target.connect_drop(move |_, value, _, _| {
        let dest_dir = current_path.borrow().clone();
        if let Ok(file) = value.get::<gtk4::gio::File>() {
            if let Some(src_path) = file.path() {
                let dest = dest_dir.join(src_path.file_name().unwrap());
                if src_path != dest {
                    let _ = std::fs::rename(&src_path, &dest);
                }
            }
            return true;
        }
        false
    });
    drop_target
}
