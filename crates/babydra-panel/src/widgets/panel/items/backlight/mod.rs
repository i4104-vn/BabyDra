pub mod render;

pub use babydra_common::helper::backlight::{
    DDC_BUS, BRIGHTNESS_STATE, BRIGHTNESS_SYNCED,
    detect_ddc_bus, has_backlight, get_current_brightness, set_brightness, query_ddcutil_brightness,
};
