use gtk4::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use gtk4::gdk::FileList;

/// Creates a DropTarget for dropping files/folders into a target directory.
pub fn create_dir_drop_target(dest_path: PathBuf) -> gtk4::DropTarget {
    let drop_target = gtk4::DropTarget::new(
        FileList::static_type(),
        gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY,
    );
    drop_target.connect_drop(move |_, value, _, _| {
        if let Ok(file_list) = value.get::<FileList>() {
            let mut destinations = Vec::new();
            let mut sources = Vec::new();
            for file in file_list.files() {
                if let Some(src_path) = file.path() {
                    let dest = dest_path.join(src_path.file_name().unwrap());
                    if src_path != dest {
                        if std::fs::rename(&src_path, &dest).is_ok() {
                            destinations.push(dest);
                            sources.push(src_path);
                        }
                    }
                }
            }
            if !destinations.is_empty() {
                crate::explore::context_menu::clipboard::UNDO_STACK.with(|stack| {
                    stack.borrow_mut().push(crate::explore::context_menu::clipboard::UndoOperation {
                        is_cut: true,
                        sources,
                        destinations,
                    });
                });
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
            let mut destinations = Vec::new();
            let mut sources = Vec::new();
            for file in file_list.files() {
                if let Some(src_path) = file.path() {
                    let dest = dest_dir.join(src_path.file_name().unwrap());
                    if src_path != dest {
                        if std::fs::rename(&src_path, &dest).is_ok() {
                            destinations.push(dest);
                            sources.push(src_path);
                        }
                    }
                }
            }
            if !destinations.is_empty() {
                crate::explore::context_menu::clipboard::UNDO_STACK.with(|stack| {
                    stack.borrow_mut().push(crate::explore::context_menu::clipboard::UndoOperation {
                        is_cut: true,
                        sources,
                        destinations,
                    });
                });
            }
            return true;
        }
        false
    });
    drop_target
}
