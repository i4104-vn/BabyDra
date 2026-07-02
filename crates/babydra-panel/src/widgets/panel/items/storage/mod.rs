pub mod render;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct DiskInfo {
    pub filesystem: String,
    pub size: String,
    pub used: String,
    pub percent: f64,
    pub mount_point: String,
}

fn get_parent_drive(filesystem: &str) -> String {
    if filesystem.starts_with("/dev/sd") {
        if filesystem.len() >= 8 {
            return filesystem[0..8].to_string();
        }
    } else if filesystem.starts_with("/dev/nvme") {
        if let Some(p_idx) = filesystem.rfind('p') {
            if p_idx > 9 {
                return filesystem[0..p_idx].to_string();
            }
        }
    }
    filesystem.to_string()
}

fn format_size(kb: u64) -> String {
    let gb = kb as f64 / 1024.0 / 1024.0;
    if gb >= 1000.0 {
        format!("{:.1} TB", gb / 1024.0)
    } else {
        format!("{:.1} GB", gb)
    }
}

pub fn get_disk_list() -> Vec<DiskInfo> {
    let mut drive_map: HashMap<String, (u64, u64, u64)> = HashMap::new();
    let mut seen_partitions = std::collections::HashSet::new();

    let output = std::process::Command::new("df")
        .output();
    
    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        for line in stdout.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                let filesystem = parts[0];
                if filesystem.starts_with("/dev/") {
                    if !seen_partitions.insert(filesystem.to_string()) {
                        continue;
                    }

                    let total_kb = parts[1].parse::<u64>().unwrap_or(0);
                    let used_kb = parts[2].parse::<u64>().unwrap_or(0);
                    let avail_kb = parts[3].parse::<u64>().unwrap_or(0);

                    let parent = get_parent_drive(filesystem);
                    let entry = drive_map.entry(parent).or_insert((0, 0, 0));
                    entry.0 += total_kb;
                    entry.1 += used_kb;
                    entry.2 += avail_kb;
                }
            }
        }
    }

    let mut list = Vec::new();
    for (drive, (total, used, _avail)) in drive_map {
        if total > 0 {
            let percent = (used as f64 / total as f64) * 100.0;
            list.push(DiskInfo {
                filesystem: drive.clone(),
                size: format_size(total),
                used: format_size(used),
                percent,
                mount_point: drive,
            });
        }
    }

    list.sort_by(|a, b| a.filesystem.cmp(&b.filesystem));
    list
}
