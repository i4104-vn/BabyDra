pub mod badge;
pub mod buttons;
pub mod card;
pub mod close_button;
pub mod list_group;
pub mod modal;
pub mod navbar;
pub mod placeholder;
pub mod popovers;
pub mod progress;
pub mod slider;
pub mod spinners;
pub mod switch;
pub mod tooltips;
pub mod wifi;

// Re-export all builders under components namespace to maintain compatibility
pub use badge::{create_icon_badge, create_status_badge};
pub use buttons::{
    create_accent_button, create_battery_percentage_icon, create_button,
    create_colored_icon_button, create_colored_icon_widget, create_fab, create_icon_button,
    create_icon_label_button, create_sidebar_item_button, create_sidebar_item_button_with_widget,
    create_square_toggle_tile, create_toggle_tile, create_vpn_shield_icon,
    create_wallpaper_thumbnail_icon, create_wifi_signal_icon, update_toggle_tile_state,
};
pub use card::{
    create_card, create_card_with_class, create_item_row, create_scrollable_list,
    create_subtitle, create_switch_card, create_title,
};
pub use close_button::{create_close_button, create_close_button_with_label};
pub use list_group::{clear_box, clear_list_box, create_list_row};
pub use modal::create_dialog_box;
pub use navbar::{create_sidebar_row, create_sidebar_row_with_badge};
pub use placeholder::{create_placeholder_message, create_placeholder_row, PlaceholderState};
pub use popovers::{
    attach_hover_popover, build_hover_popover_card, create_popover, create_popover_with_content,
    HoverPopoverRow,
};
pub use progress::{create_disk_progress, create_progress_bar};
pub use slider::CustomSlider;
pub use spinners::{create_loading_box, create_spinner};
pub use switch::{create_switch, CustomSwitch, ToggleRow};
pub use tooltips::set_tooltip;
pub use wifi::{
    create_system_wifi_signal_icon, create_wifi_signal_icon_for_network,
    create_wifi_signal_icon_from_strength, render_wifi_signal_svg,
};
