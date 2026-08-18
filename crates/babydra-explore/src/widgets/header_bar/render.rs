use crate::widgets::state::HeaderBarWidgets;
use babydra_core::i18n::t;
use gtk4::prelude::*;
use gtk4::{Box, Entry, Orientation, Separator, Stack};

/// Builds all header bar items including the navigation buttons, address breadcrumbs stack, search bar, and edit toolbar.
pub fn build_header_bar_ui() -> HeaderBarWidgets {
    let container = Box::new(Orientation::Vertical, 0);

    // ── Dummy Navigation Bar (Created but not appended to global container) ──
    let btn_back = babydra_ui_kit::components::create_icon_button(
        "back",
        16,
        &["nav-btn"],
        Some(&t("explore.back")),
        || {},
    );
    let btn_forward = babydra_ui_kit::components::create_icon_button(
        "forward",
        16,
        &["nav-btn"],
        Some(&t("explore.forward")),
        || {},
    );
    let btn_up = babydra_ui_kit::components::create_icon_button(
        "up",
        16,
        &["nav-btn"],
        Some(&t("explore.up")),
        || {},
    );
    let btn_refresh = babydra_ui_kit::components::create_icon_button(
        "refresh",
        16,
        &["nav-btn"],
        Some(&t("explore.refresh")),
        || {},
    );

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

    // Search entry
    let search = Entry::builder()
        .placeholder_text(&t("explore.search_placeholder"))
        .primary_icon_name("system-search-symbolic")
        .css_classes(vec!["search-entry".to_string()])
        .build();
    search.set_size_request(200, -1);

    // ── Row 2: Command Toolbar ─────────────────────────────────
    let toolbar = Box::new(Orientation::Horizontal, 2);
    toolbar.set_css_classes(&["toolbar"]);
    toolbar.set_margin_start(6);
    toolbar.set_margin_end(6);
    container.append(&toolbar);

    let btn_new_folder = babydra_ui_kit::components::create_icon_label_button(
        "folder-new",
        &t("explore.new_folder"),
        "toolbar-btn",
    );
    btn_new_folder.add_css_class("new-btn");
    btn_new_folder.set_tooltip_text(Some(&t("explore.new_folder")));

    let btn_cut = babydra_ui_kit::components::create_icon_button(
        "cut",
        16,
        &["toolbar-btn"],
        Some(&t("explore.cut")),
        || {},
    );
    let btn_copy = babydra_ui_kit::components::create_icon_button(
        "copy",
        16,
        &["toolbar-btn"],
        Some(&t("explore.copy")),
        || {},
    );
    let btn_paste = babydra_ui_kit::components::create_icon_button(
        "paste",
        16,
        &["toolbar-btn"],
        Some(&t("explore.paste")),
        || {},
    );
    let btn_rename = babydra_ui_kit::components::create_icon_button(
        "rename",
        16,
        &["toolbar-btn"],
        Some(&t("explore.rename")),
        || {},
    );
    let btn_delete = babydra_ui_kit::components::create_icon_button(
        "trash",
        16,
        &["toolbar-btn"],
        Some(&t("explore.delete")),
        || {},
    );
    let sep1 = Separator::new(Orientation::Vertical);
    sep1.set_css_classes(&["toolbar-sep"]);
    let sep2 = Separator::new(Orientation::Vertical);
    sep2.set_css_classes(&["toolbar-sep"]);
    let btn_view_icons = babydra_ui_kit::components::create_icon_button(
        "view-grid",
        16,
        &["toolbar-btn"],
        Some(&t("explore.view_grid")),
        || {},
    );
    let btn_view_list = babydra_ui_kit::components::create_icon_button(
        "view-list",
        16,
        &["toolbar-btn"],
        Some(&t("explore.view_list")),
        || {},
    );
    let btn_settings = babydra_ui_kit::components::create_icon_button(
        "settings",
        16,
        &["toolbar-btn"],
        Some(&t("explore.settings")),
        || {},
    );

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

    // DropDown for sorting (Auto, By date, By group)
    let sort_options = [
        t("explore.sort_auto"),
        t("explore.sort_date"),
        t("explore.sort_group"),
    ];
    let sort_options_strs: Vec<&str> = sort_options.iter().map(|s| s.as_str()).collect();
    let dropdown_sort = gtk4::DropDown::from_strings(&sort_options_strs);
    dropdown_sort.set_css_classes(&["toolbar-dropdown"]);
    dropdown_sort.set_tooltip_text(Some(&t("explore.sort_by")));
    toolbar.append(&dropdown_sort);

    toolbar.append(&btn_view_icons);
    toolbar.append(&btn_view_list);

    let sep3 = Separator::new(Orientation::Vertical);
    sep3.set_css_classes(&["toolbar-sep"]);
    toolbar.append(&sep3);
    toolbar.append(&btn_settings);

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
        dropdown_sort,
        btn_new_folder,
        btn_cut,
        btn_copy,
        btn_paste,
        btn_rename,
        btn_delete,
        btn_settings,
    }
}
