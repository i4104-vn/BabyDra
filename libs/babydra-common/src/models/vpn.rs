use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VpnConn {
    pub name: String,
    pub conn_type: String,
    pub active: bool,
    pub gateway: String,
    pub username: String,
    pub path: String,
    pub ip_address: String,
    pub remote_server: String,
    pub dev_iface: String,
    pub cipher: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VpnConnDetails {
    pub name: String,
    pub original_name: Option<String>,
    pub vpn_type: String,
    pub gateway: String,
    pub username: String,
    pub password: String,
    pub ca_cert: String,
    pub config_file: Option<String>,
}
