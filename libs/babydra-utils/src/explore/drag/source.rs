use gtk4::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use babydra_common::load_cropped_square_pixbuf;
use gtk4::gdk::FileList;

/// Creates a DragSource for a file path, setting preview thumbnails or fallback icons.
pub fn create_drag_source(
    path: &PathBuf,
    icon_name: &str,
    selected_paths: Rc<RefCell<Vec<PathBuf>>>,
) -> gtk4::DragSource {
    let drag_source = gtk4::DragSource::new();
    drag_source.set_actions(gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY);
    let path_clone = path.clone();
    let sel_paths = selected_paths.clone();

    drag_source.connect_prepare(move |_, _, _| {
        // If the path being dragged is part of the current selection, drag the whole selection.
        // Otherwise, drag only the single item.
        let targets = {
            let s = sel_paths.borrow();
            if s.contains(&path_clone) {
                s.clone()
            } else {
                vec![path_clone.clone()]
            }
        };

        let gio_files: Vec<gtk4::gio::File> = targets.iter()
            .map(|p| gtk4::gio::File::for_path(p))
            .collect();
            
        let file_list = FileList::from_array(&gio_files);
        Some(gtk4::gdk::ContentProvider::for_value(&file_list.to_value()))
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
