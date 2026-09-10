use crate::{
    filter_apps_for_workspace, get_current_workspace, switch_workspace, DEFAULT_WORKSPACE_COUNT,
};
use babydra_core::focus_window;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_workspace_popover(parent: &impl IsA<gtk4::Widget>) -> gtk4::Popover {
    let popover = gtk4::Popover::new();
    popover.set_parent(parent);
    popover.set_position(gtk4::PositionType::Bottom);
    popover.add_css_class("workspace-switcher-popover");
    popover.set_autohide(true);

    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    main_box.set_size_request(230, -1);
    main_box.add_css_class("workspace-popover-container");

    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    header_box.add_css_class("workspace-popover-header");

    let header_icon = babydra_ui_kit::ui::icon::get_icon("view-grid-symbolic", 14);
    header_icon.add_css_class("workspace-popover-header-icon");
    header_box.append(&header_icon);

    let header_text = babydra_core::i18n::trans("common.workspaces");
    let header_lbl = gtk4::Label::new(Some(&header_text));
    header_lbl.add_css_class("workspace-popover-header-title");
    header_lbl.set_hexpand(true);
    header_lbl.set_halign(gtk4::Align::Start);
    header_box.append(&header_lbl);

    main_box.append(&header_box);

    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.add_css_class("workspace-popover-separator");
    main_box.append(&sep);

    let items_box = gtk4::Box::new(gtk4::Orientation::Vertical, 3);
    items_box.add_css_class("workspace-popover-items");
    main_box.append(&items_box);

    popover.set_child(Some(&main_box));

    let active_flyout: Rc<RefCell<Option<gtk4::Popover>>> = Rc::new(RefCell::new(None));
    let stored_flyouts: Rc<RefCell<Vec<gtk4::Popover>>> = Rc::new(RefCell::new(Vec::new()));

    let rebuild_popover = {
        let items_box = items_box.clone();
        let popover_c = popover.clone();
        let active_flyout_c = active_flyout.clone();
        let stored_flyouts_c = stored_flyouts.clone();

        Rc::new(move || {
            if let Some(old_flyout) = active_flyout_c.borrow_mut().take() {
                old_flyout.popdown();
            }
            for fly in stored_flyouts_c.borrow_mut().drain(..) {
                fly.unparent();
            }

            while let Some(child) = items_box.first_child() {
                items_box.remove(&child);
            }

            let running_apps = babydra_core::get_running_apps();
            let current_ws = get_current_workspace();

            for id in 1..=DEFAULT_WORKSPACE_COUNT {
                let ws_apps = filter_apps_for_workspace(id, &running_apps, current_ws);

                let row_btn = gtk4::Button::new();
                row_btn.add_css_class("workspace-popover-row");
                if id == current_ws {
                    row_btn.add_css_class("active");
                }

                let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
                row_box.set_valign(gtk4::Align::Center);

                let num_badge = gtk4::Label::new(Some(&id.to_string()));
                num_badge.add_css_class("workspace-popover-num");
                row_box.append(&num_badge);

                let ws_name = format!("{} {}", babydra_core::i18n::trans("common.workspace"), id);
                let name_lbl = gtk4::Label::new(Some(&ws_name));
                name_lbl.add_css_class("workspace-popover-name");
                name_lbl.set_hexpand(true);
                name_lbl.set_halign(gtk4::Align::Start);
                row_box.append(&name_lbl);

                if !ws_apps.is_empty() {
                    let count_pill = gtk4::Label::new(Some(&ws_apps.len().to_string()));
                    count_pill.add_css_class("workspace-popover-count");
                    row_box.append(&count_pill);
                }

                if id == current_ws {
                    let active_dot = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
                    active_dot.add_css_class("workspace-popover-active-dot");
                    row_box.append(&active_dot);
                }

                let chevron = babydra_ui_kit::ui::icon::get_icon("pan-end-symbolic", 12);
                chevron.add_css_class("workspace-popover-chevron");
                row_box.append(&chevron);

                row_btn.set_child(Some(&row_box));

                let pop_close = popover_c.clone();
                row_btn.connect_clicked(move |_| {
                    switch_workspace(id);
                    pop_close.popdown();
                });

                // Flyout for apps in this workspace
                let flyout = gtk4::Popover::new();
                flyout.set_parent(&row_btn);
                flyout.set_position(gtk4::PositionType::Right);
                flyout.add_css_class("workspace-apps-flyout");
                flyout.set_autohide(false);

                let flyout_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
                flyout_box.set_size_request(210, -1);
                flyout_box.add_css_class("workspace-flyout-container");

                let flyout_title = gtk4::Label::new(Some(&format!("Workspace {}", id)));
                flyout_title.add_css_class("workspace-flyout-header-title");
                flyout_title.set_halign(gtk4::Align::Start);
                flyout_box.append(&flyout_title);

                let flyout_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
                flyout_sep.add_css_class("workspace-popover-separator");
                flyout_box.append(&flyout_sep);

                if ws_apps.is_empty() {
                    let empty_lbl = gtk4::Label::new(Some("(Trống)"));
                    empty_lbl.add_css_class("workspace-flyout-empty");
                    empty_lbl.set_halign(gtk4::Align::Center);
                    flyout_box.append(&empty_lbl);
                } else {
                    for app in &ws_apps {
                        let app_btn = gtk4::Button::new();
                        app_btn.add_css_class("workspace-app-flyout-item");

                        let app_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
                        app_box.set_valign(gtk4::Align::Center);

                        let app_icon_str =
                            app.icon.as_deref().unwrap_or("application-x-executable");
                        let icon_w = babydra_ui_kit::ui::icon::get_fallback_icon(
                            app_icon_str,
                            "application-x-executable",
                        );
                        icon_w.set_pixel_size(18);
                        app_box.append(&icon_w);

                        let app_name = gtk4::Label::new(None);
                        app_name.add_css_class("workspace-app-flyout-name");
                        app_name.set_ellipsize(gtk4::pango::EllipsizeMode::End);
                        app_name.set_max_width_chars(18);
                        app_name.set_halign(gtk4::Align::Start);
                        let title_text = app
                            .window_title
                            .as_deref()
                            .filter(|t| !t.is_empty())
                            .unwrap_or(&app.name);
                        app_name.set_text(title_text);
                        app_box.append(&app_name);

                        app_btn.set_child(Some(&app_box));

                        let pop_close2 = popover_c.clone();
                        let flyout_close = flyout.clone();
                        let app_id_str = app.app_id.clone().unwrap_or_else(|| app.name.clone());
                        let win_title_str =
                            app.window_title.clone().unwrap_or_else(|| app.name.clone());
                        app_btn.connect_clicked(move |_| {
                            switch_workspace(id);
                            focus_window(&app_id_str, &win_title_str);
                            flyout_close.popdown();
                            pop_close2.popdown();
                        });

                        flyout_box.append(&app_btn);
                    }
                }

                flyout.set_child(Some(&flyout_box));
                stored_flyouts_c.borrow_mut().push(flyout.clone());

                let motion_ctrl = gtk4::EventControllerMotion::new();
                let flyout_show = flyout.clone();
                let active_f = active_flyout_c.clone();
                motion_ctrl.connect_enter(move |_, _, _| {
                    let mut current = active_f.borrow_mut();
                    if let Some(ref prev) = *current {
                        if prev != &flyout_show {
                            prev.popdown();
                        }
                    }
                    flyout_show.popup();
                    *current = Some(flyout_show.clone());
                });
                row_btn.add_controller(motion_ctrl);

                items_box.append(&row_btn);
            }
        })
    };

    let active_fly_close = active_flyout.clone();
    popover.connect_closed(move |_| {
        if let Some(fly) = active_fly_close.borrow_mut().take() {
            fly.popdown();
        }
    });

    let rebuild_trigger = rebuild_popover.clone();
    popover.connect_show(move |_| {
        rebuild_trigger();
    });

    popover
}
