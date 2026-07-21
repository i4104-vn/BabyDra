use gtk4::prelude::*;
use gtk4::ApplicationWindow;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::path::PathBuf;
use babydra_common::{SessionState, ActivePane};

mod render;
pub mod handlers;
pub mod layout;
pub mod widgets;

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

    // Channels for file watching/reloading
    let (watch_tx, watch_rx) = tokio::sync::mpsc::unbounded_channel::<()>();

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

    // Scrolled window cells to resolve ordering in closure capture
    let left_scroll_cell = Rc::new(RefCell::new(None::<gtk4::Box>));
    let right_scroll_cell = Rc::new(RefCell::new(None::<gtk4::Box>));

    // Left pane navigation channels
    let (left_nav_tx, left_rx) = tokio::sync::mpsc::unbounded_channel::<PathBuf>();
    let left_nav_cb = move |path: PathBuf| {
        let _ = left_nav_tx.send(path);
    };

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

    // Create StatusBar cells
    let status_bar_widgets_cell = Rc::new(RefCell::new(None::<crate::widgets::status_bar::StatusBarWidgets>));
    let rebuild_shortcuts_cell = Rc::new(RefCell::new(None::<Rc<dyn Fn()>>));

    // Create TabBar Container
    let tab_bar_box = Rc::new(RefCell::new(None::<gtk4::Box>));

    // Create StatusBar
    let status_bar_widgets = crate::widgets::status_bar::create_status_bar();
    ui.vbox.append(&status_bar_widgets.container);
    let status_bar_lbl_rc = Rc::new(status_bar_widgets.lbl_status.clone());
    status_bar_widgets_cell.replace(Some(status_bar_widgets.clone()));

    // Setup navigation closures
    let rebuild_tabs_cell = Rc::new(RefCell::new(None::<Rc<dyn Fn()>>));
    let (navigate_pane_ref, navigate_pane_no_watch_ref, _watchers) = handlers::setup_navigation(
        session.clone(),
        active_pane.clone(),
        left_content_handle.clone(),
        right_content_handle.clone(),
        left_scroll_cell.clone(),
        right_scroll_cell.clone(),
        status_bar_widgets_cell.clone(),
        tab_bar_box.clone(),
        status_bar_lbl_rc.clone(),
        rebuild_tabs_cell.clone(),
        watch_tx.clone(),
        left_rx,
    );

    // Toggle hidden files closure
    let toggle_hidden = {
        let session_c = session.clone();
        let nav = navigate_pane_ref.clone();
        let active = active_pane.clone();
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



            let path = session_c.borrow().active_tab().current_path.clone();
            if let Some(ref f) = *nav.borrow() {
                f(active.get(), path);
            }
        }
    };
    let toggle_hidden_rc = Rc::new(toggle_hidden) as Rc<dyn Fn()>;

    // Setup global window navigation callback
    let nav_ref_for_header = navigate_pane_ref.clone();
    let active_pane_for_header = active_pane.clone();
    let left_handle_for_nav = left_content_handle.clone();
    let right_handle_for_nav = right_content_handle.clone();
    let nav_callback = move |path: PathBuf| {
        if let Some(ref f) = *nav_ref_for_header.borrow() {
            let active = active_pane_for_header.get();
            f(active, path);
            
            // If split view is open, refresh the other pane too to keep listings in sync
            if let Some(ref right) = *right_handle_for_nav.borrow() {
                let other_pane = if active == ActivePane::Left {
                    ActivePane::Right
                } else {
                    ActivePane::Left
                };
                let other_path = if other_pane == ActivePane::Left {
                    left_handle_for_nav.current_path.borrow().clone()
                } else {
                    right.current_path.borrow().clone()
                };
                f(other_pane, other_path);
            }
        }
    };
    let nav_callback_rc = Rc::new(nav_callback) as Rc<dyn Fn(PathBuf)>;

    // HeaderBar event callbacks
    let view_mode_callback = {
        let left = left_content_handle.clone();
        let right = right_content_handle.clone();
        let status_bar_widgets_c = status_bar_widgets_cell.clone();
        move |mode: String| {
            crate::widgets::content_view::set_content_view_mode(&left, &mode);
            if let Some(ref r) = *right.borrow() {
                crate::widgets::content_view::set_content_view_mode(r, &mode);
            }

            // Save updated settings
            {
                let mut current_settings = babydra_common::load_explore_settings();
                current_settings.view_mode = mode.clone();
                babydra_common::save_explore_settings(&current_settings);
            }

            // Update button active classes in status bar
            if let Some(ref sw) = *status_bar_widgets_c.borrow() {
                if mode == "list" {
                    sw.btn_view_list.add_css_class("status-bar-btn-active");
                    sw.btn_view_icons.remove_css_class("status-bar-btn-active");
                } else {
                    sw.btn_view_icons.add_css_class("status-bar-btn-active");
                    sw.btn_view_list.remove_css_class("status-bar-btn-active");
                }
            }
        }
    };
    let view_mode_callback_rc = Rc::new(view_mode_callback) as Rc<dyn Fn(String)>;

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
    let sort_callback_rc = Rc::new(sort_callback) as Rc<dyn Fn(String)>;

    // Setup preview panel visibility toggle closure
    let toggle_preview_rc = layout::setup_preview_toggle(
        ui.layout_paned.clone(),
        revealer.clone(),
        preview_visible.clone(),
        user_wants_preview.clone(),
        status_bar_widgets_cell.clone(),
    );

    // Wire status bar buttons click
    {
        let toggle_p = toggle_preview_rc.clone();
        let view_mode_cb = view_mode_callback_rc.clone();
        let sort_cb = sort_callback_rc.clone();
        let parent_win = ui.window.clone().upcast::<gtk4::Window>();
        let rebuild_cell_c = rebuild_shortcuts_cell.clone();
        let session_cc = session.clone();
        let nav_c = navigate_pane_ref.clone();
        let act_c = active_pane.clone();
        let preview_vc = preview_visible.clone();
        let toggle_p_c = toggle_preview_rc.clone();

        if let Some(ref sw) = *status_bar_widgets_cell.borrow() {
            sw.btn_toggle_preview.connect_clicked(move |_| {
                toggle_p();
            });

            // View modes (Grid / List)
            {
                let cb = view_mode_cb.clone();
                sw.btn_view_icons.connect_clicked(move |_| {
                    cb("icons".to_string());
                });
            }
            {
                let cb = view_mode_cb.clone();
                sw.btn_view_list.connect_clicked(move |_| {
                    cb("list".to_string());
                });
            }

            // Sort Dropdown
            {
                let cb = sort_cb.clone();
                sw.dropdown_sort.connect_selected_notify(move |dd| {
                    let selected = dd.selected();
                    let mode = match selected {
                        0 => "auto".to_string(),
                        1 => "date".to_string(),
                        2 => "group".to_string(),
                        _ => "auto".to_string(),
                    };
                    cb(mode);
                });
            }

            // Settings button
            {
                let parent_win_c = parent_win.clone();
                let rebuild_c = rebuild_cell_c.clone();
                let session_inner = session_cc.clone();
                let nav_inner = nav_c.clone();
                let act_inner = act_c.clone();
                let preview_inner = preview_vc.clone();
                let toggle_p_inner = toggle_p_c.clone();

                sw.btn_settings.connect_clicked(move |_| {
                    let rebuild_inner_cb = rebuild_c.clone();
                    let session_inner_cb = session_inner.clone();
                    let nav_inner_cb = nav_inner.clone();
                    let act_inner_cb = act_inner.clone();
                    let preview_inner_cb = preview_inner.clone();
                    let toggle_p_inner_cb = toggle_p_inner.clone();

                    crate::widgets::settings_dialog::show_settings_dialog(&parent_win_c, move || {
                        let settings = babydra_common::load_explore_settings();
                        
                        // 1. Sync hidden files visibility setting in tab state
                        {
                            let mut s = session_inner_cb.borrow_mut();
                            let tab = s.active_tab_mut();
                            tab.show_hidden = settings.show_hidden;
                        }

                        // 2. Sync preview panel visibility
                        let preview_changed = preview_inner_cb.get() != settings.preview_visible;
                        if preview_changed {
                            toggle_p_inner_cb();
                        }

                        // 3. Trigger navigate refresh
                        let path = session_inner_cb.borrow().active_tab().current_path.clone();
                        if let Some(ref f) = *nav_inner_cb.borrow() {
                            f(act_inner_cb.get(), path);
                        }

                        // 4. Rebuild shortcuts
                        if let Some(ref rebuild) = *rebuild_inner_cb.borrow() {
                            rebuild();
                        }
                    });
                });
            }

            // Sync initial view mode class for status bar buttons
            if settings.view_mode == "list" {
                sw.btn_view_list.add_css_class("status-bar-btn-active");
                sw.btn_view_icons.remove_css_class("status-bar-btn-active");
            } else {
                sw.btn_view_icons.add_css_class("status-bar-btn-active");
                sw.btn_view_list.remove_css_class("status-bar-btn-active");
            }
        }
    }


    let _rebuild_tabs_rc = widgets::setup_tab_bar(&ui.vbox, session.clone(), navigate_pane_ref.clone(), tab_bar_box.clone(), rebuild_tabs_cell.clone());

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
    let toggle_split_view_rc = layout::setup_split_view(
        ui.split_paned.clone(),
        is_split.clone(),
        right_scroll_cell.clone(),
        right_content_handle.clone(),
        session.clone(),
        active_pane.clone(),
        navigate_pane_ref.clone(),
        info_widgets_rc.clone(),
        left_content_scroll.clone(),
        left_content_handle.clone(),
    );

    // Define clipboard and undo callbacks
    let cut_cb = {
        let left = left_content_handle.clone();
        let right = right_content_handle.clone();
        let act = active_pane.clone();
        let session = session.clone();
        let nav = navigate_pane_ref.clone();
        move || {
            let paths = if act.get() == ActivePane::Left {
                left.selected_paths.borrow().clone()
            } else {
                right.borrow().as_ref()
                    .map(|r| r.selected_paths.borrow().clone())
                    .unwrap_or_default()
            };
            if !paths.is_empty() {
                let current_path = session.borrow().active_tab().current_path.clone();
                babydra_utils::explore::context_menu::clipboard::set_system_clipboard_files(&paths, true);
                babydra_utils::explore::CLIPBOARD.with(|cb| cb.replace(Some((paths, true))));
                if let Some(ref f) = *nav.borrow() {
                    f(act.get(), current_path);
                }
            }
        }
    };
    let cut_cb_rc = Rc::new(cut_cb) as Rc<dyn Fn()>;

    let copy_cb = {
        let left = left_content_handle.clone();
        let right = right_content_handle.clone();
        let act = active_pane.clone();
        let session = session.clone();
        let nav = navigate_pane_ref.clone();
        move || {
            let paths = if act.get() == ActivePane::Left {
                left.selected_paths.borrow().clone()
            } else {
                right.borrow().as_ref()
                    .map(|r| r.selected_paths.borrow().clone())
                    .unwrap_or_default()
            };
            if !paths.is_empty() {
                let current_path = session.borrow().active_tab().current_path.clone();
                babydra_utils::explore::context_menu::clipboard::set_system_clipboard_files(&paths, false);
                babydra_utils::explore::CLIPBOARD.with(|cb| cb.replace(Some((paths, false))));
                if let Some(ref f) = *nav.borrow() {
                    f(act.get(), current_path);
                }
            }
        }
    };
    let copy_cb_rc = Rc::new(copy_cb) as Rc<dyn Fn()>;

    let paste_cb = {
        let session = session.clone();
        let act = active_pane.clone();
        let nav = navigate_pane_ref.clone();
        move || {
            let current_path = session.borrow().active_tab().current_path.clone();
            let nav_cb = {
                let nav = nav.clone();
                let act = act.clone();
                Rc::new(move |p| {
                    if let Some(ref f) = *nav.borrow() {
                        f(act.get(), p);
                    }
                }) as Rc<dyn Fn(PathBuf)>
            };
            babydra_utils::explore::context_menu::clipboard::execute_paste_from_system_clipboard(
                current_path.clone(),
                current_path,
                nav_cb,
            );
        }
    };
    let paste_cb_rc = Rc::new(paste_cb) as Rc<dyn Fn()>;

    let undo_cb = {
        let session = session.clone();
        let act = active_pane.clone();
        let nav = navigate_pane_ref.clone();
        move || {
            let current_path = session.borrow().active_tab().current_path.clone();
            let nav_cb = {
                let nav = nav.clone();
                let act = act.clone();
                Rc::new(move |p| {
                    if let Some(ref f) = *nav.borrow() {
                        f(act.get(), p);
                    }
                }) as Rc<dyn Fn(PathBuf)>
            };
            babydra_utils::explore::context_menu::clipboard::execute_undo(nav_cb, current_path);
        }
    };
    let undo_cb_rc = Rc::new(undo_cb) as Rc<dyn Fn()>;

    let current_key_controller = Rc::new(RefCell::new(None::<gtk4::EventControllerKey>));

    let rebuild_shortcuts = {
        let window = ui.window.clone();
        let current_controller = current_key_controller.clone();
        let toggle_split_view_rc = toggle_split_view_rc.clone();
        let toggle_preview_rc = toggle_preview_rc.clone();
        let toggle_hidden_rc = toggle_hidden_rc.clone();
        let cut_cb_rc = cut_cb_rc.clone();
        let copy_cb_rc = copy_cb_rc.clone();
        let paste_cb_rc = paste_cb_rc.clone();
        let undo_cb_rc = undo_cb_rc.clone();
        move || {
            if let Some(ref old_controller) = *current_controller.borrow() {
                window.remove_controller(old_controller);
            }

            let settings = babydra_common::load_explore_settings();
            let mut shortcuts = Vec::new();

            let mut add_shortcut = |action: &str, cb: Rc<dyn Fn()>| {
                let shortcut_str = settings.get_keybind(action);
                if let Some((keyval, modifiers)) = handlers::events::parse_shortcut(&shortcut_str) {
                    shortcuts.push(handlers::events::KeyShortcut {
                        keyval: keyval.clone(),
                        modifiers,
                        callback: cb.clone(),
                    });
                    
                    if let Some(name) = keyval.name() {
                        if name.len() == 1 {
                            let upper_name = name.to_uppercase();
                            if let Some(upper_key) = gtk4::gdk::Key::from_name(&upper_name) {
                                shortcuts.push(handlers::events::KeyShortcut {
                                    keyval: upper_key,
                                    modifiers,
                                    callback: cb.clone(),
                                });
                            }
                        }
                    }
                }
            };

            add_shortcut("toggle_split", toggle_split_view_rc.clone());
            add_shortcut("toggle_preview", toggle_preview_rc.clone());
            add_shortcut("toggle_hidden", toggle_hidden_rc.clone());
            add_shortcut("cut", cut_cb_rc.clone());
            add_shortcut("copy", copy_cb_rc.clone());
            add_shortcut("paste", paste_cb_rc.clone());
            add_shortcut("undo", undo_cb_rc.clone());

            let new_controller = handlers::events::setup_key_shortcuts(&window, shortcuts);
            current_controller.replace(Some(new_controller));
        }
    };
    let rebuild_shortcuts_rc = Rc::new(rebuild_shortcuts) as Rc<dyn Fn()>;
    rebuild_shortcuts_cell.replace(Some(rebuild_shortcuts_rc.clone()));

    // Install initial shortcuts
    rebuild_shortcuts_rc();

    // Set up window resize response logic
    handlers::setup_window_resize_handler(
        &ui.window,
        ui.layout_paned.clone(),
        revealer.clone(),
        preview_visible.clone(),
        user_wants_preview.clone(),
        status_bar_widgets_cell.clone(),
    );

    // Watcher event receiver loop
    handlers::setup_file_watcher_receiver(
        session.clone(),
        navigate_pane_no_watch_ref.clone(),
        active_pane.clone(),
        left_content_handle.clone(),
        right_content_handle.clone(),
        watch_rx,
    );

    // D-Bus service loop
    handlers::setup_dbus_receiver(navigate_pane_no_watch_ref.clone(), active_pane.clone());

    // Start initial navigation
    let path = session.borrow().active_tab().current_path.clone();
    if let Some(ref f) = *navigate_pane_ref.borrow() {
        f(ActivePane::Left, path);
    }

    ui.window
}
