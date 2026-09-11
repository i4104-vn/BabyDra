//! General shared utilities across core services.

pub mod command;
pub mod fs;
pub mod gsettings;
pub mod xdg;

pub use command::{
    get_home_dir, pkill, pkill_signal, run_cmd, run_cmd_bool, run_cmd_output, run_sudo, spawn_sh,
};
pub use fs::{
    append_line_if_missing, ensure_dir_exists, format_bytes, get_babydra_config_dir,
    get_home_path, read_sysfs_f64, read_sysfs_u32, read_trimmed_string, set_unix_mode,
};
pub use gsettings::{
    get as gsettings_get, get_bool as gsettings_get_bool, set as gsettings_set,
    set_bool as gsettings_set_bool, set_string as gsettings_set_string,
};
pub use xdg::{
    find_matching_apps, get_setting as xdg_get_setting, open_path as xdg_open,
    query_default as xdg_query_default, set_default as xdg_set_default,
    set_setting as xdg_set_setting,
};