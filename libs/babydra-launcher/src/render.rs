//! UI renderer and main window coordinator for the search/app launcher overlay.

use babydra_common::find_desktop_apps;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer};
use std::rc::Rc;
use std::cell::RefCell;
use crate::widgets::footer::create_launcher_footer;
use crate::widgets::app_row::create_list_app_widget;
use crate::widgets::search::build_browser_search_button;
use crate::widgets::file_search::create_file_row;
use babydra_common::search_files;

/// Builds the application launcher UI, connecting its key navigation,
/// search entry box, and single-column grouped results list.
pub fn build_launcher_ui(
    app: &gtk4::Application,
    launcher_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
) -> gtk4::ApplicationWindow {
    let window = gtk4::ApplicationWindow::new(app);
    babydra_utils::ui::theme::apply_theme_class(&window);
    
    babydra_utils::ui::window::init_layer_window(
        &window,
        Layer::Overlay,
        KeyboardMode::OnDemand,
        -1,
        &[
            (Edge::Top, true),
            (Edge::Bottom, true),
            (Edge::Left, true),
            (Edge::Right, true),
        ],
        -1,
    );

    window.add_css_class("launcher-window");

    let box_layout = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    box_layout.add_css_class("launcher-box");
    box_layout.set_halign(gtk4::Align::Start);
    box_layout.set_valign(gtk4::Align::Start);
    box_layout.set_size_request(420, 600);
    box_layout.set_margin_top(50);
    box_layout.set_margin_start(16);

    // --- Header ---
    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    header_box.add_css_class("menu-header");
    header_box.set_margin_top(20);
    header_box.set_margin_start(24);
    header_box.set_margin_end(24);
    header_box.set_margin_bottom(12);

    let brand_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    brand_box.set_valign(gtk4::Align::Center);

    let brand_label = gtk4::Label::new(Some("Applications"));
    brand_label.add_css_class("brand-text");
    brand_label.set_valign(gtk4::Align::Center);

    brand_box.append(&brand_label);

    header_box.append(&brand_box);
    box_layout.append(&header_box);

    // --- Search Entry ---
    let search_entry = gtk4::Entry::new();
    search_entry.set_placeholder_text(Some("Type to search apps, files or web..."));
    search_entry.add_css_class("launcher-search");
    search_entry.set_margin_start(24);
    search_entry.set_margin_end(24);
    search_entry.set_margin_bottom(12);
    box_layout.append(&search_entry);

    // --- Content Scroll Area ---
    let scrolled_window = gtk4::ScrolledWindow::new();
    scrolled_window.add_css_class("list-container");
    scrolled_window.set_hexpand(true);
    scrolled_window.set_vexpand(true);
    scrolled_window.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scrolled_window.set_margin_start(24);
    scrolled_window.set_margin_end(24);

    let list_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    scrolled_window.set_child(Some(&list_box));
    box_layout.append(&scrolled_window);

    // --- Footer & Separator ---
    let footer_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    footer_sep.add_css_class("launcher-footer-separator");
    footer_sep.set_margin_start(20);
    footer_sep.set_margin_end(20);
    footer_sep.set_margin_top(6);
    footer_sep.set_margin_bottom(6);
    box_layout.append(&footer_sep);

    let footer = create_launcher_footer();
    footer.set_margin_start(20);
    footer.set_margin_end(20);
    footer.set_margin_bottom(14);
    box_layout.append(&footer);

    window.set_child(Some(&box_layout));

    // Apps list
    let apps = find_desktop_apps();
    let apps_rc = Rc::new(apps);

    // State
    let current_query = Rc::new(RefCell::new(String::new()));
    let selected_index = Rc::new(RefCell::new(Some(0usize)));
    let expanded = Rc::new(RefCell::new(false));

    // Toggle button for other apps
    let toggle_btn = gtk4::Button::new();
    toggle_btn.add_css_class("launcher-toggle-btn");
    toggle_btn.set_cursor_from_name(Some("pointer"));
    let toggle_label_text_collapsed = format!("{}  ▶", babydra_common::i18n::t("launcher.other_apps"));
    toggle_btn.set_label(&toggle_label_text_collapsed);

    let expanded_clone = expanded.clone();
    let toggle_btn_clone = toggle_btn.clone();
    let list_box_clone = list_box.clone();
    let current_query_clone = current_query.clone();
    let apps_clone = apps_rc.clone();
    let window_clone = window.clone();
    let selected_index_clone = selected_index.clone();

    toggle_btn.connect_clicked(move |_| {
        let mut exp = expanded_clone.borrow_mut();
        *exp = !*exp;
        
        let toggle_label_text = if *exp {
            format!("{}  ▼", babydra_common::i18n::t("launcher.other_apps"))
        } else {
            format!("{}  ▶", babydra_common::i18n::t("launcher.other_apps"))
        };
        toggle_btn_clone.set_label(&toggle_label_text);

        repopulate_results(
            &list_box_clone,
            &current_query_clone.borrow(),
            &apps_clone,
            &window_clone,
            *exp,
            &toggle_btn_clone,
            selected_index_clone.clone(),
        );
    });

    // Populate initial state
    repopulate_results(
        &list_box,
        &current_query.borrow(),
        &apps_rc,
        &window,
        *expanded.borrow(),
        &toggle_btn,
        selected_index.clone(),
    );

    // Search entry connect
    let debounce_source_id: Rc<RefCell<Option<gtk4::glib::SourceId>>> = Rc::new(RefCell::new(None));
    let current_query_search = current_query.clone();
    let d_source_id = debounce_source_id.clone();
    let list_box_search = list_box.clone();
    let apps_search = apps_rc.clone();
    let window_search = window.clone();
    let expanded_search = expanded.clone();
    let toggle_btn_search = toggle_btn.clone();
    let selected_index_search = selected_index.clone();

    search_entry.connect_changed(move |entry| {
        let text = entry.text().to_string();
        *current_query_search.borrow_mut() = text;
        
        if let Some(source_id) = d_source_id.borrow_mut().take() {
            source_id.remove();
        }
        
        let current_query = current_query_search.clone();
        let d_source_id_clone = d_source_id.clone();
        let list_box = list_box_search.clone();
        let apps = apps_search.clone();
        let window = window_search.clone();
        let expanded = expanded_search.clone();
        let toggle_btn = toggle_btn_search.clone();
        let selected_index = selected_index_search.clone();

        let new_source_id = gtk4::glib::timeout_add_local_once(
            std::time::Duration::from_millis(200),
            move || {
                *d_source_id_clone.borrow_mut() = None;
                *selected_index.borrow_mut() = Some(0);
                repopulate_results(
                    &list_box,
                    &current_query.borrow(),
                    &apps,
                    &window,
                    *expanded.borrow(),
                    &toggle_btn,
                    selected_index.clone(),
                );
            }
        );
        *d_source_id.borrow_mut() = Some(new_source_id);
    });

    // Handle slide animations
    let is_animating = Rc::new(std::cell::Cell::new(false));
    let is_animating_clone = is_animating.clone();
    let win_clone_close = window.clone();
    let box_layout_clone_close = box_layout.clone();
    let lw_inner = launcher_window.clone();
    window.connect_close_request(move |_| {
        if is_animating_clone.get() {
            return gtk4::glib::Propagation::Stop;
        }
        is_animating_clone.set(true);
        if let Ok(mut borrow) = lw_inner.try_borrow_mut() {
            *borrow = None;
        }
        let win_cb = win_clone_close.clone();
        let box_layout_cb = box_layout_clone_close.clone();
        babydra_utils::ui::animation::slide_out_cb(
            box_layout_cb.upcast_ref(),
            babydra_utils::ui::animation::SlideDirection::Up,
            40,
            450,
            false,
            move || {
                win_cb.destroy();
            }
        );
        gtk4::glib::Propagation::Stop
    });

    babydra_utils::ui::window::setup_click_outside_dismiss(&window, &box_layout);

    window.connect_is_active_notify(|win| {
        if !win.is_active() {
            win.close();
        }
    });

    // Key event controller for keyboard navigation
    let key_controller = gtk4::EventControllerKey::new();
    key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    let win_clone = window.clone();
    let list_box_key = list_box.clone();
    let selected_index_key = selected_index.clone();
    let search_entry_key = search_entry.clone();

    key_controller.connect_key_pressed(move |_, key, _, _| {
        match key {
            gtk4::gdk::Key::Escape => {
                win_clone.close();
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Down => {
                let buttons = get_visible_selectable_buttons(&list_box_key);
                if !buttons.is_empty() {
                    let current = selected_index_key.borrow().unwrap_or(0);
                    let next = (current + 1) % buttons.len();
                    *selected_index_key.borrow_mut() = Some(next);
                    update_highlight(&list_box_key, Some(next));
                    buttons[next].grab_focus();
                }
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Up => {
                let buttons = get_visible_selectable_buttons(&list_box_key);
                if !buttons.is_empty() {
                    let current = selected_index_key.borrow().unwrap_or(0);
                    let prev = if current == 0 { buttons.len() - 1 } else { current - 1 };
                    *selected_index_key.borrow_mut() = Some(prev);
                    update_highlight(&list_box_key, Some(prev));
                    buttons[prev].grab_focus();
                }
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Return | gtk4::gdk::Key::KP_Enter => {
                let buttons = get_visible_selectable_buttons(&list_box_key);
                if let Some(idx) = *selected_index_key.borrow() {
                    if idx < buttons.len() {
                        buttons[idx].activate();
                    }
                } else if !buttons.is_empty() {
                    buttons[0].activate();
                }
                gtk4::glib::Propagation::Stop
            }
            _ => {
                if !search_entry_key.is_focus() {
                    if let Some(c) = key.to_unicode() {
                        if !c.is_control() {
                            let text = search_entry_key.text().to_string();
                            search_entry_key.set_text(&format!("{}{}", text, c));
                            search_entry_key.set_position(-1);
                            search_entry_key.grab_focus();
                            return gtk4::glib::Propagation::Stop;
                        }
                    } else if key == gtk4::gdk::Key::BackSpace {
                        let mut text = search_entry_key.text().to_string();
                        text.pop();
                        search_entry_key.set_text(&text);
                        search_entry_key.set_position(-1);
                        search_entry_key.grab_focus();
                        return gtk4::glib::Propagation::Stop;
                    }
                }
                gtk4::glib::Propagation::Proceed
            }
        }
    });
    window.add_controller(key_controller);

    // Focus state listeners
    let list_box_focus = list_box.clone();
    let selected_index_focus = selected_index.clone();

    window.connect_focus_widget_notify(move |win| {
        if let Some(focus_widget) = gtk4::prelude::RootExt::focus(win) {
            let buttons = get_visible_selectable_buttons(&list_box_focus);
            if let Some(pos) = buttons.iter().position(|b| b.clone().upcast::<gtk4::Widget>() == focus_widget) {
                *selected_index_focus.borrow_mut() = Some(pos);
                update_highlight(&list_box_focus, Some(pos));
            }
        }
    });

    let list_box_map = list_box.clone();
    let search_entry_map = search_entry.clone();
    window.connect_map(move |_| {
        gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(50), {
            let list_box = list_box_map.clone();
            let search_entry = search_entry_map.clone();
            move || {
                let buttons = get_visible_selectable_buttons(&list_box);
                if !buttons.is_empty() {
                    buttons[0].grab_focus();
                } else {
                    search_entry.grab_focus();
                }
            }
        });
    });

    babydra_utils::ui::animation::slide_in(
        box_layout.upcast_ref(),
        babydra_utils::ui::animation::SlideDirection::Down,
        40,
        450,
    );

    window
}

/// Helper function to traverse the list box to collect all visible clickable button widgets.
fn get_visible_selectable_buttons(list_box: &gtk4::Box) -> Vec<gtk4::Button> {
    let mut buttons = Vec::new();
    let mut child = list_box.first_child();
    while let Some(w) = child {
        if w.is_visible() {
            if let Some(btn) = w.downcast_ref::<gtk4::Button>() {
                buttons.push(btn.clone());
            } else if let Some(sub_box) = w.downcast_ref::<gtk4::Box>() {
                let mut sub_child = sub_box.first_child();
                while let Some(sw) = sub_child {
                    if sw.is_visible() {
                        if let Some(btn) = sw.downcast_ref::<gtk4::Button>() {
                            buttons.push(btn.clone());
                        }
                    }
                    sub_child = sw.next_sibling();
                }
            }
        }
        child = w.next_sibling();
    }
    buttons
}

/// Applies active highlit/selected styling to the active child button index while removing it from others.
fn update_highlight(list_box: &gtk4::Box, selected_idx: Option<usize>) {
    let buttons = get_visible_selectable_buttons(list_box);
    for (i, btn) in buttons.iter().enumerate() {
        if Some(i) == selected_idx {
            btn.add_css_class("selected");
        } else {
            btn.remove_css_class("selected");
        }
    }
}

/// Re-populates the unified results list box dynamically.
fn repopulate_results(
    list_box: &gtk4::Box,
    query: &str,
    apps: &[babydra_common::DesktopApp],
    window: &gtk4::ApplicationWindow,
    expanded: bool,
    toggle_btn: &gtk4::Button,
    selected_index: Rc<RefCell<Option<usize>>>,
) {
    // 1. Remove all children from list_box
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    let q = query.trim().to_lowercase();

    if q.is_empty() {
        // --- 1. APPLICATIONS GROUP (When empty query) ---
        let app_title = gtk4::Label::new(Some("Applications"));
        app_title.add_css_class("launcher-section-title");
        app_title.set_halign(gtk4::Align::Start);
        list_box.append(&app_title);

        let mut dep_count = 0;
        for app in apps {
            if !app.is_dependency {
                let btn = create_list_app_widget(app, window);
                list_box.append(&btn);
            } else {
                dep_count += 1;
            }
        }

        // Add the toggle button if there are dependency apps
        if dep_count > 0 {
            list_box.append(toggle_btn);

            // Add dependency apps (visible only if expanded)
            for app in apps {
                if app.is_dependency {
                    let btn = create_list_app_widget(app, window);
                    btn.set_visible(expanded);
                    list_box.append(&btn);
                }
            }
        }
    } else {
        // --- 1. APPLICATIONS GROUP (When filtering) ---
        let mut matched_apps = Vec::new();
        for app in apps {
            if app.name.to_lowercase().contains(&q) {
                matched_apps.push(app.clone());
            }
        }

        if !matched_apps.is_empty() {
            let app_title = gtk4::Label::new(Some("Applications"));
            app_title.add_css_class("launcher-section-title");
            app_title.set_halign(gtk4::Align::Start);
            list_box.append(&app_title);

            for app in &matched_apps {
                let btn = create_list_app_widget(app, window);
                list_box.append(&btn);
            }
        }

        // --- 2. WEB SEARCH GROUP ---
        let web_title = gtk4::Label::new(Some("Web Search"));
        web_title.add_css_class("launcher-section-title");
        web_title.set_halign(gtk4::Align::Start);
        list_box.append(&web_title);

        let (browser_btn, _) = build_browser_search_button(&query);
        let q_for_browser = query.to_string();
        let win_to_close = window.clone();
        browser_btn.connect_clicked(move |_| {
            let search_query = q_for_browser.replace(' ', "+");
            let url = format!("https://www.google.com/search?q={}", search_query);
            let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
            win_to_close.close();
        });

        let motion = gtk4::EventControllerMotion::new();
        let btn_clone = browser_btn.clone();
        motion.connect_enter(move |_, _, _| {
            btn_clone.grab_focus();
        });
        browser_btn.add_controller(motion);
        list_box.append(&browser_btn);

        // --- 3. FILES GROUP (Asynchronous search placeholder container) ---
        let files_container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        list_box.append(&files_container);

        let query_for_search = query.to_string();
        let (sender, receiver) = std::sync::mpsc::channel::<Vec<std::path::PathBuf>>();

        std::thread::spawn(move || {
            let results = search_files(&query_for_search);
            let _ = sender.send(results);
        });

        let win_clone = window.clone();
        let list_box_clone = list_box.clone();
        let selected_index_clone = selected_index.clone();
        
        gtk4::glib::idle_add_local(move || {
            match receiver.try_recv() {
                Ok(matched_files) => {
                    if !matched_files.is_empty() {
                        let files_title = gtk4::Label::new(Some("Files & Directories"));
                        files_title.add_css_class("launcher-section-title");
                        files_title.set_halign(gtk4::Align::Start);
                        files_container.append(&files_title);

                        for file_path in &matched_files {
                            let btn = create_file_row(file_path, &win_clone);
                            files_container.append(&btn);
                        }

                        // Make sure highlight adapts after files load
                        update_highlight(&list_box_clone, *selected_index_clone.borrow());
                    }
                    gtk4::glib::ControlFlow::Break
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    gtk4::glib::ControlFlow::Continue
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    gtk4::glib::ControlFlow::Break
                }
            }
        });
    }

    update_highlight(list_box, *selected_index.borrow());
}
