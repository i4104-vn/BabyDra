use crate::models::vpn::*;
use crate::services::utils::{run_cmd, run_cmd_bool};

/// Connects to `VPN`.
pub fn connect_vpn(name: &str) -> bool {
    run_cmd_bool(&["nmcli", "connection", "up", name])
}

/// Disconnects from `VPN`.
pub fn disconnect_vpn(name: &str) -> bool {
    run_cmd_bool(&["nmcli", "connection", "down", name])
}

/// Delete VPN connection.
pub fn delete_vpn_connection(name: &str) -> bool {
    run_cmd_bool(&["nmcli", "connection", "delete", name])
}

/// Persists `VPN connection`.
pub fn save_vpn_connection(details: &VpnConnDetails) -> Result<(), String> {
    if let Some(ref src_path) = details.config_file {
        if !src_path.is_empty() {
            let _ = crate::services::system::vpn::config::copy_vpn_config_to_babydra_dir(src_path);
        }
    }

    let conn_name = if details.name.is_empty() {
        "VPN Connection"
    } else {
        &details.name
    };
    let orig_name = details.original_name.as_deref().unwrap_or(conn_name);

    let name_str = conn_name.to_string();
    let orig_str = orig_name.to_string();
    let un_str = details.username.clone();
    let pw_str = details.password.clone();
    let gw_str = details.gateway.clone();
    let ca_str = details.ca_cert.clone();
    let vpn_type = if details.vpn_type.is_empty() {
        "openvpn"
    } else {
        &details.vpn_type
    };

    let exists = run_cmd(&["nmcli", "connection", "show", &orig_str]).is_some();

    if exists {
        if orig_str != name_str {
            run_cmd_bool(&[
                "nmcli",
                "connection",
                "modify",
                &orig_str,
                "connection.id",
                &name_str,
            ]);
        }
        if !un_str.is_empty() {
            run_cmd_bool(&[
                "nmcli",
                "connection",
                "modify",
                &name_str,
                "vpn.user-name",
                &un_str,
            ]);
        }
        if !pw_str.is_empty() {
            run_cmd_bool(&[
                "nmcli",
                "connection",
                "modify",
                &name_str,
                "vpn.secrets",
                &format!("password={}", pw_str),
            ]);
            run_cmd_bool(&[
                "nmcli",
                "connection",
                "modify",
                &name_str,
                "+vpn.data",
                "password-flags=0",
            ]);
        }
        if !gw_str.is_empty() {
            run_cmd_bool(&[
                "nmcli",
                "connection",
                "modify",
                &name_str,
                "+vpn.data",
                &format!("remote={}", gw_str),
            ]);
        }
        if !ca_str.is_empty() {
            run_cmd_bool(&[
                "nmcli",
                "connection",
                "modify",
                &name_str,
                "+vpn.data",
                &format!("ca={}", ca_str),
            ]);
        }
        return Ok(());
    }

    // New connection creation via nmcli
    let service_type = format!("org.freedesktop.NetworkManager.{}", vpn_type);
    let mut add_args = vec![
        "nmcli".to_string(),
        "connection".to_string(),
        "add".to_string(),
        "type".to_string(),
        "vpn".to_string(),
        "con-name".to_string(),
        name_str.clone(),
        "vpn-type".to_string(),
        vpn_type.to_string(),
        "vpn.service-type".to_string(),
        service_type,
    ];

    if !un_str.is_empty() {
        add_args.push("vpn.user-name".to_string());
        add_args.push(un_str.clone());
    }

    let mut vpn_data_items = vec!["password-flags=0".to_string()];
    if !gw_str.is_empty() {
        vpn_data_items.push(format!("remote={}", gw_str));
    }
    if !ca_str.is_empty() {
        vpn_data_items.push(format!("ca={}", ca_str));
    }
    let conn_type = if !ca_str.is_empty() && !un_str.is_empty() {
        "password-tls"
    } else if !ca_str.is_empty() {
        "tls"
    } else {
        "password"
    };
    vpn_data_items.push(format!("connection-type={}", conn_type));

    let vpn_data_str = vpn_data_items.join(",");
    add_args.push("vpn.data".to_string());
    add_args.push(vpn_data_str);

    if !pw_str.is_empty() {
        add_args.push("vpn.secrets".to_string());
        add_args.push(format!("password={}", pw_str));
    }

    let args_ref: Vec<&str> = add_args.iter().map(|s| s.as_str()).collect();
    let status = run_cmd_bool(&args_ref);

    if status {
        if !pw_str.is_empty() {
            run_cmd_bool(&[
                "nmcli",
                "connection",
                "modify",
                &name_str,
                "vpn.secrets",
                &format!("password={}", pw_str),
            ]);
            run_cmd_bool(&[
                "nmcli",
                "connection",
                "modify",
                &name_str,
                "+vpn.data",
                "password-flags=0",
            ]);
        }
        Ok(())
    } else {
        Err("Failed to create VPN connection via nmcli".to_string())
    }
}
