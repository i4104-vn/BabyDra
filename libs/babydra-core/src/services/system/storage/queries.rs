//! Sizing disk partitions and storage query.

use super::helper::{format_disk_size, get_parent_drive};
pub use crate::models::DiskInfo;
use std::collections::HashMap;

/// Returns the current `disk list`.
pub fn get_disk_list() -> Vec<DiskInfo> {
    let mut drive_map: HashMap<String, (u64, u64, u64)> = HashMap::new();
    let mut seen_partitions = std::collections::HashSet::new();

    let output = std::process::Command::new("df").output();

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
                size: format_disk_size(total),
                used: format_disk_size(used),
                percent,
                mount_point: drive,
            });
        }
    }

    list.sort_by(|a, b| a.filesystem.cmp(&b.filesystem));
    list
}
