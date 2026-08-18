//! Drag and Drop subsystem for Desktop: DragSource, Desktop DropTarget, and Folder DropTarget.

use crate::state::DesktopState;
use babydra_core::load_cropped_square_pixbuf;
use gtk4::gdk::FileList;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// Creates a DragSource on an icon widget for dragging files to other grid slots, folders, or external apps.
pub fn create_icon_drag_source(
    path: &PathBuf,
    icon_name: &str,
    selected_paths: Rc<RefCell<Vec<PathBuf>>>,
) -> gtk4::DragSource {
    let drag_source = gtk4::DragSource::new();
    drag_source.set_actions(gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY);

    let path_clone = path.clone();
    let sel_paths = selected_paths.clone();

    drag_source.connect_prepare(move |_, _, _| {
        let targets = {
            let s = sel_paths.borrow();
            if s.contains(&path_clone) {
                s.clone()
            } else {
                vec![path_clone.clone()]
            }
        };

        let gio_files: Vec<gtk4::gio::File> = targets
            .iter()
            .map(|p| gtk4::gio::File::for_path(p))
            .collect();

        let file_list = FileList::from_array(&gio_files);
        Some(gtk4::gdk::ContentProvider::for_value(&file_list.to_value()))
    });

    let has_preview = if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        matches!(
            ext_str.as_str(),
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg"
        )
    } else {
        false
    };

    if has_preview {
        if let Ok(pixbuf) = load_cropped_square_pixbuf(path, 64) {
            let texture = gtk4::gdk::Texture::for_pixbuf(&pixbuf);
            drag_source.set_icon(Some(&texture), 32, 32);
            return drag_source;
        }
    }

    if let Some(display) = gtk4::gdk::Display::default() {
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
    }

    drag_source
}

/// Creates a DropTarget on the desktop background to handle internal repositioning and external file drops.
pub fn create_desktop_drop_target(
    state: Rc<RefCell<DesktopState>>,
    refresh_cb: Rc<dyn Fn()>,
) -> gtk4::DropTarget {
    let drop_target = gtk4::DropTarget::new(
        FileList::static_type(),
        gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY,
    );

    let state_drop = state.clone();
    let ref_cb = refresh_cb.clone();

    drop_target.connect_drop(move |_, value, x, y| {
        let desktop_dir = DesktopState::desktop_dir();
        if let Ok(file_list) = value.get::<FileList>() {
            let sources: Vec<PathBuf> = file_list
                .files()
                .iter()
                .filter_map(|f| f.path())
                .collect();

            if sources.is_empty() {
                return false;
            }

            let mut internal_sources = Vec::new();
            let mut external_sources = Vec::new();

            for src in sources {
                if src.parent() == Some(&desktop_dir) {
                    internal_sources.push(src);
                } else {
                    external_sources.push(src);
                }
            }

            // 1. Internal Repositioning (Drag icon to a new spot on the desktop)
            if !internal_sources.is_empty() {
                let spacing = state_drop.borrow().config.grid_spacing.max(64) as i32;
                let base_x = (x as i32 / spacing) * spacing + 16;
                let mut base_y = (y as i32 / spacing) * spacing + 16;

                for src in internal_sources {
                    if let Some(file_name) = src.file_name().and_then(|n| n.to_str()) {
                        state_drop.borrow_mut().set_icon_position(
                            file_name.to_string(),
                            base_x,
                            base_y,
                        );
                        base_y += spacing; // Stack multiple moved items vertically
                    }
                }
                ref_cb();
            }

            // 2. External Files Ingestion (Copy files dropped from external apps to ~/Desktop)
            if !external_sources.is_empty() {
                let ref_cb_inner = ref_cb.clone();
                let state_inner = state_drop.clone();
                let start_x = x as i32;
                let start_y = y as i32;

                glib::spawn_future_local(async move {
                    let spacing = state_inner.borrow().config.grid_spacing.max(64) as i32;
                    let cur_x = (start_x / spacing) * spacing + 16;
                    let mut cur_y = (start_y / spacing) * spacing + 16;

                    for src in external_sources {
                        if let Some(filename) = src.file_name().map(|n| n.to_os_string()) {
                            let dest = desktop_dir.join(&filename);
                            if src != dest {
                                if babydra_core::copy_path(src, dest).await.is_ok() {
                                    if let Some(name_str) = filename.to_str() {
                                        state_inner.borrow_mut().set_icon_position(
                                            name_str.to_string(),
                                            cur_x,
                                            cur_y,
                                        );
                                        cur_y += spacing;
                                    }
                                }
                            }
                        }
                    }
                    ref_cb_inner();
                });
            }

            return true;
        }
        false
    });

    drop_target
}

/// Creates a DropTarget on a directory icon on desktop to allow dropping files inside that directory.
pub fn create_folder_drop_target(
    target_folder: PathBuf,
    widget: gtk4::Box,
    refresh_cb: Rc<dyn Fn()>,
) -> gtk4::DropTarget {
    let drop_target = gtk4::DropTarget::new(
        FileList::static_type(),
        gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY,
    );

    let w_enter = widget.clone();
    drop_target.connect_enter(move |_, _, _| {
        w_enter.add_css_class("drop-target-hover");
        gtk4::gdk::DragAction::MOVE
    });

    let w_leave = widget.clone();
    drop_target.connect_leave(move |_| {
        w_leave.remove_css_class("drop-target-hover");
    });

    let w_drop = widget.clone();
    let ref_cb = refresh_cb.clone();
    let dest_dir = target_folder.clone();

    drop_target.connect_drop(move |_, value, _, _| {
        w_drop.remove_css_class("drop-target-hover");
        if let Ok(file_list) = value.get::<FileList>() {
            let sources: Vec<PathBuf> = file_list
                .files()
                .iter()
                .filter_map(|f| f.path())
                .collect();

            if !sources.is_empty() {
                let ref_cb_inner = ref_cb.clone();
                let dest_folder = dest_dir.clone();

                glib::spawn_future_local(async move {
                    for src in sources {
                        if let Some(filename) = src.file_name() {
                            let dest = dest_folder.join(filename);
                            if src != dest {
                                let _ = babydra_core::move_path(src, dest).await;
                            }
                        }
                    }
                    ref_cb_inner();
                });
                return true;
            }
        }
        false
    });

    drop_target
}
