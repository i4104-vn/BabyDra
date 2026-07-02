//! UI renderer and main window coordinator for the search/app launcher overlay.

use crate::core::find_desktop_apps;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer};
use std::rc::Rc;
use std::cell::RefCell;
use crate::widgets::footer::create_launcher_footer;
use crate::widgets::app_row::create_list_app_widget;
use crate::widgets::search::populate_search_results;

/// Builds the application launcher UI, connecting its key navigation,
/// search entry box, left-column app grid, and right-column file/web results.
pub fn build_launcher_ui(
    app: &gtk4::Application,
    launcher_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
) -> gtk4::ApplicationWindow {
    let window = gtk4::ApplicationWindow::new(app);
    babydra_common::apply_theme_class(&window);
    
    babydra_common::window::init_layer_window(
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

    let box_layout = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    box_layout.add_css_class("launcher-box");
    box_layout.set_halign(gtk4::Align::Start);
    box_layout.set_valign(gtk4::Align::Start);
    box_layout.set_size_request(780, 560);
    box_layout.set_margin_top(50);
    box_layout.set_margin_start(12);

    let search_entry = gtk4::Entry::new();
    search_entry.set_placeholder_text(Some(&babydra_common::i18n::t("launcher.search_placeholder")));
    search_entry.add_css_class("launcher-search");
    search_entry.set_margin_top(16);
    search_entry.set_margin_start(16);
    search_entry.set_margin_end(16);

    let columns_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    columns_box.add_css_class("launcher-columns-box");
    columns_box.set_vexpand(true);

    let left_scroll = gtk4::ScrolledWindow::new();
    left_scroll.add_css_class("launcher-left-column");
    left_scroll.set_size_request(280, -1);
    left_scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);

    let left_list_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

    let apps = find_desktop_apps();
    let apps_rc = Rc::new(apps);

    let toggle_btn = gtk4::Button::new();
    toggle_btn.add_css_class("launcher-toggle-btn");
    let toggle_label_text_expanded = format!("{}  ▼", babydra_common::i18n::t("launcher.other_apps"));
    let toggle_label_text_collapsed = format!("{}  ▶", babydra_common::i18n::t("launcher.other_apps"));
    toggle_btn.set_label(&toggle_label_text_collapsed);

    let mut app_widgets = Vec::new();
    let mut installed_widgets = Vec::new();
    let mut dep_widgets = Vec::new();

    for app in apps_rc.iter() {
        let btn = create_list_app_widget(app, &window);
        app_widgets.push((app.clone(), btn.clone()));
        if app.is_dependency {
            dep_widgets.push((app.clone(), btn));
        } else {
            installed_widgets.push((app.clone(), btn));
        }
    }

    for (_, btn) in &installed_widgets {
        left_list_box.append(btn);
    }

    left_list_box.append(&toggle_btn);

    for (_, btn) in &dep_widgets {
        left_list_box.append(btn);
        btn.set_visible(false);
    }

    let expanded = Rc::new(RefCell::new(false));
    let expanded_clone = expanded.clone();
    let dep_widgets_clone = dep_widgets.clone();
    let toggle_btn_clone = toggle_btn.clone();
    let toggle_label_text_expanded_clone = toggle_label_text_expanded.clone();
    let toggle_label_text_collapsed_clone = toggle_label_text_collapsed.clone();

    toggle_btn.connect_clicked(move |_| {
        let mut exp = expanded_clone.borrow_mut();
        *exp = !*exp;
        if *exp {
            toggle_btn_clone.set_label(&toggle_label_text_expanded_clone);
        } else {
            toggle_btn_clone.set_label(&toggle_label_text_collapsed_clone);
        }
        for (_, btn) in &dep_widgets_clone {
            btn.set_visible(*exp);
        }
    });

    let app_widgets_rc = Rc::new(app_widgets);
    left_scroll.set_child(Some(&left_list_box));

    let right_scroll = gtk4::ScrolledWindow::new();
    right_scroll.add_css_class("launcher-right-column");
    right_scroll.set_hexpand(true);
    right_scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);

    let current_query = Rc::new(RefCell::new(String::new()));
    let selected_index = Rc::new(RefCell::new(Some(0usize)));

    let populate_impl = {
        let current_query = current_query.clone();
        let app_widgets = app_widgets_rc.clone();
        let right_scroll = right_scroll.clone();
        let window = window.clone();
        let toggle_btn = toggle_btn.clone();
        let expanded = expanded.clone();
        let dep_count = dep_widgets.len();

        move || {
            let query = current_query.borrow().trim().to_lowercase();

            if query.is_empty() {
                toggle_btn.set_visible(dep_count > 0);
                for (app, btn) in app_widgets.iter() {
                    if app.is_dependency {
                        btn.set_visible(*expanded.borrow());
                    } else {
                        btn.set_visible(true);
                    }
                }
            } else {
                toggle_btn.set_visible(false);
                for (app, btn) in app_widgets.iter() {
                    let matches = app.name.to_lowercase().contains(&query);
                    btn.set_visible(matches);
                }
            }

            populate_search_results(
                &right_scroll,
                &current_query.borrow(),
                &window,
            );
        }
    };

    let populate_impl_rc = Rc::new(populate_impl);

    populate_impl_rc();
    update_highlight(&left_list_box, &right_scroll, Some(0));

    let debounce_source_id: Rc<RefCell<Option<gtk4::glib::SourceId>>> = Rc::new(RefCell::new(None));
    let current_query_search = current_query.clone();
    let populate_grid_search = populate_impl_rc.clone();
    let d_source_id = debounce_source_id.clone();
    let selected_index_changed = selected_index.clone();
    let left_list_box_changed = left_list_box.clone();
    let right_scroll_changed = right_scroll.clone();

    search_entry.connect_changed(move |entry| {
        let text = entry.text().to_string();
        *current_query_search.borrow_mut() = text;
        
        if let Some(source_id) = d_source_id.borrow_mut().take() {
            source_id.remove();
        }
        
        let populate_clone = populate_grid_search.clone();
        let d_source_id_clone = d_source_id.clone();
        let selected_index_clone = selected_index_changed.clone();
        let left_list_box_clone = left_list_box_changed.clone();
        let right_scroll_clone = right_scroll_changed.clone();

        let new_source_id = gtk4::glib::timeout_add_local_once(
            std::time::Duration::from_millis(200),
            move || {
                populate_clone();
                *d_source_id_clone.borrow_mut() = None;
                *selected_index_clone.borrow_mut() = Some(0);
                update_highlight(&left_list_box_clone, &right_scroll_clone, Some(0));
            }
        );
        *d_source_id.borrow_mut() = Some(new_source_id);
    });

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
        babydra_common::animation::slide_out_cb(
            box_layout_cb.upcast_ref(),
            babydra_common::animation::SlideDirection::Up,
            40,
            450,
            false,
            move || {
                win_cb.destroy();
            }
        );
        gtk4::glib::Propagation::Stop
    });

    babydra_common::window::setup_click_outside_dismiss(&window, &box_layout);

    window.connect_is_active_notify(|win| {
        if !win.is_active() {
            win.close();
        }
    });

    let key_controller = gtk4::EventControllerKey::new();
    key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    let win_clone = window.clone();
    let left_list_box_clone = left_list_box.clone();
    let right_scroll_clone = right_scroll.clone();
    let selected_index_clone = selected_index.clone();
    let search_entry_clone = search_entry.clone();

    key_controller.connect_key_pressed(move |_, key, _, _| {
        match key {
            gtk4::gdk::Key::Escape => {
                win_clone.close();
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Down => {
                let buttons = get_visible_selectable_buttons(&left_list_box_clone, &right_scroll_clone);
                if !buttons.is_empty() {
                    let current = selected_index_clone.borrow().unwrap_or(0);
                    let next = (current + 1) % buttons.len();
                    *selected_index_clone.borrow_mut() = Some(next);
                    update_highlight(&left_list_box_clone, &right_scroll_clone, Some(next));
                    buttons[next].grab_focus();
                }
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Up => {
                let buttons = get_visible_selectable_buttons(&left_list_box_clone, &right_scroll_clone);
                if !buttons.is_empty() {
                    let current = selected_index_clone.borrow().unwrap_or(0);
                    let prev = if current == 0 { buttons.len() - 1 } else { current - 1 };
                    *selected_index_clone.borrow_mut() = Some(prev);
                    update_highlight(&left_list_box_clone, &right_scroll_clone, Some(prev));
                    buttons[prev].grab_focus();
                }
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Return | gtk4::gdk::Key::KP_Enter => {
                let buttons = get_visible_selectable_buttons(&left_list_box_clone, &right_scroll_clone);
                if let Some(idx) = *selected_index_clone.borrow() {
                    if idx < buttons.len() {
                        buttons[idx].activate();
                    }
                } else if !buttons.is_empty() {
                    buttons[0].activate();
                }
                gtk4::glib::Propagation::Stop
            }
            _ => {
                if !search_entry_clone.is_focus() {
                    if let Some(c) = key.to_unicode() {
                        if !c.is_control() {
                            let text = search_entry_clone.text().to_string();
                            search_entry_clone.set_text(&format!("{}{}", text, c));
                            search_entry_clone.set_position(-1);
                            search_entry_clone.grab_focus();
                            return gtk4::glib::Propagation::Stop;
                        }
                    } else if key == gtk4::gdk::Key::BackSpace {
                        let mut text = search_entry_clone.text().to_string();
                        text.pop();
                        search_entry_clone.set_text(&text);
                        search_entry_clone.set_position(-1);
                        search_entry_clone.grab_focus();
                        return gtk4::glib::Propagation::Stop;
                    }
                }
                gtk4::glib::Propagation::Proceed
            }
        }
    });
    window.add_controller(key_controller);

    box_layout.append(&search_entry);
    
    columns_box.append(&left_scroll);
    columns_box.append(&right_scroll);
    columns_box.set_margin_start(16);
    columns_box.set_margin_end(16);
    box_layout.append(&columns_box);

    let footer_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    footer_sep.add_css_class("launcher-footer-separator");
    footer_sep.set_margin_start(16);
    footer_sep.set_margin_end(16);
    box_layout.append(&footer_sep);

    let footer = create_launcher_footer();
    footer.set_margin_start(16);
    footer.set_margin_end(16);
    footer.set_margin_bottom(16);
    box_layout.append(&footer);

    window.set_child(Some(&box_layout));

    babydra_common::animation::slide_in(
        box_layout.upcast_ref(),
        babydra_common::animation::SlideDirection::Down,
        40,
        450,
    );

    let left_list_box_focus = left_list_box.clone();
    let right_scroll_focus = right_scroll.clone();
    let search_entry_focus = search_entry.clone();
    window.connect_map(move |_| {
        gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(50), {
            let left_list_box = left_list_box_focus.clone();
            let right_scroll = right_scroll_focus.clone();
            let search_entry = search_entry_focus.clone();
            move || {
                let buttons = get_visible_selectable_buttons(&left_list_box, &right_scroll);
                if !buttons.is_empty() {
                    buttons[0].grab_focus();
                } else {
                    search_entry.grab_focus();
                }
            }
        });
    });

    window
}

/// Helper function to traverse left and right grids to collect all visible clickable button widgets.
fn get_visible_selectable_buttons(
    left_list_box: &gtk4::Box,
    right_scroll: &gtk4::ScrolledWindow,
) -> Vec<gtk4::Button> {
    let mut buttons = Vec::new();

    let mut child = left_list_box.first_child();
    while let Some(w) = child {
        if w.is_visible() {
            if let Some(btn) = w.downcast_ref::<gtk4::Button>() {
                buttons.push(btn.clone());
            }
        }
        child = w.next_sibling();
    }

    if let Some(right_child) = right_scroll.child() {
        if let Some(right_box) = right_child.downcast_ref::<gtk4::Box>() {
            let mut sub_child = right_box.first_child();
            while let Some(w) = sub_child {
                if w.is_visible() {
                    if let Some(btn) = w.downcast_ref::<gtk4::Button>() {
                        buttons.push(btn.clone());
                    } else if let Some(files_box) = w.downcast_ref::<gtk4::Box>() {
                        let mut file_child = files_box.first_child();
                        while let Some(fw) = file_child {
                            if fw.is_visible() {
                                if let Some(btn) = fw.downcast_ref::<gtk4::Button>() {
                                    buttons.push(btn.clone());
                                }
                            }
                            file_child = fw.next_sibling();
                        }
                    }
                }
                sub_child = w.next_sibling();
            }
        }
    }

    buttons
}

/// Applies highlit/selected styling to the active child button index while removing it from others.
fn update_highlight(
    left_list_box: &gtk4::Box,
    right_scroll: &gtk4::ScrolledWindow,
    selected_idx: Option<usize>,
) {
    let buttons = get_visible_selectable_buttons(left_list_box, right_scroll);
    for (i, btn) in buttons.iter().enumerate() {
        if Some(i) == selected_idx {
            btn.add_css_class("selected");
        } else {
            btn.remove_css_class("selected");
        }
    }
}

