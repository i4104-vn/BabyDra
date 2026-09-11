pub use crate::models::system::monitor::{AppResourceUsage, CpuTime, MonitorSnapshot};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Duration;

static MONITOR_SENDERS: Mutex<Vec<std::sync::mpsc::Sender<MonitorSnapshot>>> = Mutex::new(Vec::new());
static MONITOR_STARTED: AtomicBool = AtomicBool::new(false);

pub fn get_app_resource_usage(app_id: &str, exec: &str, name: &str) -> AppResourceUsage {
    let mut tokens: Vec<String> = Vec::new();

    let id_clean = app_id.strip_suffix(".desktop").unwrap_or(app_id).to_lowercase();
    if !id_clean.is_empty() {
        tokens.push(id_clean.clone());
        if let Some(last) = id_clean.split('.').last() {
            if last.len() >= 3 && last != id_clean {
                tokens.push(last.to_string());
            }
        }
    }

    let exec_bin = exec.split_whitespace().next().unwrap_or("");
    let exec_clean = std::path::Path::new(exec_bin)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("")
        .to_lowercase();
    if !exec_clean.is_empty() && !tokens.contains(&exec_clean) {
        tokens.push(exec_clean);
    }

    let name_clean = name.to_lowercase();
    if tokens.is_empty() && !name_clean.is_empty() {
        tokens.push(name_clean);
    }

    let mut total_cpu = 0.0;
    let mut total_rss_kb = 0u64;
    let mut is_running = false;

    if let Ok(output) = std::process::Command::new("ps")
        .args(["-eo", "state,%cpu,rss,comm,args"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 5 {
                    continue;
                }
                let state_char = parts[0].chars().next().unwrap_or('S');
                let cpu: f64 = parts[1].parse().unwrap_or(0.0);
                let rss: u64 = parts[2].parse().unwrap_or(0);
                let comm_lower = parts[3].to_lowercase();
                let args_lower = parts[4..].join(" ").to_lowercase();

                let mut matched = false;
                for t in &tokens {
                    let short_t = if t.len() > 15 { &t[..15] } else { t.as_str() };
                    if comm_lower == *t
                        || comm_lower.starts_with(short_t)
                        || args_lower.starts_with(t)
                        || args_lower.contains(&format!("/{}", t))
                    {
                        matched = true;
                        break;
                    }
                }

                if matched {
                    total_cpu += cpu;
                    total_rss_kb += rss;
                    if state_char == 'R' {
                        is_running = true;
                    }
                }
            }
        }
    }

    let ram_mb = total_rss_kb as f64 / 1024.0;
    let ram_formatted = if ram_mb >= 1024.0 {
        format!("{:.1} GB", ram_mb / 1024.0)
    } else {
        format!("{:.1} MB", ram_mb)
    };
    let num_cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .max(1) as f64;
    let normalized_cpu = (total_cpu / num_cpus).clamp(0.0, 100.0);
    let cpu_formatted = format!("{:.1}%", normalized_cpu);
    let is_running = is_running || total_cpu > 0.0;

    AppResourceUsage {
        cpu_percent: normalized_cpu,
        ram_mb,
        ram_formatted,
        cpu_formatted,
        is_running,
    }
}

static LAST_CPU: Mutex<Option<CpuTime>> = Mutex::new(None);

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

pub fn get_gpu_usage() -> Option<f64> {
    // AMD GPU
    if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("card") && name_str[4..].chars().all(|c| c.is_ascii_digit()) {
                let busy_file = entry.path().join("device/gpu_busy_percent");
                if busy_file.exists() {
                    if let Ok(content) = std::fs::read_to_string(&busy_file) {
                        if let Ok(val) = content.trim().parse::<f64>() {
                            return Some(val.clamp(0.0, 100.0));
                        }
                    }
                }
            }
        }
    }

    // Intel GPU
    static LAST_INTEL_SAMPLE: Mutex<Option<(std::time::Instant, u64)>> = Mutex::new(None);
    if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("card") && name_str[4..].chars().all(|c| c.is_ascii_digit()) {
                let card_dir = entry.path();
                let rc6_paths = [
                    card_dir.join("gt/gt0/rc6_residency_ms"),
                    card_dir.join("gt_rc6_ms"),
                ];

                for rc6_path in &rc6_paths {
                    if rc6_path.exists() {
                        if let Ok(content) = std::fs::read_to_string(rc6_path) {
                            if let Ok(current_rc6) = content.trim().parse::<u64>() {
                                let now = std::time::Instant::now();
                                let mut guard = LAST_INTEL_SAMPLE.lock().unwrap();
                                if let Some((last_time, last_rc6)) = *guard {
                                    let dt_ms = now.duration_since(last_time).as_secs_f64() * 1000.0;
                                    *guard = Some((now, current_rc6));

                                    if dt_ms >= 50.0 {
                                        let d_rc6 = current_rc6.saturating_sub(last_rc6) as f64;
                                        let idle_ratio = (d_rc6 / dt_ms).clamp(0.0, 1.0);
                                        let busy_pct = (1.0 - idle_ratio) * 100.0;
                                        return Some(busy_pct.clamp(0.0, 100.0));
                                    }
                                } else {
                                    *guard = Some((now, current_rc6));
                                }
                            }
                        }
                    }
                }

                // Intel Frequency ratio fallback
                let act_path = card_dir.join("gt_act_freq_mhz");
                let max_path = card_dir.join("gt_max_freq_mhz");
                let min_path = card_dir.join("gt_min_freq_mhz");
                if act_path.exists() && max_path.exists() {
                    let act = std::fs::read_to_string(&act_path)
                        .ok()
                        .and_then(|s| s.trim().parse::<f64>().ok());
                    let max = std::fs::read_to_string(&max_path)
                        .ok()
                        .and_then(|s| s.trim().parse::<f64>().ok());
                    let min = std::fs::read_to_string(&min_path)
                        .ok()
                        .and_then(|s| s.trim().parse::<f64>().ok())
                        .unwrap_or(0.0);
                    if let (Some(act_val), Some(max_val)) = (act, max) {
                        if max_val > min {
                            let ratio = ((act_val - min) / (max_val - min)) * 100.0;
                            return Some(ratio.clamp(0.0, 100.0));
                        }
                    }
                }
            }
        }
    }

    // NVIDIA GPU
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .args(["--query-gpu=utilization.gpu", "--format=csv,noheader,nounits"])
        .output()
    {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            if let Some(first_line) = s.lines().next() {
                if let Ok(val) = first_line.trim().parse::<f64>() {
                    return Some(val.clamp(0.0, 100.0));
                }
            }
        }
    }

    None
}

fn collect_snapshot() -> MonitorSnapshot {
    let cpu_percent = if let Some(current_cpu) = get_cpu_raw() {
        let mut last_cpu_guard = LAST_CPU.lock().unwrap();
        let percent = if let Some(ref last) = *last_cpu_guard {
            let total_diff = current_cpu.total.saturating_sub(last.total);
            let idle_diff = current_cpu.idle.saturating_sub(last.idle);
            if total_diff > 0 {
                let used_diff = total_diff.saturating_sub(idle_diff);
                (used_diff as f64 / total_diff as f64) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        };
        *last_cpu_guard = Some(current_cpu);
        percent
    } else {
        0.0
    };

    let (ram_used_gb, ram_total_gb, ram_percent) = get_ram_usage().unwrap_or((0.0, 0.0, 0.0));
    let gpu_percent = get_gpu_usage().unwrap_or(0.0);
    let uptime = get_formatted_uptime();

    MonitorSnapshot {
        cpu_percent,
        ram_used_gb,
        ram_total_gb,
        ram_percent,
        gpu_percent,
        uptime,
    }
}

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

pub use crate::models::system::monitor::MonitorReceiver;

pub fn subscribe() -> MonitorReceiver {
    init_monitor_service();
    let (tx, rx) = std::sync::mpsc::channel();
    MONITOR_SENDERS.lock().unwrap().push(tx);
    MonitorReceiver::new(rx)
}

pub fn init_monitor_service() {
    if MONITOR_STARTED.swap(true, Ordering::SeqCst) {
        return;
    }
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_millis(2000));
        let snapshot = collect_snapshot();
        let mut senders = MONITOR_SENDERS.lock().unwrap();
        senders.retain(|tx| tx.send(snapshot.clone()).is_ok());
    });
}