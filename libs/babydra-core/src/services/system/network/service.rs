pub use crate::models::network::{ActiveNetworkInfo, ActiveNetworkType, NetSpeed, NetStats, NetworkSnapshot};
use crate::services::system::wifi::client::{
    ActiveConnectionProxyBlocking, DeviceProxyBlocking, NetworkManagerProxyBlocking,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use zbus::blocking::Connection;

static NETWORK_SENDERS: Mutex<Vec<std::sync::mpsc::Sender<NetworkSnapshot>>> = Mutex::new(Vec::new());
static NETWORK_STARTED: AtomicBool = AtomicBool::new(false);
static LAST_NET_STATS: Mutex<Option<(Instant, NetStats)>> = Mutex::new(None);

pub fn get_net_bytes() -> NetStats {
    let mut total_rx = 0u64;
    let mut total_tx = 0u64;

    if let Ok(file) = std::fs::File::open("/proc/net/dev") {
        let reader = std::io::BufReader::new(file);
        for line in std::io::BufRead::lines(reader).flatten() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 10 {
                let iface = parts[0].trim_end_matches(':');
                if iface != "lo"
                    && !iface.starts_with("veth")
                    && !iface.starts_with("docker")
                    && !iface.starts_with("br-")
                {
                    let rx: u64 = parts[1].parse().unwrap_or(0);
                    let tx: u64 = parts[9].parse().unwrap_or(0);
                    total_rx += rx;
                    total_tx += tx;
                }
            }
        }
    }

    NetStats {
        rx_bytes: total_rx,
        tx_bytes: total_tx,
    }
}

pub fn calculate_network_speed() -> NetSpeed {
    let current_bytes = get_net_bytes();
    let now = Instant::now();

    let mut guard = LAST_NET_STATS.lock().unwrap();
    if let Some((last_time, last_bytes)) = guard.clone() {
        let elapsed = now.duration_since(last_time).as_secs_f64();
        if elapsed > 0.1 {
            let rx_diff = current_bytes.rx_bytes.saturating_sub(last_bytes.rx_bytes) as f64;
            let tx_diff = current_bytes.tx_bytes.saturating_sub(last_bytes.tx_bytes) as f64;

            let rx_speed = rx_diff / elapsed;
            let tx_speed = tx_diff / elapsed;

            *guard = Some((now, current_bytes));
            return NetSpeed { rx_speed, tx_speed };
        }
    }

    *guard = Some((now, current_bytes));
    NetSpeed {
        rx_speed: 0.0,
        tx_speed: 0.0,
    }
}

pub fn get_active_network_info() -> ActiveNetworkInfo {
    // 1. Try NetworkManager D-Bus
    if let Ok(conn) = Connection::system() {
        if let Ok(nm) = NetworkManagerProxyBlocking::new(&conn) {
            let primary_path = nm.primary_connection().ok();
            let active_paths = nm.active_connections().unwrap_or_default();

            let target_path = primary_path.filter(|p| p.as_str() != "/").or_else(|| {
                for path in &active_paths {
                    if path.as_str() == "/" {
                        continue;
                    }
                    if let Ok(builder) =
                        ActiveConnectionProxyBlocking::builder(&conn).path(path.clone())
                    {
                        if let Ok(ac) = builder.build() {
                            let state = ac.state().unwrap_or(0);
                            let is_default = ac.default().unwrap_or(false);
                            let conn_type = ac.type_().unwrap_or_default();
                            // State 2 is NM_ACTIVE_CONNECTION_STATE_ACTIVATED
                            if state == 2
                                && (is_default
                                    || conn_type.contains("ethernet")
                                    || conn_type.contains("wireless"))
                            {
                                return Some(path.clone());
                            }
                        }
                    }
                }
                None
            });

            if let Some(path) = target_path {
                if let Ok(builder) = ActiveConnectionProxyBlocking::builder(&conn).path(path) {
                    if let Ok(ac) = builder.build() {
                        let name = ac.id().unwrap_or_default();
                        let conn_type = ac.type_().unwrap_or_default();
                        let devices = ac.devices().unwrap_or_default();
                        let mut iface_name = String::new();
                        let mut dev_type = 0;

                        if let Some(dev_path) = devices.first() {
                            if let Ok(dev_builder) =
                                DeviceProxyBlocking::builder(&conn).path(dev_path.clone())
                            {
                                if let Ok(dev) = dev_builder.build() {
                                    iface_name = dev.interface().unwrap_or_default();
                                    dev_type = dev.device_type().unwrap_or(0);
                                }
                            }
                        }

                        let ip_address = get_local_ip();

                        // 2 is NM_DEVICE_TYPE_WIFI
                        let is_wifi = conn_type == "802-11-wireless"
                            || dev_type == 2
                            || iface_name.starts_with("wl");
                        if is_wifi {
                            let display_name = if name.is_empty() {
                                let (_, wifi_ssid) =
                                    crate::services::system::wifi::get_wifi_state();
                                if !wifi_ssid.is_empty()
                                    && wifi_ssid != "Disconnected"
                                    && wifi_ssid != "Off"
                                {
                                    wifi_ssid
                                } else {
                                    "Wi-Fi".to_string()
                                }
                            } else {
                                name
                            };

                            return ActiveNetworkInfo {
                                network_type: ActiveNetworkType::Wifi,
                                is_connected: true,
                                name: display_name,
                                ip_address,
                                icon_name: "wifi".to_string(),
                                interface: iface_name,
                            };
                        } else {
                            let display_name = if name.is_empty() {
                                "Ethernet".to_string()
                            } else {
                                name
                            };

                            return ActiveNetworkInfo {
                                network_type: ActiveNetworkType::Ethernet,
                                is_connected: true,
                                name: display_name,
                                ip_address,
                                icon_name: "desktop".to_string(),
                                interface: iface_name,
                            };
                        }
                    }
                }
            }
        }
    }

    // 2. Fallback: Parse `ip route get 1.1.1.1`
    if let Ok(output) = std::process::Command::new("sh")
        .arg("-c")
        .arg("ip route get 1.1.1.1 2>/dev/null")
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout);
        if !text.trim().is_empty() {
            let mut dev = String::new();
            let mut src = String::new();

            let tokens: Vec<&str> = text.split_whitespace().collect();
            for i in 0..tokens.len() {
                if tokens[i] == "dev" && i + 1 < tokens.len() {
                    dev = tokens[i + 1].to_string();
                } else if tokens[i] == "src" && i + 1 < tokens.len() {
                    src = tokens[i + 1].to_string();
                }
            }

            if !dev.is_empty() && dev != "lo" {
                let ip_address = if src.is_empty() {
                    get_local_ip()
                } else {
                    src
                };
                if dev.starts_with("wl") {
                    let (_, wifi_ssid) = crate::services::system::wifi::get_wifi_state();
                    let name = if !wifi_ssid.is_empty()
                        && wifi_ssid != "Disconnected"
                        && wifi_ssid != "Off"
                    {
                        wifi_ssid
                    } else {
                        "Wi-Fi".to_string()
                    };

                    return ActiveNetworkInfo {
                        network_type: ActiveNetworkType::Wifi,
                        is_connected: true,
                        name,
                        ip_address,
                        icon_name: "wifi".to_string(),
                        interface: dev,
                    };
                } else {
                    return ActiveNetworkInfo {
                        network_type: ActiveNetworkType::Ethernet,
                        is_connected: true,
                        name: "Ethernet".to_string(),
                        ip_address,
                        icon_name: "desktop".to_string(),
                        interface: dev,
                    };
                }
            }
        }
    }

    // 3. Disconnected fallback
    ActiveNetworkInfo::default()
}

fn collect_network_snapshot() -> NetworkSnapshot {
    let active_info = get_active_network_info();
    let net_speed = calculate_network_speed();
    let net_bytes = get_net_bytes();

    NetworkSnapshot {
        active_info,
        rx_speed: net_speed.rx_speed,
        tx_speed: net_speed.tx_speed,
        rx_bytes: net_bytes.rx_bytes,
        tx_bytes: net_bytes.tx_bytes,
    }
}

pub fn get_local_ip() -> String {
    if let Ok(output) = std::process::Command::new("sh")
        .arg("-c")
        .arg("ip route get 1.1.1.1 2>/dev/null | grep -oP 'src \\K[0-9.]+' | head -n 1")
        .output()
    {
        let ip = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !ip.is_empty() {
            return ip;
        }
    }
    "127.0.0.1".to_string()
}

pub fn format_speed(bytes_per_sec: f64) -> String {
    if bytes_per_sec < 1024.0 {
        format!("{:.0} B/s", bytes_per_sec)
    } else if bytes_per_sec < 1024.0 * 1024.0 {
        format!("{:.1} KB/s", bytes_per_sec / 1024.0)
    } else {
        format!("{:.1} MB/s", bytes_per_sec / (1024.0 * 1024.0))
    }
}

pub use crate::models::network::NetworkReceiver;

pub fn subscribe() -> NetworkReceiver {
    init_network_monitor_service();
    let (tx, rx) = std::sync::mpsc::channel();
    NETWORK_SENDERS.lock().unwrap().push(tx);
    NetworkReceiver::new(rx)
}

pub fn init_network_monitor_service() {
    if NETWORK_STARTED.swap(true, Ordering::SeqCst) {
        return;
    }
    std::thread::spawn(move || loop {
        let has_subscribers = {
            let senders = NETWORK_SENDERS.lock().unwrap();
            !senders.is_empty()
        };

        if !has_subscribers {
            std::thread::sleep(Duration::from_millis(1000));
            continue;
        }

        std::thread::sleep(Duration::from_millis(200));
        let snapshot = collect_network_snapshot();
        let mut senders = NETWORK_SENDERS.lock().unwrap();
        senders.retain(|tx| tx.send(snapshot.clone()).is_ok());
    });
}

pub fn get_network_speed() -> NetSpeed {
    calculate_network_speed()
}