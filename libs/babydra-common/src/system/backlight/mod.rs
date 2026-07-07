//! Screen backlight / brightness subsystem module wrapper.

pub mod detection;
pub mod state;
pub mod control;

pub use detection::{detect_ddc_bus, has_backlight, DDC_BUS, BRIGHTNESS_SYNCED};
pub use state::{get_current_brightness, query_ddcutil_brightness, BRIGHTNESS_STATE};
pub use control::set_brightness;
