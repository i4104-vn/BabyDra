use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Orientation, Stack, Separator};
use babydra_common::HeaderBarWidgets;
use babydra_common::i18n::t;

/// Builds all header bar items including the navigation buttons, address breadcrumbs stack, search bar, and edit toolbar.
pub fn build_header_bar_ui() -> HeaderBarWidgets {
    let container = Box::new(Orientation::Vertical, 0);

    // ── Row 1: Navigation Bar ──────────────────────────────────
    let nav_row = Box::new(Orientation::Horizontal, 4);
    nav_row.set_css_classes(&["nav-bar"]);
    nav_row.set_margin_start(6);
    nav_row.set_margin_end(6);
    container.append(&nav_row);

    let btn_back = Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("back", 16))
        .build();
    let btn_forward = Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("forward", 16))
        .build();
    let btn_up = Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("up", 16))
        .build();
    let btn_refresh = Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("refresh", 16))
        .build();

    btn_back.set_tooltip_text(Some(&t("explore.back")));
    btn_forward.set_tooltip_text(Some(&t("explore.forward")));
    btn_up.set_tooltip_text(Some(&t("explore.up")));
    btn_refresh.set_tooltip_text(Some(&t("explore.refresh")));

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
        .placeholder_text(&t("explore.search_placeholder"))
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

    let new_folder_box = Box::new(Orientation::Horizontal, 6);
    let new_folder_img = babydra_utils::ui::icon::get_icon("folder-new", 16);
    let new_folder_lbl = gtk4::Label::new(Some(&t("explore.new_folder")));
    new_folder_box.append(&new_folder_img);
    new_folder_box.append(&new_folder_lbl);

    let btn_new_folder = Button::builder()
        .child(&new_folder_box)
        .build();
    let btn_cut = Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("cut", 16))
        .build();
    let btn_copy = Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("copy", 16))
        .build();
    let btn_paste = Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("paste", 16))
        .build();
    let btn_rename = Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("rename", 16))
        .build();
    let btn_delete = Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("trash", 16))
        .build();
    let sep1 = Separator::new(Orientation::Vertical);
    sep1.set_css_classes(&["toolbar-sep"]);
    let sep2 = Separator::new(Orientation::Vertical);
    sep2.set_css_classes(&["toolbar-sep"]);
    let btn_view_icons = Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("view-grid", 16))
        .build();
    let btn_view_list = Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("view-list", 16))
        .build();
    let btn_settings = Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("settings", 16))
        .build();

    btn_new_folder.set_tooltip_text(Some(&t("explore.new_folder")));
    btn_cut.set_tooltip_text(Some(&t("explore.cut")));
    btn_copy.set_tooltip_text(Some(&t("explore.copy")));
    btn_paste.set_tooltip_text(Some(&t("explore.paste")));
    btn_rename.set_tooltip_text(Some(&t("explore.rename")));
    btn_delete.set_tooltip_text(Some(&t("explore.delete")));
    btn_view_icons.set_tooltip_text(Some(&t("explore.view_grid")));
    btn_view_list.set_tooltip_text(Some(&t("explore.view_list")));
    btn_settings.set_tooltip_text(Some(&t("explore.settings")));

    btn_new_folder.set_css_classes(&["toolbar-btn", "new-btn"]);

    for btn in &[&btn_cut, &btn_copy, &btn_paste, &btn_rename, &btn_delete,
                 &btn_view_icons, &btn_view_list, &btn_settings] {
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

    // DropDown for sorting (Auto, Theo ngày, Theo group)
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
