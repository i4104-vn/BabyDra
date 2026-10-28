//! VPN UI layout generator with smart card grid layout.

use babydra_ui_kit::components::modals::{VpnConfigDialog, VpnLogDialog};
use crate::widgets::helpers::{create_placeholder_box, PlaceholderState};
use gtk4::prelude::*;

/// Builds the VPN settings page UI with a responsive card grid.
pub fn build_vpn_ui() -> (
    gtk4::Box,
    gtk4::Button,
    crate::widgets::helpers::ResponsiveCardGrid,
    gtk4::Stack,
    VpnConfigDialog,
    VpnLogDialog,
) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_vexpand(true);
    main_box.set_valign(gtk4::Align::Fill);

    // Header Row: Title on Left, Add Button on Right
    let header_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    header_row.set_margin_bottom(4);

    let title_lbl = gtk4::Label::new(Some(&babydra_core::i18n::trans("settings.vpn_title")));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(gtk4::Align::Start);
    title_lbl.set_hexpand(true);
    header_row.append(&title_lbl);

    let add_custom_btn = gtk4::Button::new();
    add_custom_btn.add_css_class("icon-btn");
    add_custom_btn.add_css_class("circular");
    add_custom_btn.set_cursor_from_name(Some("pointer"));
    add_custom_btn.set_valign(gtk4::Align::Center);
    add_custom_btn.set_size_request(34, 34);
    let add_icon = babydra_ui_kit::ui::icon::get_icon("plus", 16);
    add_icon.set_pixel_size(16);
    add_custom_btn.set_child(Some(&add_icon));
    add_custom_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("settings.vpn_add_profile")));
    header_row.append(&add_custom_btn);

    main_box.append(&header_row);

    // Overlay to host Stack and Dialog Modals
    let overlay = gtk4::Overlay::new();
    overlay.set_vexpand(true);
    overlay.set_hexpand(true);

    let stack = gtk4::Stack::new();
    stack.set_vexpand(true);
    stack.set_hexpand(true);
    stack.set_transition_type(gtk4::StackTransitionType::Crossfade);

    // 1. Grid of smart cards
    let card_grid = crate::widgets::helpers::ResponsiveCardGrid::new();
    stack.add_named(&card_grid.scrolled_win, Some("cards"));

    // 2. Loading state
    let loading_box = create_placeholder_box(PlaceholderState::Loading);
    stack.add_named(&loading_box, Some("loading"));

    // 3. Empty state
    let empty_box = create_placeholder_box(PlaceholderState::Empty {
        title_key: "settings.vpn_no_profiles",
        desc_key: Some("settings.vpn_no_profiles_sub"),
        icon_name: "shield",
    });
    stack.add_named(&empty_box, Some("empty"));

    stack.set_visible_child_name("loading");
    overlay.set_child(Some(&stack));

    // VpnConfigDialog & VpnLogDialog Modal Overlays
    let config_dialog = VpnConfigDialog::new();
    overlay.add_overlay(&config_dialog.container);

    let log_dialog = VpnLogDialog::new();
    overlay.add_overlay(&log_dialog.container);

    main_box.append(&overlay);

    (
        main_box,
        add_custom_btn,
        card_grid,
        stack,
        config_dialog,
        log_dialog,
    )
}
