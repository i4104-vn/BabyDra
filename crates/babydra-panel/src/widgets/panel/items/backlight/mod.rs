pub mod render;

pub use babydra_core::services::system::backlight::{
    detect_ddc_bus, get_current_brightness, has_backlight, query_ddcutil_brightness,
    set_brightness, BRIGHTNESS_STATE, BRIGHTNESS_SYNCED,
};
