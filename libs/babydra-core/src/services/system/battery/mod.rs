//! Battery subsystem service for reading /sys/class/power_supply, saver and limit control.

pub mod limit;
pub mod reader;
pub mod saver;
pub mod service;

pub use limit::{charge_limit_path, has_charge_limit, set_charge_limit, set_charge_limit_pw};
pub use reader::get_battery_info;
pub use saver::apply_battery_saver;
pub use service::{init_battery_service, subscribe};
