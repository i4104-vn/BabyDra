use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Orientation, Stack, Separator};
use babydra_common::HeaderBarWidgets;

/// Builds all header bar items including the navigation buttons, address breadcrumbs stack, search bar, and edit toolbar.
pub fn build_header_bar_ui() -> HeaderBarWidgets {
    let container = Box::new(Orientation::Vertical, 0);

    // ── Row 1: Navigation Bar ──────────────────────────────────
    let nav_row = Box::new(Orientation::Horizontal, 4);
    nav_row.set_css_classes(&["nav-bar"]);
    nav_row.set_margin_start(6);
    nav_row.set_margin_end(6);
    container.append(&nav_row);

    let btn_back    = Button::from_icon_name("go-previous-symbolic");
    let btn_forward = Button::from_icon_name("go-next-symbolic");
    let btn_up      = Button::from_icon_name("go-up-symbolic");
    let btn_refresh = Button::from_icon_name("view-refresh-symbolic");

    for btn in &[&btn_back, &btn_forward, &btn_up, &btn_refresh] {
        btn.set_css_classes(&["nav-btn"]);
    }

    nav_row.append(&btn_back);
    nav_row.append(&btn_forward);
    nav_row.append(&btn_up);
    nav_row.append(&btn_refresh);

    // Address bar wrapper
    let address_wrap = Box::new(Orientation::Horizontal, 0);
    address_wrap.set_css_classes(&["address-bar-wrap"]);
    address_wrap.set_hexpand(true);
    address_wrap.set_valign(gtk4::Align::Center);

    let address_stack = Stack::new();
    address_stack.set_hexpand(true);

    let breadcrumb_box = Box::new(Orientation::Horizontal, 2);
    breadcrumb_box.set_valign(gtk4::Align::Center);
    address_stack.add_named(&breadcrumb_box, Some("breadcrumbs"));

    let entry_address = Entry::new();
    entry_address.set_hexpand(true);
    entry_address.set_css_classes(&["address-entry"]);
    address_stack.add_named(&entry_address, Some("address"));

    address_wrap.append(&address_stack);
    nav_row.append(&address_wrap);

    // Search entry
    let search = Entry::builder()
        .placeholder_text("Search")
        .primary_icon_name("system-search-symbolic")
        .css_classes(vec!["search-entry".to_string()])
        .build();
    search.set_size_request(200, -1);
    nav_row.append(&search);

    // ── Row 2: Command Toolbar ─────────────────────────────────
    let toolbar = Box::new(Orientation::Horizontal, 2);
    toolbar.set_css_classes(&["toolbar"]);
    toolbar.set_margin_start(6);
    toolbar.set_margin_end(6);
    container.append(&toolbar);

    let btn_new_folder   = Button::with_label("⊕ New Folder");
    let btn_cut          = Button::from_icon_name("edit-cut-symbolic");
    let btn_copy         = Button::from_icon_name("edit-copy-symbolic");
    let btn_paste        = Button::from_icon_name("edit-paste-symbolic");
    let btn_rename       = Button::from_icon_name("edit-rename-symbolic");
    let btn_delete       = Button::from_icon_name("edit-delete-symbolic");
    let sep1 = Separator::new(Orientation::Vertical);
    sep1.set_css_classes(&["toolbar-sep"]);
    let sep2 = Separator::new(Orientation::Vertical);
    sep2.set_css_classes(&["toolbar-sep"]);
    let btn_view_icons   = Button::from_icon_name("view-grid-symbolic");
    let btn_view_list    = Button::from_icon_name("view-list-symbolic");

    btn_new_folder.set_css_classes(&["toolbar-btn", "new-btn"]);
    for btn in &[&btn_cut, &btn_copy, &btn_paste, &btn_rename, &btn_delete,
                 &btn_view_icons, &btn_view_list] {
        btn.set_css_classes(&["toolbar-btn"]);
    }

    toolbar.append(&btn_new_folder);
    toolbar.append(&sep1);
    toolbar.append(&btn_cut);
    toolbar.append(&btn_copy);
    toolbar.append(&btn_paste);
    toolbar.append(&sep2);
    toolbar.append(&btn_rename);
    toolbar.append(&btn_delete);

    // push view toggle to the right
    let spacer = Box::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    toolbar.append(&spacer);
    toolbar.append(&btn_view_icons);
    toolbar.append(&btn_view_list);

    HeaderBarWidgets {
        container,
        btn_back,
        btn_forward,
        btn_up,
        btn_refresh,
        breadcrumb_box,
        entry_address,
        address_stack,
        address_wrap,
        search,
        btn_view_icons,
        btn_view_list,
        btn_new_folder,
        btn_cut,
        btn_copy,
        btn_paste,
        btn_rename,
        btn_delete,
    }
}
