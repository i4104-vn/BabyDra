use gtk4::prelude::*;

/// Builds the `tray container` UI.
pub fn build_tray_container() -> gtk4::Box {
    let tray_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    tray_container.add_css_class("panel-tray-box");
    tray_container.set_valign(gtk4::Align::Center);
    tray_container.set_halign(gtk4::Align::Center);
    tray_container
}

/// Builds the `tray button` UI.
pub fn build_tray_button(icon_name: &str, title: &str) -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("panel-tray-item-btn");
    btn.set_tooltip_text(Some(title));
    btn.set_valign(gtk4::Align::Center);
    btn.set_halign(gtk4::Align::Center);
    btn.set_receives_default(false);

    let icon = babydra_ui_kit::ui::icon::get_system_or_file_icon(icon_name, "image-missing");
    icon.set_pixel_size(16);
    btn.set_child(Some(&icon));

    btn
}

thread_local! {
    static ACTIVE_POPOVER: std::cell::RefCell<Option<gtk4::Popover>> = std::cell::RefCell::new(None);
}

/// Shows the `context menu`.
pub fn show_context_menu(
    btn: &gtk4::Button,
    service: &str,
    items: &[babydra_core::tray::MenuItem],
) {
    ACTIVE_POPOVER.with(|p| {
        if let Some(old_popover) = p.borrow_mut().take() {
            old_popover.popdown();
            old_popover.unparent();
        }
    });

    let action_group = gtk4::gio::SimpleActionGroup::new();
    let menu_model = build_gio_menu(items, &action_group, service);

    let popover = gtk4::PopoverMenu::from_model(Some(&menu_model));
    popover.set_parent(btn);
    popover.set_position(gtk4::PositionType::Bottom);
    popover.set_has_arrow(true);
    popover.add_css_class("tray-context-menu");

    btn.insert_action_group("tray", Some(&action_group));

    popover.popup();

    ACTIVE_POPOVER.with(|p| {
        *p.borrow_mut() = Some(popover.upcast::<gtk4::Popover>());
    });
}

/// Builds the `gio menu` UI.
fn build_gio_menu(
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
            let submenu = build_gio_menu(&item.children, action_group, service);
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
