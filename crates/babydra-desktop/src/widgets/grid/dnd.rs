//! Drag and Drop subsystem for Desktop: DragSource, Desktop DropTarget, and Folder DropTarget.

use crate::state::{snap_to_grid, DesktopState};
use babydra_core::load_cropped_square;
use gtk4::gdk::FileList;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// Creates a DragSource on an icon widget for dragging files to other grid slots, folders, or external apps.
pub fn create_icon_drag(
    path: &PathBuf,
    icon_name: &str,
    state: Rc<RefCell<DesktopState>>,
    is_dragging: Rc<std::cell::Cell<bool>>,
) -> gtk4::DragSource {
    let drag_source = gtk4::DragSource::new();
    drag_source.set_actions(gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY);

    let path_clone = path.clone();
    let state_clone = state.clone();

    let is_drag_begin = is_dragging.clone();
    drag_source.connect_drag_begin(move |_, _| {
        is_drag_begin.set(true);
    });

    let is_drag_end = is_dragging.clone();
    drag_source.connect_drag_end(move |_, _, _| {
        is_drag_end.set(false);
    });

    drag_source.connect_prepare(move |_, _, _| {
        let targets = {
            let s = state_clone.borrow();
            if s.is_selected(&path_clone) {
                let mut list = vec![path_clone.clone()];
                for p in &s.selected_paths {
                    if p != &path_clone {
                        list.push(p.clone());
                    }
                }
                list
            } else {
                vec![path_clone.clone()]
            }
        };

        let gio_files: Vec<gtk4::gio::File> = targets
            .iter()
            .map(|p| gtk4::gio::File::for_path(p))
            .collect();

        let file_list = FileList::from_array(&gio_files);
        let file_provider = gtk4::gdk::ContentProvider::for_value(&file_list.to_value());

        let mut uri_list = String::new();
        for f in &gio_files {
            uri_list.push_str(&f.uri());
            uri_list.push_str("\r\n");
        }
        let uri_bytes = glib::Bytes::from(uri_list.as_bytes());
        let uri_provider = gtk4::gdk::ContentProvider::for_bytes("text/uri-list", &uri_bytes);

        let mut gnome_content = "copy\n".to_string();
        for f in &gio_files {
            gnome_content.push_str(&f.uri());
            gnome_content.push('\n');
        }
        let gnome_bytes = glib::Bytes::from(gnome_content.as_bytes());
        let gnome_provider =
            gtk4::gdk::ContentProvider::for_bytes("x-special/gnome-copied-files", &gnome_bytes);

        let text_plain = targets
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("\n");
        let text_bytes = glib::Bytes::from(text_plain.as_bytes());
        let text_provider =
            gtk4::gdk::ContentProvider::for_bytes("text/plain;charset=utf-8", &text_bytes);

        let union_provider = gtk4::gdk::ContentProvider::new_union(&[
            file_provider,
            uri_provider,
            gnome_provider,
            text_provider,
        ]);

        Some(union_provider)
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
        if let Ok(pixbuf) = load_cropped_square(path, 64) {
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
pub fn create_desktop_drop(
    state: Rc<RefCell<DesktopState>>,
    refresh_pos_cb: Rc<dyn Fn()>,
) -> gtk4::DropTarget {
    let drop_target = gtk4::DropTarget::new(
        FileList::static_type(),
        gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY,
    );

    let state_drop = state.clone();
    let ref_pos_cb = refresh_pos_cb.clone();

    drop_target.connect_drop(move |_, value, x, y| {
        let desktop_dir = DesktopState::desktop_dir();
        if let Ok(file_list) = value.get::<FileList>() {
            let sources: Vec<PathBuf> = file_list.files().iter().filter_map(|f| f.path()).collect();

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
                let cell_w = state_drop.borrow().config.grid_spacing.max(80) as i32;
                let cell_h = cell_w + 14;

                let state_ref = state_drop.borrow();
                let current_positions = state_ref.compute_all_positions();

                let anchor_src = &internal_sources[0];
                let anchor_name = anchor_src
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                let anchor_current_pos = current_positions
                    .get(anchor_name)
                    .copied()
                    .unwrap_or((0, 0));
                drop(state_ref);

                let (base_x, base_y) = snap_to_grid(
                    x as i32,
                    y as i32,
                    cell_w,
                    cell_h,
                    crate::state::DEFAULT_MARGIN_X,
                    crate::state::DEFAULT_MARGIN_Y,
                );

                let offset_x = base_x - anchor_current_pos.0;
                let offset_y = base_y - anchor_current_pos.1;

                for src in internal_sources {
                    if let Some(file_name) = src.file_name().and_then(|n| n.to_str()) {
                        let cur_pos = current_positions
                            .get(file_name)
                            .copied()
                            .unwrap_or(anchor_current_pos);
                        let new_x = cur_pos.0 + offset_x;
                        let new_y = cur_pos.1 + offset_y;

                        state_drop.borrow_mut().set_icon_position(
                            file_name.to_string(),
                            new_x,
                            new_y,
                        );
                    }
                }
                ref_pos_cb();
            }

            // 2. External Files Ingestion (Copy files dropped from external apps to ~/Desktop)
            if !external_sources.is_empty() {
                let state_inner = state_drop.clone();
                let drop_x = x as i32;
                let drop_y = y as i32;
                let ref_pos_cb_inner = ref_pos_cb.clone();

                glib::spawn_future_local(async move {
                    let cell_w = state_inner.borrow().config.grid_spacing.max(80) as i32;
                    let cell_h = cell_w + 14;
                    let (base_x, mut base_y) = snap_to_grid(
                        drop_x,
                        drop_y,
                        cell_w,
                        cell_h,
                        crate::state::DEFAULT_MARGIN_X,
                        crate::state::DEFAULT_MARGIN_Y,
                    );

                    for src in external_sources {
                        if let Some(filename) = src.file_name().map(|n| n.to_os_string()) {
                            let dest = desktop_dir.join(&filename);
                            if src != dest {
                                if babydra_core::copy_path(src, dest.clone()).await.is_ok() {
                                    #[cfg(unix)]
                                    {
                                        if dest.extension().is_some_and(|e| e == "desktop") {
                                            use std::os::unix::fs::PermissionsExt;
                                            if let Ok(metadata) = std::fs::metadata(&dest) {
                                                let mut perms = metadata.permissions();
                                                perms.set_mode(perms.mode() | 0o755);
                                                let _ = std::fs::set_permissions(&dest, perms);
                                            }
                                        }
                                    }
                                    if let Some(name_str) = filename.to_str() {
                                        state_inner.borrow_mut().set_icon_position(
                                            name_str.to_string(),
                                            base_x,
                                            base_y,
                                        );
                                        base_y += cell_h;
                                    }
                                }
                            }
                        }
                    }
                    ref_pos_cb_inner();
                });
            }

            return true;
        }
        false
    });

    drop_target
}

/// Creates a DropTarget on a directory icon on desktop to allow dropping files inside that directory.
pub fn create_folder_drop(
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
            let sources: Vec<PathBuf> = file_list.files().iter().filter_map(|f| f.path()).collect();

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
