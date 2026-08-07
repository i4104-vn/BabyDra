//! Network traffic monitoring and speed formatting.

use std::sync::Mutex;
use std::time::Instant;
pub use crate::models::{NetStats, NetSpeed};

static LAST_NET_STATS: Mutex<Option<(Instant, NetStats)>> = Mutex::new(None);

/// Reads total RX and TX bytes from `/proc/net/dev` across active network interfaces.
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

    NetStats { rx_bytes: total_rx, tx_bytes: total_tx }
}

/// Calculates current network download (RX) and upload (TX) speed per second.
pub fn get_network_speed() -> NetSpeed {
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
    NetSpeed { rx_speed: 0.0, tx_speed: 0.0 }
}

/// Formats speed value into human-readable string (e.g. `1.2 MB/s`, `450 KB/s`, `12 B/s`).
pub fn format_speed(bytes_per_sec: f64) -> String {
    if bytes_per_sec < 1024.0 {
        format!("{:.0} B/s", bytes_per_sec)
    } else if bytes_per_sec < 1024.0 * 1024.0 {
        format!("{:.1} KB/s", bytes_per_sec / 1024.0)
    } else {
        format!("{:.1} MB/s", bytes_per_sec / (1024.0 * 1024.0))
    }
}
<<<<<<< HEAD

/// Retrieves local IPv4 address of the active default network route interface.
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

=======
>>>>>>> hard-develop
