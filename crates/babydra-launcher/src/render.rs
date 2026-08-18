//! UI renderer and main window coordinator for the search/app launcher overlay.

use crate::results::{get_visible_buttons, repopulate_results, update_highlight};
use crate::widgets::footer::create_launcher_foot;
use babydra_core::find_desktop_apps;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer};
use std::cell::RefCell;
use std::rc::Rc;

/// Builds the application launcher UI, connecting its key navigation,
/// search entry box, and single-column grouped results list.
pub fn build_launcher_ui(
    app: &gtk4::Application,
    launcher_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
) -> gtk4::ApplicationWindow {
    let window = gtk4::ApplicationWindow::new(app);
    babydra_ui_kit::ui::theme::apply_theme_class(&window);

    babydra_ui_kit::ui::window::init_layer_window(
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
        None,
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

    let brand_label = gtk4::Label::new(Some(&babydra_core::i18n::trans("launcher.apps")));
    brand_label.add_css_class("brand-text");
    brand_label.set_valign(gtk4::Align::Center);

    brand_box.append(&brand_label);

    header_box.append(&brand_box);
    box_layout.append(&header_box);

    // --- Search Entry ---
    let search_entry = gtk4::Entry::new();
    search_entry.set_placeholder_text(Some(&babydra_core::i18n::trans("launcher.search_hint")));
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

    let footer = create_launcher_foot();
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
    let toggle_label_text_collapsed =
        format!("{}  ▶", babydra_core::i18n::trans("launcher.other_apps"));
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
            format!("{}  ▼", babydra_core::i18n::trans("launcher.other_apps"))
        } else {
            format!("{}  ▶", babydra_core::i18n::trans("launcher.other_apps"))
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

        let new_source_id =
            gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(200), move || {
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
            });
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
        babydra_ui_kit::ui::animation::slide_out_cb(
            box_layout_cb.upcast_ref(),
            babydra_ui_kit::ui::animation::SlideDirection::Up,
            40,
            450,
            false,
            move || {
                win_cb.destroy();
            },
        );
        gtk4::glib::Propagation::Stop
    });

    babydra_ui_kit::ui::window::setup_click_outside_dismiss(&window, &box_layout);

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

    key_controller.connect_key_pressed(move |_, key, _, _| match key {
        gtk4::gdk::Key::Escape => {
            win_clone.close();
            gtk4::glib::Propagation::Stop
        }
        gtk4::gdk::Key::Down => {
            let buttons = get_visible_buttons(&list_box_key);
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
            let buttons = get_visible_buttons(&list_box_key);
            if !buttons.is_empty() {
                let current = selected_index_key.borrow().unwrap_or(0);
                let prev = if current == 0 {
                    buttons.len() - 1
                } else {
                    current - 1
                };
                *selected_index_key.borrow_mut() = Some(prev);
                update_highlight(&list_box_key, Some(prev));
                buttons[prev].grab_focus();
            }
            gtk4::glib::Propagation::Stop
        }
        gtk4::gdk::Key::Return | gtk4::gdk::Key::KP_Enter => {
            let buttons = get_visible_buttons(&list_box_key);
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
    });
    window.add_controller(key_controller);

    // Focus state listeners
    let list_box_focus = list_box.clone();
    let selected_index_focus = selected_index.clone();

    window.connect_focus_widget_notify(move |win| {
        if let Some(focus_widget) = gtk4::prelude::RootExt::focus(win) {
            let buttons = get_visible_buttons(&list_box_focus);
            if let Some(pos) = buttons
                .iter()
                .position(|b| b.clone().upcast::<gtk4::Widget>() == focus_widget)
            {
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
                let buttons = get_visible_buttons(&list_box);
                if !buttons.is_empty() {
                    buttons[0].grab_focus();
                } else {
                    search_entry.grab_focus();
                }
            }
        });
    });

    babydra_ui_kit::ui::animation::slide_in(
        box_layout.upcast_ref(),
        babydra_ui_kit::ui::animation::SlideDirection::Down,
        40,
        450,
    );

    window
}
