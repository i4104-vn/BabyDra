pub mod service;

pub use crate::models::shell::network::{ActiveNetworkInfo, ActiveNetworkType, NetSpeed, NetStats};
pub use service::{
    format_speed, get_active_network_info, get_local_ip, get_net_bytes, get_network_speed,
    init_network_monitor_service, subscribe, NetworkSnapshot,
};
