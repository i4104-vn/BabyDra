//! Interactive notifications history manager.
//! Manages loading, grouped rendering (by app), and expanding/collapsing of notifications,
//! as well as formatting timestamps and clearing history.

use super::notification_group::format_elapsed_time;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

/// Configures and manages the interactive historical notifications list stack.
/// Sets up periodic timers to update clock time, date, and detects when new notifications arrive.
pub fn setup_notifs_list(
    notif_stack: &gtk4::Box,
    clear_btn: &gtk4::Button,
    big_time: &gtk4::Label,
    big_date: &gtk4::Label,
) {
    let expanded_apps = Rc::new(RefCell::new(HashSet::<String>::new()));
    let render_notifications_holder: Rc<RefCell<Option<Rc<dyn Fn()>>>> =
        Rc::new(RefCell::new(None));

    let render_notifications = {
        let notif_stack = notif_stack.clone();
        let expanded_apps = expanded_apps.clone();
        let holder = render_notifications_holder.clone();
        move || {
            let render_notifications_rc = holder.borrow().as_ref().unwrap().clone();

            notif_stack.set_opacity(1.0);
            notif_stack.set_margin_top(0);
            notif_stack.set_margin_bottom(0);
            notif_stack.set_margin_start(0);
            notif_stack.set_margin_end(0);

            while let Some(child) = notif_stack.first_child() {
                notif_stack.remove(&child);
            }

            let notifications: Vec<_> =
                babydra_island::widgets::notification::HISTORICAL_NOTIFICATIONS
                    .with(|list| list.borrow().iter().cloned().collect());

            if notifications.is_empty() {
                let empty_label =
                    gtk4::Label::new(Some(&babydra_core::i18n::trans("panel.no_notifications")));
                empty_label.add_css_class("notif-empty-label");
                empty_label.set_halign(gtk4::Align::Center);
                empty_label.set_valign(gtk4::Align::Center);
                empty_label.set_vexpand(true);
                notif_stack.append(&empty_label);
            } else {
                let (grouped, app_order) =
                    super::notification_group::group_notifs_by_app(&notifications);

                for app_key in app_order {
                    let list = &grouped[&app_key];
                    let display_app_name = if app_key == "system" {
                        babydra_core::i18n::trans("panel.system")
                    } else {
                        let mut chars = app_key.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                        }
                    };

                    let is_expanded = expanded_apps.borrow().contains(&app_key);

                    let group_container = if is_expanded {
                        render_expanded_group(
                            &app_key,
                            &display_app_name,
                            list,
                            expanded_apps.clone(),
                            render_notifications_rc.clone(),
                        )
                    } else {
                        render_collapsed_group(
                            &app_key,
                            &display_app_name,
                            list,
                            expanded_apps.clone(),
                            render_notifications_rc.clone(),
                        )
                    };

                    notif_stack.append(&group_container);
                }
            }
        }
    };

    let render_rc = Rc::new(render_notifications);
    *render_notifications_holder.borrow_mut() = Some(render_rc.clone());

    render_rc();

    let clear_btn_render_clone = render_rc.clone();
    let notif_stack_clear_clone = notif_stack.clone();
    clear_btn.connect_clicked(move |_| {
        let callback = clear_btn_render_clone.clone();
        babydra_island::widgets::notification::SHARED_NOTIFICATION.with(|sn| {
            *sn.borrow_mut() = None;
        });
        babydra_island::widgets::notification::HISTORICAL_NOTIFICATIONS.with(|list| {
            list.borrow_mut().clear();
        });
        babydra_ui_kit::ui::animation::slide_out_cb(
            notif_stack_clear_clone.upcast_ref(),
            babydra_ui_kit::ui::animation::SlideDirection::Up,
            20,
            450,
            false,
            move || {
                callback();
            },
        );
    });

    let last_notif_count = Rc::new(std::cell::Cell::new(
        babydra_island::widgets::notification::HISTORICAL_NOTIFICATIONS
            .with(|list| list.borrow().len()),
    ));

    let bt_clone = big_time.clone();
    let bd_clone = big_date.clone();
    let render_timer_clone = render_rc.clone();
    let last_count_clone = last_notif_count.clone();
    let update_header = move || {
        let current_now = chrono::Local::now();
        bt_clone.set_text(&current_now.format("%I:%M %p").to_string());

        let weekday_key = format!(
            "weekday.{}",
            current_now.format("%a").to_string().to_lowercase()
        );
        let weekday = babydra_core::i18n::trans(&weekday_key);
        let month_key = format!("month.{}", current_now.format("%m").to_string());
        let month_str = babydra_core::i18n::trans(&month_key);

        let date_str = babydra_core::i18n::trans("panel.date_format")
            .replace("{weekday}", &weekday)
            .replace("{day}", &current_now.format("%d").to_string())
            .replace("{month}", &month_str);
        bd_clone.set_text(&date_str);

        let current_count = babydra_island::widgets::notification::HISTORICAL_NOTIFICATIONS
            .with(|list| list.borrow().len());
        if current_count != last_count_clone.get() {
            last_count_clone.set(current_count);
            render_timer_clone();
        }

        glib::ControlFlow::Continue
    };
    glib::timeout_add_local(std::time::Duration::from_millis(500), update_header);
}

/// Renders the expanded group layout displaying all historical notifications grouped
/// under the specific application name with slide animation transitions.
fn render_expanded_group(
    app_key: &str,
    display_app_name: &str,
    list: &[babydra_island::models::ActiveNotification],
    expanded_apps: Rc<RefCell<HashSet<String>>>,
    render_notifications_rc: Rc<dyn Fn()>,
) -> gtk4::Box {
    let group_container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    group_container.add_css_class("notif-group-container");

    let group_header = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    group_header.add_css_class("notif-group-header");

    let latest_notif = list.last().unwrap();
    let icon_widget = make_notif_icon(&latest_notif.icon);
    icon_widget.set_pixel_size(18);
    icon_widget.set_valign(gtk4::Align::Center);
    icon_widget.set_halign(gtk4::Align::Center);
    icon_widget.add_css_class("notif-item-icon");

    let title_lbl = gtk4::Label::new(Some(display_app_name));
    title_lbl.add_css_class("notif-item-title");
    title_lbl.set_halign(gtk4::Align::Start);
    title_lbl.set_hexpand(true);

    let chevron = babydra_ui_kit::ui::icon::get_icon("up", 12);
    chevron.set_pixel_size(12);
    chevron.set_opacity(0.4);
    chevron.set_valign(gtk4::Align::Center);

    group_header.append(&icon_widget);
    group_header.append(&title_lbl);
    group_header.append(&chevron);
    group_container.append(&group_header);

    let sub_box = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    sub_box.add_css_class("notif-sub-box");

    for notif in list.iter().rev() {
        let item_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
        item_box.add_css_class("notif-stack-item");

        let icon_widget = make_notif_icon(&notif.icon);
        icon_widget.set_pixel_size(18);
        icon_widget.set_valign(gtk4::Align::Center);
        icon_widget.set_halign(gtk4::Align::Center);
        icon_widget.add_css_class("notif-item-icon");

        let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
        text_box.set_hexpand(true);

        let title_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        title_box.set_hexpand(true);

        let title_lbl = gtk4::Label::new(Some(&notif.title));
        title_lbl.add_css_class("notif-item-title");
        title_lbl.set_halign(gtk4::Align::Start);
        title_lbl.set_hexpand(true);
        title_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        title_lbl.set_lines(1);

        let time_str = format_elapsed_time(notif.timestamp);
        let time_lbl = gtk4::Label::new(Some(&time_str));
        time_lbl.add_css_class("notif-item-sub-time");
        time_lbl.set_halign(gtk4::Align::End);
        time_lbl.set_valign(gtk4::Align::Center);

        title_box.append(&title_lbl);
        title_box.append(&time_lbl);

        let body_lbl = gtk4::Label::new(Some(&notif.body));
        body_lbl.add_css_class("notif-item-body");
        body_lbl.set_halign(gtk4::Align::Start);
        body_lbl.set_hexpand(true);
        body_lbl.set_wrap(true);
        body_lbl.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
        body_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        body_lbl.set_lines(3);

        text_box.append(&title_box);
        text_box.append(&body_lbl);

        item_box.append(&icon_widget);
        item_box.append(&text_box);
        sub_box.append(&item_box);
    }

    group_container.append(&sub_box);

    let click_gesture = gtk4::GestureClick::new();
    let ea_c = expanded_apps.clone();
    let ak_c = app_key.to_string();
    let render_c = render_notifications_rc.clone();
    let sub_box_c = sub_box.clone();
    click_gesture.connect_pressed(move |_, _, _, _| {
        let ea_cb = ea_c.clone();
        let ak_cb = ak_c.clone();
        let render_cb = render_c.clone();
        babydra_ui_kit::ui::animation::slide_out_cb(
            sub_box_c.upcast_ref(),
            babydra_ui_kit::ui::animation::SlideDirection::Up,
            15,
            400,
            false,
            move || {
                ea_cb.borrow_mut().remove(&ak_cb);
                render_cb();
            },
        );
    });
    group_header.add_controller(click_gesture);

    babydra_ui_kit::ui::animation::slide_in(
        sub_box.upcast_ref(),
        babydra_ui_kit::ui::animation::SlideDirection::Down,
        15,
        450,
    );

    group_container
}

/// Renders the collapsed group layout displaying only the latest notification from an application,
/// and draws a 3D visual stack layer if the application has multiple unread notifications.
fn render_collapsed_group(
    app_key: &str,
    display_app_name: &str,
    list: &[babydra_island::models::ActiveNotification],
    expanded_apps: Rc<RefCell<HashSet<String>>>,
    render_notifications_rc: Rc<dyn Fn()>,
) -> gtk4::Box {
    let group_container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    group_container.add_css_class("notif-group-container");

    let latest_notif = list.last().unwrap();

    let main_item = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    main_item.add_css_class("notif-stack-item");

    let icon_widget = make_notif_icon(&latest_notif.icon);
    icon_widget.set_pixel_size(18);
    icon_widget.set_valign(gtk4::Align::Center);
    icon_widget.set_halign(gtk4::Align::Center);
    icon_widget.add_css_class("notif-item-icon");

    let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    text_box.set_hexpand(true);

    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    header_box.set_hexpand(true);
    let app_title = gtk4::Label::new(Some(display_app_name));
    app_title.add_css_class("notif-item-title");
    app_title.set_halign(gtk4::Align::Start);
    app_title.set_hexpand(true);
    app_title.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    app_title.set_lines(1);
    header_box.append(&app_title);

    let right_widget = if list.len() > 1 {
        let badge = gtk4::Label::new(Some(&format!("{}", list.len())));
        badge.add_css_class("notif-count-badge");
        badge.add_css_class("notif-item-sub-time");
        badge.set_halign(gtk4::Align::End);
        badge.set_valign(gtk4::Align::Center);
        badge.upcast::<gtk4::Widget>()
    } else {
        let time_str = format_elapsed_time(latest_notif.timestamp);
        let time_lbl = gtk4::Label::new(Some(&time_str));
        time_lbl.add_css_class("notif-item-sub-time");
        time_lbl.set_halign(gtk4::Align::End);
        time_lbl.set_valign(gtk4::Align::Center);
        time_lbl.upcast::<gtk4::Widget>()
    };
    header_box.append(&right_widget);

    let body_lbl = gtk4::Label::new(Some(&latest_notif.body));
    body_lbl.add_css_class("notif-item-body");
    body_lbl.set_halign(gtk4::Align::Start);
    body_lbl.set_hexpand(true);
    body_lbl.set_wrap(true);
    body_lbl.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
    body_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    body_lbl.set_lines(3);

    text_box.append(&header_box);
    text_box.append(&body_lbl);

    main_item.append(&icon_widget);
    main_item.append(&text_box);
    group_container.append(&main_item);

    if list.len() > 1 {
        let layer1 = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        layer1.add_css_class("notif-stack-item-layered-1");
        let layer2 = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        layer2.add_css_class("notif-stack-item-layered-2");
        group_container.append(&layer2);
        group_container.append(&layer1);
    }

    let click_gesture = gtk4::GestureClick::new();
    let ea_c = expanded_apps.clone();
    let ak_c = app_key.to_string();
    let render_c = render_notifications_rc.clone();
    click_gesture.connect_pressed(move |_, _, _, _| {
        ea_c.borrow_mut().insert(ak_c.clone());
        render_c();
    });
    group_container.add_controller(click_gesture);

    group_container
}

/// Make notif icon.
fn make_notif_icon(icon: &str) -> gtk4::Image {
    let size = 18;
    // Absolute path – use directly if file exists
    if icon.starts_with('/') && std::path::Path::new(icon).exists() {
        let img = gtk4::Image::new();
        img.set_from_file(Some(icon));
        img.set_pixel_size(size);
        return img;
    }
    // Named icon – check if it exists in the current icon theme
    if !icon.is_empty() && icon != "babydra" {
        if let Some(display) = gdk4::Display::default() {
            let theme = gtk4::IconTheme::for_display(&display);
            // Strip extension if present
            let clean = icon
                .trim_end_matches(|c| c == '.')
                .rsplitn(2, '.')
                .last()
                .unwrap_or(icon);
            if theme.has_icon(clean) {
                let img = gtk4::Image::from_icon_name(clean);
                img.set_pixel_size(size);
                return img;
            }
        }
    }
    // Fallback: embedded logo
    babydra_ui_kit::ui::icon::get_logo_png(size)
}
