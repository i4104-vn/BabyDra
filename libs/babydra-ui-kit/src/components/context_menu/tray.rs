//! Context menu integration for System Tray icons and DBus menu specifications.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static ACTIVE_TRAY_POPOVER: RefCell<Option<gtk4::Popover>> = RefCell::new(None);
}

/// Closes and unparents the currently active tray context menu popover if one exists.
pub fn close_active_tray_menu() {
    ACTIVE_TRAY_POPOVER.with(|p| {
        if let Some(old_popover) = p.borrow_mut().take() {
            old_popover.popdown();
            old_popover.unparent();
        }
    });
}

/// Displays a context menu for a system tray item based on DBus menu items using hierarchical Popovers.
pub fn show_tray_context_menu(
    btn: &gtk4::Button,
    service: &str,
    items: &[babydra_core::tray::MenuItem],
) {
    close_active_tray_menu();

    let popover = gtk4::Popover::builder()
        .has_arrow(true)
        .autohide(true)
        .position(gtk4::PositionType::Bottom)
        .build();

    popover.set_parent(btn);
    popover.add_css_class("context-menu-popover");
    popover.add_css_class("tray-context-menu");

    let active_sub_popover = Rc::new(RefCell::new(None::<gtk4::Popover>));
    let vbox = build_tray_menu_box(items, service, &popover, active_sub_popover);
    popover.set_child(Some(&vbox));

    popover.popup();

    ACTIVE_TRAY_POPOVER.with(|p| {
        *p.borrow_mut() = Some(popover);
    });
}

/// Recursively builds the vertical Box of menu items and attached submenus (menu cấp 2+).
pub fn build_tray_menu_box(
    items: &[babydra_core::tray::MenuItem],
    service: &str,
    root_popover: &gtk4::Popover,
    active_sub_popover: Rc<RefCell<Option<gtk4::Popover>>>,
) -> gtk4::Box {
    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    vbox.set_css_classes(&["context-menu-box"]);
    vbox.set_width_request(190);

    for item in items {
        if !item.visible {
            continue;
        }
        if item.is_separator {
            let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
            sep.add_css_class("context-menu-separator");
            vbox.append(&sep);
            continue;
        }

        let label_text = if item.label.is_empty() {
            "Item".to_string()
        } else {
            item.label.replace('_', "")
        };

        let is_destructive = label_text.to_lowercase().contains("quit")
            || label_text.to_lowercase().contains("exit");

        let is_checked = item.toggle_state == Some(1);

        if !item.children.is_empty() {
            // Item with Submenu (Menu cấp 2) -> Arrow on the right
            let btn = create_tray_submenu_button(&label_text, item.enabled);

            let sub_popover = gtk4::Popover::builder()
                .has_arrow(true)
                .autohide(false)
                .position(gtk4::PositionType::Right)
                .build();
            sub_popover.set_parent(&btn);
            sub_popover.add_css_class("context-menu-popover");
            sub_popover.add_css_class("tray-context-menu");

            let child_active_sub = Rc::new(RefCell::new(None::<gtk4::Popover>));
            let sub_box = build_tray_menu_box(&item.children, service, root_popover, child_active_sub);
            sub_popover.set_child(Some(&sub_box));

            // Instant submenu replacement on click or hover
            let active_sub_c = active_sub_popover.clone();
            let sub_pop_c = sub_popover.clone();
            let open_sub = move || {
                let current_sub = active_sub_c.borrow().clone();
                if let Some(old_pop) = current_sub {
                    if old_pop != sub_pop_c {
                        old_pop.popdown();
                        *active_sub_c.borrow_mut() = None;
                    }
                }
                sub_pop_c.popup();
                *active_sub_c.borrow_mut() = Some(sub_pop_c.clone());
            };

            let open_sub_c1 = open_sub.clone();
            btn.connect_clicked(move |_| {
                open_sub_c1();
            });

            let open_sub_c2 = open_sub.clone();
            let motion = gtk4::EventControllerMotion::new();
            motion.connect_enter(move |_, _, _| {
                open_sub_c2();
            });
            btn.add_controller(motion);

            vbox.append(&btn);
        } else {
            // Leaf item -> No leading icon, clean text, optional checkmark
            let s_name = service.to_string();
            let item_id = item.id;
            let root_pop_c = root_popover.clone();
            let active_sub_c = active_sub_popover.clone();

            let btn = create_tray_leaf_button(&label_text, is_checked, is_destructive, item.enabled);

            let active_sub_leaf = active_sub_popover.clone();
            let motion = gtk4::EventControllerMotion::new();
            motion.connect_enter(move |_, _, _| {
                if let Some(old_pop) = active_sub_leaf.borrow_mut().take() {
                    old_pop.popdown();
                }
            });
            btn.add_controller(motion);

            btn.connect_clicked(move |_| {
                if let Some(old_pop) = active_sub_c.borrow_mut().take() {
                    old_pop.popdown();
                }
                root_pop_c.popdown();
                close_active_tray_menu();
                babydra_core::tray::activate_menu_item(&s_name, item_id);
            });

            vbox.append(&btn);
        }
    }

    vbox
}

/// Creates a leaf menu item button (no leading icon, label on left, optional checkmark on left/right).
fn create_tray_leaf_button(
    label: &str,
    is_checked: bool,
    is_destructive: bool,
    sensitive: bool,
) -> gtk4::Button {
    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    hbox.set_halign(gtk4::Align::Fill);

    if is_checked {
        let check_icon = crate::ui::icon::get_icon("check", 14);
        check_icon.set_pixel_size(14);
        check_icon.set_valign(gtk4::Align::Center);
        hbox.append(&check_icon);
    }

    let lbl = gtk4::Label::builder()
        .label(label)
        .halign(gtk4::Align::Start)
        .hexpand(true)
        .valign(gtk4::Align::Center)
        .build();
    hbox.append(&lbl);

    let mut css_classes = vec![
        "flat".to_string(),
        "context-menu-item".to_string(),
    ];
    if is_destructive {
        css_classes.push("destructive".to_string());
    }

    let btn = gtk4::Button::builder()
        .child(&hbox)
        .css_classes(css_classes)
        .halign(gtk4::Align::Fill)
        .focusable(false)
        .sensitive(sensitive)
        .build();

    btn.set_cursor_from_name(Some("pointer"));
    btn
}

/// Creates a submenu menu item button (label on left, forward arrow icon on right).
fn create_tray_submenu_button(label: &str, sensitive: bool) -> gtk4::Button {
    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    hbox.set_halign(gtk4::Align::Fill);

    let lbl = gtk4::Label::builder()
        .label(label)
        .halign(gtk4::Align::Start)
        .hexpand(true)
        .valign(gtk4::Align::Center)
        .build();
    hbox.append(&lbl);

    let arrow_icon = crate::ui::icon::get_icon("forward", 12);
    arrow_icon.set_pixel_size(12);
    arrow_icon.set_halign(gtk4::Align::End);
    arrow_icon.set_valign(gtk4::Align::Center);
    hbox.append(&arrow_icon);

    let btn = gtk4::Button::builder()
        .child(&hbox)
        .css_classes(vec![
            "flat".to_string(),
            "context-menu-item".to_string(),
        ])
        .halign(gtk4::Align::Fill)
        .focusable(false)
        .sensitive(sensitive)
        .build();

    btn.set_cursor_from_name(Some("pointer"));
    btn
}

/// Builds a hierarchical GIO Menu model from DBus MenuItem descriptors (utility helper).
pub fn build_tray_gio_menu(
    items: &[babydra_core::tray::MenuItem],
    action_group: &gtk4::gio::SimpleActionGroup,
    service: &str,
) -> gtk4::gio::Menu {
    let menu = gtk4::gio::Menu::new();

    for item in items {
        if item.is_separator {
            continue;
        }

        let action_name = format!("a{}", item.id);
        let detailed_action = format!("tray.{}", action_name);

        let menu_item = gtk4::gio::MenuItem::new(Some(&item.label), Some(&detailed_action));

        if !item.children.is_empty() {
            let submenu = build_tray_gio_menu(&item.children, action_group, service);
            menu_item.set_submenu(Some(&submenu));
        }

        let s_name = service.to_string();
        let item_id = item.id;
        let action = gtk4::gio::SimpleAction::new(&action_name, None);
        action.set_enabled(item.enabled);

        action.connect_activate(move |_, _| {
            babydra_core::tray::activate_menu_item(&s_name, item_id);
        });

        action_group.add_action(&action);

        if item.visible {
            menu.append_item(&menu_item);
        }
    }

    menu
}
