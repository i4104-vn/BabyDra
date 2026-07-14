use gtk4::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use babydra_common::load_cropped_square_pixbuf;
#[allow(unused_imports)]
use gtk4::gdk::FileList;

/// Creates a DragSource for a file path, setting preview thumbnails or fallback icons.
pub fn create_drag_source(path: &PathBuf, icon_name: &str) -> gtk4::DragSource {
    let drag_source = gtk4::DragSource::new();
    drag_source.set_actions(gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY);
    let path_clone = path.clone();
    drag_source.connect_prepare(move |ds, _, _| {
        let mut target_paths = vec![path_clone.clone()];
        
        if let Some(widget) = ds.widget() {
            let mut list_row_or_flow_child = None;
            let mut next = widget.parent();
            while let Some(w) = next {
                if w.downcast_ref::<gtk4::ListBoxRow>().is_some() || w.downcast_ref::<gtk4::FlowBoxChild>().is_some() {
                    list_row_or_flow_child = Some(w);
                    break;
                }
                next = w.parent();
            }
            
            if let Some(child) = list_row_or_flow_child {
                if let Some(list_row) = child.downcast_ref::<gtk4::ListBoxRow>() {
                    if let Some(listbox) = list_row.parent().and_then(|p| p.downcast::<gtk4::ListBox>().ok()) {
                        let selected_rows = listbox.selected_rows();
                        if selected_rows.contains(list_row) {
                            let mut paths = Vec::new();
                            for r in selected_rows {
                                let path_str = r.widget_name();
                                let path = PathBuf::from(path_str.to_string());
                                if path.is_absolute() {
                                    paths.push(path);
                                }
                            }
                            if !paths.is_empty() {
                                target_paths = paths;
                            }
                        }
                    }
                } else if let Some(flow_child) = child.downcast_ref::<gtk4::FlowBoxChild>() {
                    if let Some(flowbox) = flow_child.parent().and_then(|p| p.downcast::<gtk4::FlowBox>().ok()) {
                        let selected_in_this_fb = flowbox.selected_children();
                        if selected_in_this_fb.contains(flow_child) {
                            if let Some(grid_container) = flowbox.parent().and_then(|p| p.downcast::<gtk4::Box>().ok()) {
                                let mut paths = Vec::new();
                                let mut sibling = grid_container.first_child();
                                while let Some(c) = sibling {
                                    if let Some(fb) = c.downcast_ref::<gtk4::FlowBox>() {
                                        for item in fb.selected_children() {
                                            let path_str = item.widget_name();
                                            let path = PathBuf::from(path_str.to_string());
                                            if path.is_absolute() {
                                                paths.push(path);
                                            }
                                        }
                                    }
                                    sibling = c.next_sibling();
                                }
                                if !paths.is_empty() {
                                    target_paths = paths;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        let gio_files: Vec<gtk4::gio::File> = target_paths.iter()
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

/// Creates a DropTarget for dropping files/folders into a target directory.
pub fn create_dir_drop_target(dest_path: PathBuf) -> gtk4::DropTarget {
    let drop_target = gtk4::DropTarget::new(
        FileList::static_type(),
        gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY,
    );
    drop_target.connect_drop(move |_, value, _, _| {
        if let Ok(file_list) = value.get::<FileList>() {
            for file in file_list.files() {
                if let Some(src_path) = file.path() {
                    let dest = dest_path.join(src_path.file_name().unwrap());
                    if src_path != dest {
                        let _ = std::fs::rename(&src_path, &dest);
                    }
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
        FileList::static_type(),
        gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY,
    );
    drop_target.connect_drop(move |_, value, _, _| {
        let dest_dir = current_path.borrow().clone();
        if let Ok(file_list) = value.get::<FileList>() {
            for file in file_list.files() {
                if let Some(src_path) = file.path() {
                    let dest = dest_dir.join(src_path.file_name().unwrap());
                    if src_path != dest {
                        let _ = std::fs::rename(&src_path, &dest);
                    }
                }
            }
            return true;
        }
        false
    });
    drop_target
}
