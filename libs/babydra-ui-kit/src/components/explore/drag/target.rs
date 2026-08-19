use gtk4::gdk::FileList;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// Creates a DropTarget for dropping files/folders into a target directory.
pub fn create_drop_target(dest_path: PathBuf) -> gtk4::DropTarget {
    create_drop_nav(dest_path, None)
}

/// Creates a DropTarget for dropping files/folders into a target directory with optional navigation refresh.
pub fn create_drop_nav(
    dest_path: PathBuf,
    nav_callback: Option<Rc<dyn Fn(PathBuf)>>,
) -> gtk4::DropTarget {
    let drop_target = gtk4::DropTarget::new(
        FileList::static_type(),
        gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY,
    );
    let nav_cb = nav_callback.clone();
    drop_target.connect_drop(move |_, value, _, _| {
        if let Ok(file_list) = value.get::<FileList>() {
            let sources: Vec<PathBuf> = file_list.files().iter().filter_map(|f| f.path()).collect();
            if !sources.is_empty() {
                let dest_dir = dest_path.clone();
                let nav = nav_cb.clone();
                let is_trash = dest_dir.to_string_lossy().contains("Trash");

                glib::spawn_future_local(async move {
                    let mut destinations = Vec::new();
                    let mut actual_sources = Vec::new();
                    let mut refresh_path = None;

                    for src in sources {
                        if refresh_path.is_none() {
                            refresh_path = src.parent().map(|p| p.to_path_buf());
                        }
                        if is_trash {
                            if let Ok(_) = babydra_core::send_to_trash(src.clone()).await {
                                actual_sources.push(src);
                            }
                        } else if let Some(filename) = src.file_name() {
                            let dest = dest_dir.join(filename);
                            if src != dest && !dest_dir.starts_with(&src) {
                                if let Ok(_) =
                                    babydra_core::move_path(src.clone(), dest.clone()).await
                                {
                                    destinations.push(dest);
                                    actual_sources.push(src);
                                }
                            }
                        }
                    }
                    if !actual_sources.is_empty() {
                        if !is_trash && !destinations.is_empty() {
                            crate::components::explore::context_menu::clipboard::UNDO_STACK.with(|stack| {
                                stack.borrow_mut().push(
                                    crate::components::explore::context_menu::clipboard::UndoOperation {
                                        is_cut: true,
                                        sources: actual_sources,
                                        destinations,
                                    },
                                );
                            });
                        }
                        crate::components::explore::context_menu::clipboard::apply_cut_everywhere(&[]);
                        if let Some(nav_fn) = nav {
                            if let Some(rp) = refresh_path {
                                nav_fn(rp);
                            }
                        }
                    }
                });
                return true;
            }
        }
        false
    });
    drop_target
}

/// Creates a DropTarget for dropping files/folders into a dynamic background path.
pub fn create_bg_drop(current_path: Rc<RefCell<PathBuf>>) -> gtk4::DropTarget {
    let drop_target = gtk4::DropTarget::new(
        FileList::static_type(),
        gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY,
    );
    drop_target.connect_drop(move |_, value, _, _| {
        let dest_dir = current_path.borrow().clone();
        if let Ok(file_list) = value.get::<FileList>() {
            let sources: Vec<PathBuf> = file_list.files().iter().filter_map(|f| f.path()).collect();
            if !sources.is_empty() {
                glib::spawn_future_local(async move {
                    let mut destinations = Vec::new();
                    let mut actual_sources = Vec::new();
                    for src in sources {
                        if let Some(filename) = src.file_name() {
                            let dest = dest_dir.join(filename);
                            if src != dest && !dest_dir.starts_with(&src) {
                                if let Ok(_) =
                                    babydra_core::move_path(src.clone(), dest.clone()).await
                                {
                                    destinations.push(dest);
                                    actual_sources.push(src);
                                }
                            }
                        }
                    }
                    if !destinations.is_empty() {
                        crate::components::explore::context_menu::clipboard::UNDO_STACK.with(
                            |stack| {
                                stack.borrow_mut().push(
                                crate::components::explore::context_menu::clipboard::UndoOperation {
                                    is_cut: true,
                                    sources: actual_sources,
                                    destinations,
                                },
                            );
                            },
                        );
                        crate::components::explore::context_menu::clipboard::apply_cut_everywhere(
                            &[],
                        );
                    }
                });
                return true;
            }
        }
        false
    });
    drop_target
}
