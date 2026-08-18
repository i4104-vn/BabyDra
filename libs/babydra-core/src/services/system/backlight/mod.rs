//! Screen backlight / brightness subsystem module wrapper.

pub mod control;
pub mod detection;
pub mod state;

pub use control::set_brightness;
pub use detection::{
    detect_ddc_bus, get_backlight_device, has_backlight, BRIGHTNESS_SYNCED, DDC_BUS,
};
pub use state::{get_brightness, query_ddc_brightness, BRIGHTNESS_STATE};
