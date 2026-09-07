pub mod account;
pub mod auth;
pub mod backlight;
pub mod battery;
pub mod bluetooth;
pub mod certificates;
pub mod clean;
pub mod display;
pub mod gpu;
pub mod gsettings;
pub mod keymap;
pub mod monitor;
pub mod network;
pub mod power;
pub mod reset;
pub mod startup;
pub mod storage;
pub mod theme;
pub mod updates;
pub mod volume;
pub mod vpn;
pub mod wifi;

pub use account::{
    change_user_password, get_system_hostname, get_user_account_info, update_display_name,
    update_system_hostname, validate_hostname, UserAccountInfo,
};
pub use gsettings::set_color_scheme;
