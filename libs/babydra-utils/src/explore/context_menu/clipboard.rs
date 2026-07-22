use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use gtk4::prelude::*;
use gtk4::gdk::FileList;
use crate::explore::context_menu::CLIPBOARD;

pub struct UndoOperation {
    pub is_cut: bool,
    pub sources: Vec<PathBuf>,
    pub destinations: Vec<PathBuf>,
}

thread_local! {
    pub static UNDO_STACK: RefCell<Vec<UndoOperation>> = RefCell::new(Vec::new());
}

/// Sets the files on the system clipboard using FileList and x-special/gnome-copied-files.
pub fn set_system_clipboard_files(paths: &[PathBuf], is_cut: bool) {
    let display = gtk4::gdk::Display::default().unwrap();
    let clipboard = display.clipboard();

    let gio_files: Vec<gtk4::gio::File> = paths.iter()
        .map(|p| gtk4::gio::File::for_path(p))
        .collect();
    let file_list = FileList::from_array(&gio_files);
    let file_provider = gtk4::gdk::ContentProvider::for_value(&file_list.to_value());

    // Build the x-special/gnome-copied-files content
    let action_str = if is_cut { "cut" } else { "copy" };
    let mut gnome_content = action_str.to_string();
    for p in paths {
        let uri = gtk4::gio::File::for_path(p).uri().to_string();
        gnome_content.push_str("\n");
        gnome_content.push_str(&uri);
    }
    let bytes = glib::Bytes::from(gnome_content.as_bytes());
    let gnome_provider = gtk4::gdk::ContentProvider::for_bytes("x-special/gnome-copied-files", &bytes);

    let union_provider = gtk4::gdk::ContentProvider::new_union(&[
        file_provider,
        gnome_provider,
    ]);

    let _ = clipboard.set_content(Some(&union_provider));
}

/// Dynamically updates the `.cut-item` CSS class on current GTK view items without re-scanning the directory.
pub fn apply_cut_dimming(root_widget: &impl IsA<gtk4::Widget>, cut_paths: &[PathBuf]) {
    apply_cut_class_recursive(root_widget.upcast_ref(), cut_paths);
}

/// Applies cut dimming across all active top-level windows in the application.
pub fn apply_cut_dimming_global(cut_paths: &[PathBuf]) {
    let toplevels = gtk4::Window::toplevels();
    for i in 0..toplevels.n_items() {
        if let Some(item) = toplevels.item(i) {
            if let Ok(widget) = item.downcast::<gtk4::Widget>() {
                apply_cut_class_recursive(&widget, cut_paths);
            }
        }
    }
}

fn apply_cut_class_recursive(widget: &gtk4::Widget, cut_paths: &[PathBuf]) {
    if let Ok(flowbox) = widget.clone().downcast::<gtk4::FlowBox>() {
        let mut child_opt = flowbox.first_child();
        while let Some(child) = child_opt {
            let widget_name = child.widget_name();
            let path = PathBuf::from(widget_name.as_str());
            let is_cut = cut_paths.contains(&path);
            if let Ok(flow_child) = child.clone().downcast::<gtk4::FlowBoxChild>() {
                if let Some(inner) = flow_child.child() {
                    if is_cut {
                        inner.add_css_class("cut-item");
                    } else {
                        inner.remove_css_class("cut-item");
                    }
                }
            }
            child_opt = child.next_sibling();
        }
        return;
    }

    if let Ok(listbox) = widget.clone().downcast::<gtk4::ListBox>() {
        let mut child_opt = listbox.first_child();
        while let Some(child) = child_opt {
            let widget_name = child.widget_name();
            let path = PathBuf::from(widget_name.as_str());
            let is_cut = cut_paths.contains(&path);
            if let Ok(list_row) = child.clone().downcast::<gtk4::ListBoxRow>() {
                if let Some(inner) = list_row.child() {
                    if is_cut {
                        inner.add_css_class("cut-item");
                    } else {
                        inner.remove_css_class("cut-item");
                    }
                }
            }
            child_opt = child.next_sibling();
        }
        return;
    }

    let mut child_opt = widget.first_child();
    while let Some(child) = child_opt {
        apply_cut_class_recursive(&child, cut_paths);
        child_opt = child.next_sibling();
    }
}

/// Executes paste (copy or cut) operation asynchronously and triggers navigation refresh.
pub fn execute_paste(
    sources: Vec<PathBuf>,
    dest_dir: PathBuf,
    is_cut: bool,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
) {
    glib::spawn_future_local(async move {
        let mut all_success = true;
        let mut destinations = Vec::new();
        let mut actual_sources = Vec::new();
        for src in sources {
            if let Some(filename) = src.file_name() {
                let dest = dest_dir.join(filename);
                if is_cut {
                    if let Err(e) = babydra_common::move_path(src.clone(), dest.clone()).await {
                        eprintln!("Failed to move file: {}", e);
                        all_success = false;
                    } else {
                        destinations.push(dest);
                        actual_sources.push(src);
                    }
                } else {
                    if let Err(e) = babydra_common::copy_path(src.clone(), dest.clone()).await {
                        eprintln!("Failed to copy file: {}", e);
                        all_success = false;
                    } else {
                        destinations.push(dest);
                        actual_sources.push(src);
                    }
                }
            }
        }

        if all_success && (!destinations.is_empty()) {
            UNDO_STACK.with(|stack| {
                stack.borrow_mut().push(UndoOperation {
                    is_cut,
                    sources: actual_sources,
                    destinations,
                });
            });
        }

        if is_cut && all_success {
            CLIPBOARD.with(|cb| cb.replace(None));
            // Clear system clipboard
            let display = gtk4::gdk::Display::default().unwrap();
            let _ = display.clipboard().set_content(None::<&gtk4::gdk::ContentProvider>);
            apply_cut_dimming_global(&[]);
        }
        nav_callback(current_path);
    });
}

/// Reads files from the system clipboard (supporting both text/uri-list and x-special/gnome-copied-files)
/// and performs the paste operation.
pub fn execute_paste_from_system_clipboard(
    dest_dir: PathBuf,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
) {
    let display = gtk4::gdk::Display::default().unwrap();
    let clipboard = display.clipboard();

    let nav_cb = nav_callback.clone();
    let dest_dir_c = dest_dir.clone();
    let cur_path_c = current_path.clone();

    // Check GdkClipboard formats to read files
    clipboard.read_async(
        &["x-special/gnome-copied-files", "text/uri-list"],
        glib::Priority::DEFAULT,
        None::<&gtk4::gio::Cancellable>,
        move |result| {
            if let Ok((stream, mime_type)) = result {
                let mime_type = mime_type.to_string();
                let nav_cb_inner = nav_cb.clone();
                let dest_dir_inner = dest_dir_c.clone();
                let cur_path_inner = cur_path_c.clone();

                stream.read_bytes_async(
                    65536,
                    glib::Priority::DEFAULT,
                    None::<&gtk4::gio::Cancellable>,
                    move |res_bytes| {
                        if let Ok(bytes) = res_bytes {
                            let content = String::from_utf8_lossy(&bytes).to_string();
                            let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
                            if lines.is_empty() { return; }

                            let (is_cut, uris) = if mime_type == "x-special/gnome-copied-files" {
                                let is_cut = lines[0].trim() == "cut";
                                let uris = lines[1..].iter().map(|s| s.to_string()).collect::<Vec<_>>();
                                (is_cut, uris)
                            } else {
                                // text/uri-list
                                (false, lines.iter().map(|s| s.to_string()).collect::<Vec<_>>())
                            };

                            let paths: Vec<PathBuf> = uris.iter()
                                .filter_map(|uri| {
                                    gtk4::gio::File::for_uri(uri).path()
                                })
                                .collect();

                            if !paths.is_empty() {
                                execute_paste(paths, dest_dir_inner, is_cut, cur_path_inner, nav_cb_inner);
                            }
                        }
                    },
                );
            } else {
                // Fallback to local CLIPBOARD
                let local_data = CLIPBOARD.with(|cb| cb.borrow().clone());
                if let Some((sources, is_cut)) = local_data {
                    execute_paste(sources, dest_dir_c, is_cut, cur_path_c, nav_cb);
                }
            }
        },
    );
}

/// Undo the last copy or cut/paste operation
pub fn execute_undo(nav_callback: Rc<dyn Fn(PathBuf)>, current_path: PathBuf) {
    let op = UNDO_STACK.with(|stack| stack.borrow_mut().pop());
    if let Some(op) = op {
        glib::spawn_future_local(async move {
            if op.is_cut {
                // Move destinations back to sources
                for (src, dest) in op.sources.into_iter().zip(op.destinations.into_iter()) {
                    if let Err(e) = babydra_common::move_path(dest, src).await {
                        eprintln!("Undo failed to move path back: {}", e);
                    }
                }
            } else {
                // Delete destinations
                for dest in op.destinations {
                    if let Err(e) = babydra_common::delete_path(dest).await {
                        eprintln!("Undo failed to delete copied path: {}", e);
                    }
                }
            }
            nav_callback(current_path);
        });
    }
}
