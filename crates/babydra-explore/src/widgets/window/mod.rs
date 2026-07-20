use gtk4::prelude::*;
use gtk4::ApplicationWindow;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::path::PathBuf;
use babydra_common::{SessionState, ActivePane};

mod render;
pub mod tabs;
pub mod split;
mod navigation;
mod events;

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

    // Create HeaderBar & StatusBar cells
    let header_widgets_cell = Rc::new(RefCell::new(None::<crate::widgets::header_bar::HeaderBarWidgets>));
    let status_bar_widgets_cell = Rc::new(RefCell::new(None::<crate::widgets::status_bar::StatusBarWidgets>));

    // Create TabBar Container
    let tab_bar_box = Rc::new(RefCell::new(None::<gtk4::Box>));

    // Create StatusBar
    let status_bar_widgets = crate::widgets::status_bar::create_status_bar();
    ui.vbox.append(&status_bar_widgets.container);
    let status_bar_lbl_rc = Rc::new(status_bar_widgets.lbl_status.clone());
    status_bar_widgets_cell.replace(Some(status_bar_widgets.clone()));

    // Setup navigation closures
    let (navigate_pane_ref, navigate_pane_no_watch_ref, _watcher) = navigation::setup_navigation(
        session.clone(),
        active_pane.clone(),
        left_content_handle.clone(),
        right_content_handle.clone(),
        left_scroll_cell.clone(),
        right_scroll_cell.clone(),
        status_bar_widgets_cell.clone(),
        header_widgets_cell.clone(),
        tab_bar_box.clone(),
        status_bar_lbl_rc.clone(),
        watch_tx.clone(),
        left_rx,
    );

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
        let header_widgets_c = header_widgets_cell.clone();
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

            // Update button active classes in HeaderBar
            if let Some(ref hw) = *header_widgets_c.borrow() {
                if mode == "list" {
                    hw.btn_view_list.add_css_class("toolbar-btn-active");
                    hw.btn_view_icons.remove_css_class("toolbar-btn-active");
                } else {
                    hw.btn_view_icons.add_css_class("toolbar-btn-active");
                    hw.btn_view_list.remove_css_class("toolbar-btn-active");
                }
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

    // Apply initial view mode class to header buttons based on settings
    if settings.view_mode == "list" {
        header_widgets.btn_view_list.add_css_class("toolbar-btn-active");
        header_widgets.btn_view_icons.remove_css_class("toolbar-btn-active");
    } else {
        header_widgets.btn_view_icons.add_css_class("toolbar-btn-active");
        header_widgets.btn_view_list.remove_css_class("toolbar-btn-active");
    }

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

    // Wire btn_settings click (with live setting refresh)
    {
        let win = ui.window.clone();
        let nav = navigate_pane_ref.clone();
        let active = active_pane.clone();
        let session_c = session.clone();
        let preview_visible = preview_visible.clone();
        let toggle_p = toggle_preview_rc.clone();
        header_widgets.btn_settings.connect_clicked(move |_| {
            let nav_c = nav.clone();
            let act_c = active.clone();
            let session_cc = session_c.clone();
            let preview_v = preview_visible.clone();
            let toggle_p_c = toggle_p.clone();
            let parent_win = win.clone().upcast::<gtk4::Window>();
            crate::widgets::settings_dialog::show_settings_dialog(&parent_win, move || {
                let settings = babydra_common::load_explore_settings();
                
                // 1. Sync hidden files visibility setting in tab state
                let _hidden_changed = {
                    let mut s = session_cc.borrow_mut();
                    let tab = s.active_tab_mut();
                    let old_val = tab.show_hidden;
                    tab.show_hidden = settings.show_hidden;
                    old_val != settings.show_hidden
                };

                // 2. Sync preview panel visibility
                let preview_changed = preview_v.get() != settings.preview_visible;
                if preview_changed {
                    toggle_p_c();
                }

                // 3. Trigger navigate refresh
                let path = session_cc.borrow().active_tab().current_path.clone();
                if let Some(ref f) = *nav_c.borrow() {
                    f(act_c.get(), path);
                }
            });
        });
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

    // Wire keyboard shortcut listeners
    events::setup_key_shortcuts(&ui.window, toggle_split_view_rc, toggle_preview_rc, toggle_hidden_rc);

    // Set up window resize response logic
    events::setup_window_resize_handler(
        &ui.window,
        ui.layout_paned.clone(),
        revealer.clone(),
        preview_visible.clone(),
        user_wants_preview.clone(),
        status_bar_widgets_cell.clone(),
    );

    // Watcher event receiver loop
    events::setup_file_watcher_receiver(session.clone(), navigate_pane_no_watch_ref.clone(), active_pane.clone(), watch_rx);

    // D-Bus service loop
    events::setup_dbus_receiver(navigate_pane_no_watch_ref.clone(), active_pane.clone());

    // Start initial navigation
    let path = session.borrow().active_tab().current_path.clone();
    if let Some(ref f) = *navigate_pane_ref.borrow() {
        f(ActivePane::Left, path);
    }

    ui.window
}
