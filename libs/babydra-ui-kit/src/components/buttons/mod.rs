pub mod icon;
pub mod standard;
pub mod tile;

pub use icon::{
    create_battery_icon, create_color_btn, create_colored_icon, create_icon_btn,
    create_icon_button, create_sidebar_btn, create_sidebar_wbtn, create_vpn_icon, create_wifi_icon,
    create_wp_thumb,
};
pub use standard::{create_accent_button, create_button, create_fab};
pub use tile::{create_square_tile, create_toggle_tile, update_toggle_state};
