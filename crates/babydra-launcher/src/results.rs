//! Result list helpers for the launcher (keyboard navigation, highlight, repopulation).
//! Split out of `render.rs` to keep the main builder focused on layout.

use crate::widgets::app_row::create_list_app_widget;
use crate::widgets::file_search::create_file_row;
use crate::widgets::search::build_browser_search_button;
use babydra_core::search_files;
use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;

pub fn get_visible_selectable_buttons(list_box: &gtk4::Box) -> Vec<gtk4::Button> {
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
pub fn update_highlight(list_box: &gtk4::Box, selected_idx: Option<usize>) {
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
pub fn repopulate_results(
    list_box: &gtk4::Box,
    query: &str,
    apps: &[babydra_core::DesktopApp],
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
        let app_title = gtk4::Label::new(Some(&babydra_core::i18n::t("launcher.apps")));
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
            let app_title = gtk4::Label::new(Some(&babydra_core::i18n::t("launcher.apps")));
            app_title.add_css_class("launcher-section-title");
            app_title.set_halign(gtk4::Align::Start);
            list_box.append(&app_title);

            for app in &matched_apps {
                let btn = create_list_app_widget(app, window);
                list_box.append(&btn);
            }
        }

        // --- 2. WEB SEARCH GROUP ---
        let web_title = gtk4::Label::new(Some(&babydra_core::i18n::t("launcher.web_search")));
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
                        let files_title =
                            gtk4::Label::new(Some(&babydra_core::i18n::t("launcher.files_dirs")));
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
                Err(std::sync::mpsc::TryRecvError::Empty) => gtk4::glib::ControlFlow::Continue,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => gtk4::glib::ControlFlow::Break,
            }
        });
    }

    update_highlight(list_box, *selected_index.borrow());
}
