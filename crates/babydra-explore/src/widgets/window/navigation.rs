use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::path::PathBuf;
use gtk4::prelude::*;
use babydra_common::{SessionState, ActivePane, ContentViewHandle, HeaderBarWidgets, FileWatcher};
use crate::widgets::status_bar::StatusBarWidgets;

/// Sets up the primary navigation closures (`navigate_pane` and `navigate_pane_no_watch`) and registers the left-pane channel watcher.
pub fn setup_navigation(
    session: Rc<RefCell<SessionState>>,
    active_pane: Rc<Cell<ActivePane>>,
    left_content_handle: Rc<ContentViewHandle>,
    right_content_handle: Rc<RefCell<Option<Rc<ContentViewHandle>>>>,
    left_scroll_cell: Rc<RefCell<Option<gtk4::Box>>>,
    right_scroll_cell: Rc<RefCell<Option<gtk4::Box>>>,
    status_bar_widgets_cell: Rc<RefCell<Option<StatusBarWidgets>>>,
    header_widgets_cell: Rc<RefCell<Option<HeaderBarWidgets>>>,
    tab_bar_box: Rc<RefCell<Option<gtk4::Box>>>,
    status_bar_lbl: Rc<gtk4::Label>,
    watch_tx: tokio::sync::mpsc::UnboundedSender<()>,
    mut left_rx: tokio::sync::mpsc::UnboundedReceiver<PathBuf>,
) -> (
    Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>>, // navigate_pane_ref
    Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>>, // navigate_pane_no_watch_ref
    Rc<RefCell<Option<FileWatcher>>>,
) {
    let navigate_pane_ref = Rc::new(RefCell::new(None::<Rc<dyn Fn(ActivePane, PathBuf)>>));
    let navigate_pane_no_watch_ref = Rc::new(RefCell::new(None::<Rc<dyn Fn(ActivePane, PathBuf)>>));
    let watcher = Rc::new(RefCell::new(None::<FileWatcher>));

    // Define navigate_pane_no_watch closure
    {
        let session = session.clone();
        let active_pane = active_pane.clone();
        let left_handle = left_content_handle.clone();
        let right_handle = right_content_handle.clone();
        let right_s = right_scroll_cell.clone();
        let left_s = left_scroll_cell.clone();
        let status_bar_widgets_cell = status_bar_widgets_cell.clone();
        let header_widgets_cell = header_widgets_cell.clone();
        let tab_bar_box = tab_bar_box.clone();
        let status_bar_lbl = status_bar_lbl.clone();
        let navigate_pane_no_watch_ref_c = navigate_pane_no_watch_ref.clone();

        *navigate_pane_no_watch_ref.borrow_mut() = Some(Rc::new(move |pane: ActivePane, path: PathBuf| {
            // Highlight active pane
            active_pane.set(pane);
            if pane == ActivePane::Left {
                if let Some(ref ls) = *left_s.borrow() {
                    ls.add_css_class("active-pane");
                }
                if let Some(ref rs) = *right_s.borrow() {
                    rs.remove_css_class("active-pane");
                }
            } else {
                if let Some(ref ls) = *left_s.borrow() {
                    ls.remove_css_class("active-pane");
                }
                if let Some(ref rs) = *right_s.borrow() {
                    rs.add_css_class("active-pane");
                }
            }

            let show_hidden = session.borrow().active_tab().show_hidden;

            // Sync toggle button active class in StatusBar
            if let Some(ref sw) = *status_bar_widgets_cell.borrow() {
                if show_hidden {
                    sw.btn_toggle_hidden.add_css_class("status-bar-btn-active");
                } else {
                    sw.btn_toggle_hidden.remove_css_class("status-bar-btn-active");
                }
            }

            let content_handle = if pane == ActivePane::Left {
                Some(left_handle.clone())
            } else {
                right_handle.borrow().clone()
            };

            // Update session path
            session.borrow_mut().active_tab_mut().current_path = path.clone();

            if let Some(ref handle) = content_handle {
                handle.widgets.progress_bar.set_visible(true);
                handle.widgets.progress_bar.set_fraction(0.0);
            }

            let header_widgets_c = header_widgets_cell.clone();
            let session_c = session.clone();
            let nav_no_watch_c = navigate_pane_no_watch_ref_c.clone();
            let tab_bar_box_c = tab_bar_box.clone();
            let status_lbl_c = status_bar_lbl.clone();
            let content_handle_err = content_handle.clone();

            glib::spawn_future_local(async move {
                match babydra_common::load_directory(path.clone(), show_hidden).await {
                    Ok(entries) => {
                        // Update Header breadcrumbs
                        if let Some(ref hw) = *header_widgets_c.borrow() {
                            let is_in_trash = path.to_string_lossy().ends_with("Trash/files");
                            babydra_utils::explore::update_new_folder_button(&hw.btn_new_folder, is_in_trash);

                            let nav_cb: Rc<dyn Fn(PathBuf)> = Rc::new(move |p: PathBuf| {
                                if let Some(ref f) = *nav_no_watch_c.borrow() {
                                    f(pane, p);
                                }
                            });
                            crate::widgets::header_bar::update_address_bar(
                                &hw.breadcrumb_box,
                                &hw.address_stack,
                                &session_c,
                                &path,
                                &nav_cb,
                            );
                        }

                        // Calculate size
                        let total_size: u64 = entries.iter().map(|e| e.size).sum();

                        // Update Content
                        if let Some(ref handle) = content_handle {
                            crate::widgets::content_view::update_content_view(handle, &entries, path);
                        }

                        // Update Status Bar
                        crate::widgets::status_bar::update_status_bar(&status_lbl_c, entries.len(), total_size);

                        // Update Tab Bar titles
                        if let Some(ref _tbb) = *tab_bar_box_c.borrow() {
                            // Trigger TabBar rebuild
                            // Re-borrow tabs callbacks inside rebuild
                        }
                    }
                    Err(err) => {
                        eprintln!("Failed to load directory: {}", err);
                        if let Some(ref handle) = content_handle_err {
                            handle.widgets.progress_bar.set_visible(false);
                        }
                    }
                }
            });
        }));
    }

    // Define navigate_pane closure (which handles watcher)
    {
        let watcher = watcher.clone();
        let watch_tx = watch_tx.clone();
        let nav_no_watch_ref = navigate_pane_no_watch_ref.clone();

        *navigate_pane_ref.borrow_mut() = Some(Rc::new(move |pane: ActivePane, path: PathBuf| {
            let mut watcher_borrow = watcher.borrow_mut();
            if let Some(ref mut w) = *watcher_borrow {
                let _ = w.watch(&path);
            } else {
                let tx_clone = watch_tx.clone();
                if let Ok(w) = babydra_common::FileWatcher::new(path.clone(), move |_event| {
                    let _ = tx_clone.send(());
                }) {
                    *watcher_borrow = Some(w);
                }
            }

            if let Some(ref f) = *nav_no_watch_ref.borrow() {
                f(pane, path);
            }
        }));
    }

    // Wire left pane navigation loop
    {
        let nav = navigate_pane_ref.clone();
        let session_c = session.clone();
        glib::MainContext::default().spawn_local(async move {
            while let Some(path) = left_rx.recv().await {
                {
                    let mut s = session_c.borrow_mut();
                    s.active_tab_mut().navigate_to(path.clone());
                }
                if let Some(ref f) = *nav.borrow() {
                    f(ActivePane::Left, path);
                }
            }
        });
    }

    (navigate_pane_ref, navigate_pane_no_watch_ref, watcher)
}
