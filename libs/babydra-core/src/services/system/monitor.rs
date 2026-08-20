//! System resource monitor statistical helper calculations.

pub use crate::models::shell::monitor::CpuTime;

/// Reads raw CPU tick numbers from `/proc/stat`.
pub fn get_cpu_raw() -> Option<CpuTime> {
    let file = std::fs::File::open("/proc/stat").ok()?;
    let reader = std::io::BufReader::new(file);
    if let Some(Ok(line)) = std::io::BufRead::lines(reader).next() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 && parts[0] == "cpu" {
            let user: u64 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            let nice: u64 = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
            let system: u64 = parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);
            let idle: u64 = parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);
            let iowait: u64 = parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(0);
            let irq: u64 = parts.get(6).and_then(|s| s.parse().ok()).unwrap_or(0);
            let softirq: u64 = parts.get(7).and_then(|s| s.parse().ok()).unwrap_or(0);
            let steal: u64 = parts.get(8).and_then(|s| s.parse().ok()).unwrap_or(0);

            let idle_time = idle + iowait;
            let total_time = user + nice + system + idle_time + irq + softirq + steal;
            return Some(CpuTime {
                total: total_time,
                idle: idle_time,
            });
        }
    }
    None
}

/// Reads raw RAM size information from `/proc/meminfo`.
/// Returns a tuple containing `(used_gb, total_gb, usage_percent)`.
pub fn get_ram_usage() -> Option<(f64, f64, f64)> {
    let file = std::fs::File::open("/proc/meminfo").ok()?;
    let reader = std::io::BufReader::new(file);

    let mut mem_total = 0.0;
    let mut mem_avail = 0.0;

    for line in std::io::BufRead::lines(reader).flatten() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if parts[0] == "MemTotal:" {
                mem_total = parts[1].parse::<f64>().unwrap_or(0.0);
            } else if parts[0] == "MemAvailable:" {
                mem_avail = parts[1].parse::<f64>().unwrap_or(0.0);
            }
        }
    }

    if mem_total > 0.0 {
        let used = mem_total - mem_avail;
        let percent = (used / mem_total) * 100.0;
        let used_gb = used / 1024.0 / 1024.0;
        let total_gb = mem_total / 1024.0 / 1024.0;
        Some((used_gb, total_gb, percent))
    } else {
        None
    }
}

/// Reads and formats the current system uptime (e.g. "3d 2h 15m", "4h 20m", or "45m").
pub fn get_formatted_uptime() -> String {
    if let Ok(content) = std::fs::read_to_string("/proc/uptime") {
        if let Some(first) = content.split_whitespace().next() {
            if let Ok(secs_f) = first.parse::<f64>() {
                let uptime_secs = secs_f as u64;
                let days = uptime_secs / 86400;
                let hours = (uptime_secs % 86400) / 3600;
                let mins = (uptime_secs % 3600) / 60;

                return if days > 0 {
                    format!("{}d {}h {}m", days, hours, mins)
                } else if hours > 0 {
                    format!("{}h {}m", hours, mins)
                } else {
                    format!("{}m", mins)
                };
            }
        }
    }
    "0m".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uptime_formatting() {
        let uptime = get_formatted_uptime();
        assert!(!uptime.is_empty(), "Uptime string should not be empty");
        assert!(uptime.ends_with('m'), "Uptime string should end with 'm' (e.g. 5m, 1h 20m): {}", uptime);
    }
}

