use crate::explore::context_menu::CLIPBOARD;
use gtk4::gdk::FileList;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

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

    let gio_files: Vec<gtk4::gio::File> =
        paths.iter().map(|p| gtk4::gio::File::for_path(p)).collect();
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
    let gnome_provider =
        gtk4::gdk::ContentProvider::for_bytes("x-special/gnome-copied-files", &bytes);

    let union_provider = gtk4::gdk::ContentProvider::new_union(&[file_provider, gnome_provider]);

    let _ = clipboard.set_content(Some(&union_provider));
}

pub use super::dimming::{apply_cut_dimming, apply_cut_dimming_global};

/// Executes paste (copy or cut) operation asynchronously and triggers navigation refresh.
pub fn execute_paste(
    sources: Vec<PathBuf>,
    dest_dir: PathBuf,
    is_cut: bool,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
) {
    let conflicts: Vec<String> = sources
        .iter()
        .filter_map(|src| {
            if let Some(filename) = src.file_name() {
                let dest = dest_dir.join(filename);
                if dest.exists() {
                    Some(filename.to_string_lossy().to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let sources_c = sources.clone();
    let dest_dir_c = dest_dir.clone();
    let current_path_c = current_path.clone();
    let nav_callback_c = nav_callback.clone();

    let do_paste = move || {
        perform_execute_paste(
            sources_c,
            dest_dir_c,
            is_cut,
            current_path_c,
            nav_callback_c,
        );
    };

    if !conflicts.is_empty() {
        let conflict_name = if conflicts.len() == 1 {
            conflicts[0].clone()
        } else {
            t("explore.conflict_items").replace("{}", &conflicts.len().to_string())
        };
        crate::explore::dialogs::show_conflict_dialog(
            &conflict_name,
            do_paste,
            None::<&gtk4::Window>,
        );
    } else {
        do_paste();
    }
}

use babydra_common::i18n::t;
use gtk4::{Align, Box, Label, Orientation, ProgressBar, Window};

fn perform_execute_paste(
    sources: Vec<PathBuf>,
    dest_dir: PathBuf,
    is_cut: bool,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
) {
    if sources.is_empty() {
        return;
    }

    let title_str = if is_cut {
        t("explore.moving_title")
    } else {
        t("explore.copying_title")
    };

    let dialog = Window::builder()
        .title(&title_str)
        .modal(false)
        .resizable(false)
        .default_width(400)
        .default_height(120)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    dialog.set_child(Some(&vbox));

    let lbl_status = Label::builder()
        .label(&title_str)
        .halign(Align::Start)
        .css_classes(vec!["settings-row-title".to_string()])
        .build();
    vbox.append(&lbl_status);

    let lbl_detail = Label::builder()
        .label("")
        .halign(Align::Start)
        .css_classes(vec!["settings-row-desc".to_string()])
        .build();
    vbox.append(&lbl_detail);

    let progress_bar = ProgressBar::builder()
        .hexpand(true)
        .fraction(0.0)
        .css_classes(vec!["content-loading-progress".to_string()])
        .build();
    vbox.append(&progress_bar);

    dialog.present();

    let dialog_c = dialog.clone();
    glib::spawn_future_local(async move {
        let total = sources.len();
        let mut all_success = true;
        let mut destinations = Vec::new();
        let mut actual_sources = Vec::new();

        for (idx, src) in sources.into_iter().enumerate() {
            if let Some(filename) = src.file_name() {
                let name_str = filename.to_string_lossy();
                lbl_detail.set_text(&format!("{} ({}/{})", name_str, idx + 1, total));
                let fraction = (idx + 1) as f64 / total as f64;
                progress_bar.set_fraction(fraction);

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
            glib::timeout_future(std::time::Duration::from_millis(5)).await;
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
            let display = gtk4::gdk::Display::default().unwrap();
            let _ = display
                .clipboard()
                .set_content(None::<&gtk4::gdk::ContentProvider>);
            apply_cut_dimming_global(&[]);
        }

        dialog_c.close();
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
                            let lines: Vec<&str> =
                                content.lines().filter(|l| !l.trim().is_empty()).collect();
                            if lines.is_empty() {
                                return;
                            }

                            let (is_cut, uris) = if mime_type == "x-special/gnome-copied-files" {
                                let is_cut = lines[0].trim() == "cut";
                                let uris =
                                    lines[1..].iter().map(|s| s.to_string()).collect::<Vec<_>>();
                                (is_cut, uris)
                            } else {
                                // text/uri-list
                                (
                                    false,
                                    lines.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                                )
                            };

                            let paths: Vec<PathBuf> = uris
                                .iter()
                                .filter_map(|uri| gtk4::gio::File::for_uri(uri).path())
                                .collect();

                            if !paths.is_empty() {
                                execute_paste(
                                    paths,
                                    dest_dir_inner,
                                    is_cut,
                                    cur_path_inner,
                                    nav_cb_inner,
                                );
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
