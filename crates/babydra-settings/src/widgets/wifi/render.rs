//! Wi-Fi UI layout generator with flat list and category sections matching Apps layout.

use babydra_ui_kit::components::modals::{WifiConfigDialog, WifiInfoDialog, WifiPasswordDialog};
use babydra_ui_kit::components::ToggleRow;
use gtk4::prelude::*;

pub struct WifiWidgetUi {
    pub root: gtk4::Widget,
    pub toggle_row: ToggleRow,
    pub search_entry: gtk4::Entry,
    pub refresh_btn: gtk4::Button,
    pub list_box: gtk4::ListBox,
    pub info_dialog: WifiInfoDialog,
    pub password_dialog: WifiPasswordDialog,
    pub config_dialog: WifiConfigDialog,
}

/// Builds the Wi-Fi settings page UI.
pub fn build_wifi_ui() -> WifiWidgetUi {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_vexpand(true);
    main_box.set_valign(gtk4::Align::Fill);

    // Header Row: "Wi-Fi" Title on Left, Search + Refresh + Toggle Switch on Right
    let header_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    header_row.set_margin_bottom(4);

    let page_title = gtk4::Label::new(Some(&babydra_core::i18n::trans("settings.wifi_title")));
    page_title.add_css_class("settings-page-title");
    page_title.set_halign(gtk4::Align::Start);
    page_title.set_hexpand(true);
    header_row.append(&page_title);

    let search_entry = gtk4::Entry::new();
    search_entry.set_placeholder_text(Some(&babydra_core::i18n::trans(
        "settings.wifi_search_placeholder",
    )));
    search_entry.add_css_class("sidebar-search-entry");
    search_entry.set_width_request(200);
    header_row.append(&search_entry);

    let refresh_btn = gtk4::Button::new();
    refresh_btn.add_css_class("icon-btn");
    refresh_btn.add_css_class("circular");
    refresh_btn.set_cursor_from_name(Some("pointer"));
    refresh_btn.set_valign(gtk4::Align::Center);
    refresh_btn.set_size_request(34, 34);
    let refresh_icon = babydra_ui_kit::ui::icon::get_icon("refresh", 16);
    refresh_icon.set_pixel_size(16);
    refresh_btn.set_child(Some(&refresh_icon));
    refresh_btn.set_tooltip_text(Some("Scan for Wi-Fi networks"));
    header_row.append(&refresh_btn);

    let toggle_row = ToggleRow::new(false);
    header_row.append(&toggle_row.container);
    main_box.append(&header_row);

    // Glass Panel Container with Single Flat ListBox (matching Apps list)
    let glass_card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");
    glass_card.set_vexpand(true);
    glass_card.set_valign(gtk4::Align::Fill);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);

    let list_box = gtk4::ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);

    scroll.set_child(Some(&list_box));
    glass_card.append(&scroll);
    main_box.append(&glass_card);

    // Modals
    let info_dialog = WifiInfoDialog::new();
    let password_dialog = WifiPasswordDialog::new();
    let config_dialog = WifiConfigDialog::new();

    // Attach modals to overlay so they can be displayed when triggered
    let overlay = gtk4::Overlay::new();
    overlay.set_child(Some(&main_box));
    overlay.add_overlay(&info_dialog.container);
    overlay.add_overlay(&password_dialog.container);
    overlay.add_overlay(&config_dialog.container);

    WifiWidgetUi {
        root: overlay.into(),
        toggle_row,
        search_entry,
        refresh_btn,
        list_box,
        info_dialog,
        password_dialog,
        config_dialog,
    }
}
