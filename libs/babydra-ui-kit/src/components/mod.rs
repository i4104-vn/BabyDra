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
pub mod wifi;

// Re-export all builders under components namespace
pub use badge::create_icon_badge;
pub use buttons::{
    create_accent_button, create_battery_icon, create_button, create_color_btn,
    create_colored_icon, create_fab, create_icon_btn, create_icon_button, create_sidebar_btn,
    create_sidebar_wbtn, create_square_tile, create_toggle_tile, create_vpn_icon, create_wifi_icon,
    create_wp_thumb, update_toggle_state,
};
pub use cards::{
    create_card, create_collapsible_card, create_css_card, create_scroll_list,
    create_slider_header, create_subtitle, create_switch_card, create_title, CollapsibleCard,
};
pub use context_menu::{
    build_tray_gio_menu, close_tray_menu, create_danger_btn, create_danger_item, create_footer_box,
    create_footer_btn, create_group_header, create_menu_for, create_menu_full, create_menu_item,
    create_menu_popover, create_menu_sens, create_menu_sep, create_menu_shortcut, create_menu_text,
    create_submenu_item, show_tray_menu, ContextMenuBuilder,
};
pub use list_group::{clear_box, clear_list_box, create_list_row};
pub use modals::{
    ChangeHostnameDialog, ChangeNameDialog, ChangePasswordDialog, PasswordDialog, VpnConfigDialog,
    VpnLogDialog, WifiConfigDialog, WifiInfoDialog, WifiPasswordDialog,
};
pub use placeholder::{create_placeholder, PlaceholderState};
pub use popovers::{create_popover, TooltipPopover, TooltipRow};
pub use slider::{bind_debounced_slider, CustomSlider, PillSlider};
pub use switch::{create_switch, CustomSwitch, ToggleRow};
pub use wifi::{create_rssi_icon, create_sys_wifi_icon, create_wifi_net_icon, render_wifi_svg};
