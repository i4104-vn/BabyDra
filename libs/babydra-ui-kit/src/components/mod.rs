pub mod badge;
pub mod buttons;
pub mod cards;
pub mod context_menu;
pub mod explore;
pub mod list_group;
pub mod modals;
pub mod placeholder;
pub mod popovers;
pub mod slider;
pub mod switch;
pub mod tooltips;
pub mod wifi;

// Re-export all builders under components namespace to maintain compatibility
pub use badge::create_icon_badge;
pub use buttons::{
    create_accent_button, create_battery_percentage_icon, create_button,
    create_colored_icon_button, create_colored_icon_widget, create_fab, create_icon_button,
    create_icon_label_button, create_sidebar_item_button, create_sidebar_item_button_with_widget,
    create_square_toggle_tile, create_toggle_tile, create_vpn_shield_icon,
    create_wallpaper_thumbnail_icon, create_wifi_signal_icon, update_toggle_tile_state,
};
pub use cards::{
    create_card, create_card_with_class, create_scrollable_list, create_subtitle,
    create_switch_card, create_title,
};
pub use context_menu::{
    build_tray_gio_menu, close_active_tray_menu, create_context_menu_for_widget,
    create_context_menu_popover, create_footer_container, create_footer_icon_button,
    create_menu_group_header, create_menu_item, create_menu_item_destructive,
    create_menu_item_destructive_sensitive, create_menu_item_full, create_menu_item_sensitive,
    create_menu_item_with_shortcut, create_menu_separator, create_menu_text_item,
    create_submenu_item, show_tray_context_menu, ContextMenuBuilder,
};
pub use list_group::{clear_box, clear_list_box, create_list_row};
pub use modals::{
    PasswordDialog, VpnConfigDialog, VpnLogDialog, WifiConfigDialog, WifiInfoDialog,
    WifiPasswordDialog,
};
pub use placeholder::{create_placeholder_row, PlaceholderState};
pub use popovers::{
    attach_hover_popover, build_hover_popover_card, create_popover, create_popover_with_content,
    HoverPopoverRow,
};
pub use slider::CustomSlider;
pub use switch::{create_switch, CustomSwitch, ToggleRow};
pub use tooltips::set_tooltip;
pub use wifi::{
    create_system_wifi_signal_icon, create_wifi_signal_icon_for_network,
    create_wifi_signal_icon_from_strength, render_wifi_signal_svg,
};
