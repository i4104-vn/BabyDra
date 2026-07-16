use gtk4::prelude::*;
use gtk4::ApplicationWindow;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::path::PathBuf;
use babydra_common::{SessionState, ActivePane};

mod render;
pub mod tabs;
pub mod split;

/// Creates and configures the main file explorer window, wires all component widgets (header, sidebar, content panes, tabs, info panel, status bar), and launches the navigation loops.
pub fn create_explore_window(
    app: &gtk4::Application,
    session: Rc<RefCell<SessionState>>,
) -> ApplicationWindow {
    let settings = babydra_common::load_explore_settings();
    
    // Apply settings to initial tab
    {
        let mut s = session.borrow_mut();
        let tab = s.active_tab_mut();
        tab.view_mode = settings.view_mode.clone();
        tab.show_hidden = settings.show_hidden;
    }

    let ui = render::build_window_ui(app);

    // Active state variables
    let is_split = Rc::new(Cell::new(false));
    let active_pane = Rc::new(Cell::new(ActivePane::Left));
    let preview_visible = Rc::new(Cell::new(settings.preview_visible));
    let user_wants_preview = Rc::new(Cell::new(settings.preview_visible));
    let watcher = Rc::new(RefCell::new(None::<babydra_common::FileWatcher>));

    // Channels for file watching/reloading
    let (watch_tx, mut watch_rx) = tokio::sync::mpsc::unbounded_channel::<()>();
    let _watch_tx_clone = watch_tx.clone();

    // Create InfoPanel
    let (info_panel_container, info_widgets) = crate::widgets::info_panel::create_info_panel();
    let revealer = gtk4::Revealer::builder()
        .transition_type(gtk4::RevealerTransitionType::SlideLeft)
        .transition_duration(250)
        .build();
    revealer.set_child(Some(&info_panel_container));
    revealer.set_reveal_child(preview_visible.get());
    if preview_visible.get() {
        ui.layout_paned.set_end_child(Some(&revealer));
    } else {
        ui.layout_paned.set_end_child(None::<&gtk4::Widget>);
    }

    let info_widgets_rc = Rc::new(info_widgets);

    // Navigation callbacks & cells
    let navigate_pane_ref: Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>> = Rc::new(RefCell::new(None));
    let navigate_pane_no_watch_ref: Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>> = Rc::new(RefCell::new(None));

    // Left pane navigation channels
    let (left_nav_tx, mut left_rx) = tokio::sync::mpsc::unbounded_channel::<PathBuf>();
    let left_nav_cb = move |path: PathBuf| {
        let _ = left_nav_tx.send(path);
    };

    // Scrolled window cells to resolve ordering in closure capture
    let left_scroll_cell = Rc::new(RefCell::new(None::<gtk4::ScrolledWindow>));
    let right_scroll_cell = Rc::new(RefCell::new(None::<gtk4::ScrolledWindow>));

    // Create Left ContentView
    let (left_content_scroll, left_content_handle) = crate::widgets::content_view::create_content_view(
        left_nav_cb,
        {
            let info = info_widgets_rc.clone();
            let active = active_pane.clone();
            let left_s = left_scroll_cell.clone();
            let right_s = right_scroll_cell.clone();
            move |sel| {
                active.set(ActivePane::Left);
                if let Some(ref ls) = *left_s.borrow() {
                    ls.add_css_class("active-pane");
                }
                if let Some(ref rs) = *right_s.borrow() {
                    rs.remove_css_class("active-pane");
                }
                crate::widgets::info_panel::update_info_panel(&info, &sel);
            }
        },
    );

    *left_scroll_cell.borrow_mut() = Some(left_content_scroll.clone());
    let left_content_handle = Rc::new(left_content_handle);
    ui.split_paned.set_start_child(Some(&left_content_scroll));
    left_content_scroll.add_css_class("active-pane");

    // Right pane content variables
    let right_content_handle = Rc::new(RefCell::new(None::<Rc<crate::widgets::content_view::ContentViewHandle>>));

    // Create HeaderBar
    let header_widgets_cell = Rc::new(RefCell::new(None::<crate::widgets::header_bar::HeaderBarWidgets>));
    let status_bar_widgets_cell = Rc::new(RefCell::new(None::<crate::widgets::status_bar::StatusBarWidgets>));

    // Toggle hidden files closure
    let toggle_hidden = {
        let session_c = session.clone();
        let nav = navigate_pane_ref.clone();
        let active = active_pane.clone();
        let status_widgets_c = status_bar_widgets_cell.clone();
        move || {
            let show_hidden_now = {
                let mut s = session_c.borrow_mut();
                let tab = s.active_tab_mut();
                tab.show_hidden = !tab.show_hidden;
                tab.show_hidden
            };

            // Save updated settings
            {
                let mut current_settings = babydra_common::load_explore_settings();
                current_settings.show_hidden = show_hidden_now;
                babydra_common::save_explore_settings(&current_settings);
            }

            if let Some(ref sw) = *status_widgets_c.borrow() {
                if show_hidden_now {
                    sw.btn_toggle_hidden.add_css_class("status-bar-btn-active");
                } else {
                    sw.btn_toggle_hidden.remove_css_class("status-bar-btn-active");
                }
            }

            let path = session_c.borrow().active_tab().current_path.clone();
            if let Some(ref f) = *nav.borrow() {
                f(active.get(), path);
            }
        }
    };
    let toggle_hidden_rc = Rc::new(toggle_hidden) as Rc<dyn Fn()>;

    // Create TabBar Container
    let tab_bar_box = Rc::new(RefCell::new(None::<gtk4::Box>));

    // Create StatusBar
    let status_bar_widgets = crate::widgets::status_bar::create_status_bar();
    ui.vbox.append(&status_bar_widgets.container);
    let status_bar_lbl_rc = Rc::new(status_bar_widgets.lbl_status.clone());
    status_bar_widgets_cell.replace(Some(status_bar_widgets.clone()));

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
        let status_bar_lbl = status_bar_lbl_rc.clone();
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
                handle.widgets.stack.set_visible_child_name("loading");
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
                            let mode = handle.current_mode.borrow().clone();
                            handle.widgets.stack.set_visible_child_name(&mode);
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

    // Setup global window navigation callback
    let nav_ref_for_header = navigate_pane_ref.clone();
    let active_pane_for_header = active_pane.clone();
    let nav_callback = move |path: PathBuf| {
        if let Some(ref f) = *nav_ref_for_header.borrow() {
            f(active_pane_for_header.get(), path);
        }
    };

    let nav_callback_rc = Rc::new(nav_callback) as Rc<dyn Fn(PathBuf)>;

    // HeaderBar event callbacks
    let view_mode_callback = {
        let left = left_content_handle.clone();
        let right = right_content_handle.clone();
        move |mode: String| {
            crate::widgets::content_view::set_content_view_mode(&left, &mode);
            if let Some(ref r) = *right.borrow() {
                crate::widgets::content_view::set_content_view_mode(r, &mode);
            }

            // Save updated settings
            {
                let mut current_settings = babydra_common::load_explore_settings();
                current_settings.view_mode = mode;
                babydra_common::save_explore_settings(&current_settings);
            }
        }
    };

    let search_callback = {
        let left = left_content_handle.clone();
        let right = right_content_handle.clone();
        let active = active_pane.clone();
        move |query: String| {
            if active.get() == ActivePane::Left {
                crate::widgets::content_view::filter_content_view(&left, &query);
            } else if let Some(ref r) = *right.borrow() {
                crate::widgets::content_view::filter_content_view(r, &query);
            }
        }
    };

    let sort_callback = {
        let left = left_content_handle.clone();
        let right = right_content_handle.clone();
        let active = active_pane.clone();
        move |sort_mode: String| {
            if active.get() == ActivePane::Left {
                crate::widgets::content_view::set_content_view_sort(&left, &sort_mode);
            } else if let Some(ref r) = *right.borrow() {
                crate::widgets::content_view::set_content_view_sort(r, &sort_mode);
            }
        }
    };

    // Create Header Bar Box
    let (header_box, header_widgets) = crate::widgets::header_bar::create_header_bar(
        session.clone(),
        {
            let nav = nav_callback_rc.clone();
            move |p| nav(p)
        },
        view_mode_callback,
        search_callback,
        sort_callback,
    );
    ui.vbox.insert_child_after(&header_box, None::<&gtk4::Widget>);
    header_widgets_cell.replace(Some(header_widgets.clone()));

    // Wire btn_new_folder click (dynamically handles New Folder or Empty Trash depending on path)
    {
        let session_c = session.clone();
        let nav = navigate_pane_ref.clone();
        let active = active_pane.clone();
        header_widgets.btn_new_folder.connect_clicked(move |_| {
            let path = session_c.borrow().active_tab().current_path.clone();
            let is_in_trash = path.to_string_lossy().ends_with("Trash/files");
            if is_in_trash {
                babydra_common::helper::clean::remove_trash();
                if let Some(ref f) = *nav.borrow() {
                    f(active.get(), path);
                }
            } else {
                let nav_cb = {
                    let nav = nav.clone();
                    let act = active.clone();
                    Rc::new(move |p| {
                        if let Some(ref f) = *nav.borrow() {
                            f(act.get(), p);
                        }
                    })
                };
                babydra_utils::explore::dialogs::show_new_folder_dialog(path, nav_cb);
            }
        });
    }

    // Preview toggle closure
    let toggle_preview = {
        let layout_paned = ui.layout_paned.clone();
        let revealer_c = revealer.clone();
        let preview_visible = preview_visible.clone();
        let user_wants_preview = user_wants_preview.clone();
        let status_widgets_c = status_bar_widgets_cell.clone();
        move || {
            let now_visible = !preview_visible.get();
            preview_visible.set(now_visible);
            user_wants_preview.set(now_visible);

            if now_visible {
                layout_paned.set_end_child(Some(&revealer_c));
                revealer_c.set_reveal_child(true);
            } else {
                revealer_c.set_reveal_child(false);
                let layout_paned_c = layout_paned.clone();
                let revealer_cc = revealer_c.clone();
                glib::timeout_add_local_once(std::time::Duration::from_millis(250), move || {
                    if !revealer_cc.reveals_child() {
                        layout_paned_c.set_end_child(None::<&gtk4::Widget>);
                    }
                });
            }

            // Save updated settings
            {
                let mut current_settings = babydra_common::load_explore_settings();
                current_settings.preview_visible = now_visible;
                babydra_common::save_explore_settings(&current_settings);
            }

            if let Some(ref sw) = *status_widgets_c.borrow() {
                if now_visible {
                    sw.btn_toggle_preview.add_css_class("status-bar-btn-active");
                } else {
                    sw.btn_toggle_preview.remove_css_class("status-bar-btn-active");
                }
            }
        }
    };
    let toggle_preview_rc = Rc::new(toggle_preview);

    // Wire status bar buttons click
    {
        let toggle_p = toggle_preview_rc.clone();
        let toggle_h = toggle_hidden_rc.clone();
        if let Some(ref sw) = *status_bar_widgets_cell.borrow() {
            sw.btn_toggle_preview.connect_clicked(move |_| {
                toggle_p();
            });
            sw.btn_toggle_hidden.connect_clicked(move |_| {
                toggle_h();
            });
        }
    }

    let _rebuild_tabs_rc = tabs::setup_tab_bar(&ui.vbox, session.clone(), navigate_pane_ref.clone(), tab_bar_box.clone());

    // Sidebar creation
    let sidebar = crate::widgets::sidebar::create_sidebar(
        session.clone(),
        {
            let nav = nav_callback_rc.clone();
            move |p| nav(p)
        }
    );
    ui.main_paned.prepend(&sidebar);

    // Active split toggling handler
    let toggle_split_view_rc = split::setup_split_view(
        ui.split_paned.clone(),
        is_split.clone(),
        right_scroll_cell.clone(),
        right_content_handle.clone(),
        session.clone(),
        active_pane.clone(),
        navigate_pane_ref.clone(),
        info_widgets_rc.clone(),
        left_content_scroll.clone(),
    );

    // Wire F3 key controller (split view) and F4 (preview toggle)
    {
        let toggle_split = toggle_split_view_rc.clone();
        let toggle_preview = toggle_preview_rc.clone();
        let toggle_hidden = toggle_hidden_rc.clone();
        let key_controller = gtk4::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, keyval, _, state| {
            let has_ctrl = state.contains(gtk4::gdk::ModifierType::CONTROL_MASK);
            if keyval == gtk4::gdk::Key::F3 {
                toggle_split();
                glib::Propagation::Stop
            } else if keyval == gtk4::gdk::Key::F4 {
                toggle_preview();
                glib::Propagation::Stop
            } else if has_ctrl && (keyval == gtk4::gdk::Key::h || keyval == gtk4::gdk::Key::H) {
                toggle_hidden();
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        ui.window.add_controller(key_controller);
    }

    // Auto-hide preview when window is too narrow (< 700px)
    {
        let layout_paned = ui.layout_paned.clone();
        let revealer_c = revealer.clone();
        let preview_visible = preview_visible.clone();
        let user_wants_preview = user_wants_preview.clone();
        let status_widgets_c = status_bar_widgets_cell.clone();
        ui.window.connect_default_width_notify(move |window| {
            let w = window.width();
            if w < 700 && preview_visible.get() {
                revealer_c.set_reveal_child(false);
                preview_visible.set(false);
                let layout_paned_c = layout_paned.clone();
                let revealer_cc = revealer_c.clone();
                glib::timeout_add_local_once(std::time::Duration::from_millis(250), move || {
                    if !revealer_cc.reveals_child() {
                        layout_paned_c.set_end_child(None::<&gtk4::Widget>);
                    }
                });
                if let Some(ref sw) = *status_widgets_c.borrow() {
                    sw.btn_toggle_preview.remove_css_class("status-bar-btn-active");
                }
            } else if w >= 700 && !preview_visible.get() && user_wants_preview.get() {
                layout_paned.set_end_child(Some(&revealer_c));
                revealer_c.set_reveal_child(true);
                preview_visible.set(true);
                if let Some(ref sw) = *status_widgets_c.borrow() {
                    sw.btn_toggle_preview.add_css_class("status-bar-btn-active");
                }
            }
        });
    }

    // Connect file watcher hot-reload channel receiver
    {
        let session_c = session.clone();
        let nav_no_watch_c = navigate_pane_no_watch_ref.clone();
        let active_pane_c = active_pane.clone();
        glib::MainContext::default().spawn_local(async move {
            while let Some(_) = watch_rx.recv().await {
                let path = session_c.borrow().active_tab().current_path.clone();
                if let Some(ref f) = *nav_no_watch_c.borrow() {
                    f(active_pane_c.get(), path);
                }
            }
        });
    }

    // Connect D-Bus receiver loop
    {
        let (dbus_tx, mut dbus_rx) = tokio::sync::mpsc::unbounded_channel::<std::path::PathBuf>();
        let nav_no_watch_c = navigate_pane_no_watch_ref.clone();
        let active_pane_c = active_pane.clone();
        glib::MainContext::default().spawn_local(async move {
            while let Some(path) = dbus_rx.recv().await {
                if let Some(ref f) = *nav_no_watch_c.borrow() {
                    f(active_pane_c.get(), path);
                }
            }
        });

        tokio::spawn(async move {
            if let Err(e) = babydra_common::start_dbus_service(dbus_tx).await {
                eprintln!("Failed to start D-Bus service: {}", e);
            }
        });
    }

    // Start initial navigation
    let path = session.borrow().active_tab().current_path.clone();
    if let Some(ref f) = *navigate_pane_ref.borrow() {
        f(ActivePane::Left, path);
    }

    ui.window
}
