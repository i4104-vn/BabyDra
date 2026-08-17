//! Drive parent lookup and human-readable capacity formatting helpers.

pub fn get_parent_drive(filesystem: &str) -> String {
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

pub fn format_size(kb: u64) -> String {
    let gb = kb as f64 / 1024.0 / 1024.0;
    if gb >= 1000.0 {
        format!("{:.1} TB", gb / 1024.0)
    } else {
        format!("{:.1} GB", gb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_size_returns_gb_for_small_values() {
        assert_eq!(format_size(1024 * 1024), "1.0 GB");
        assert_eq!(format_size(0), "0.0 GB");
        assert_eq!(format_size(512 * 1024), "0.5 GB");
    }

    #[test]
    fn format_size_returns_tb_for_large_values() {
        assert_eq!(format_size(1000 * 1024 * 1024), "1.0 TB");
        assert_eq!(format_size(2000 * 1024 * 1024), "2.0 TB");
    }

    #[test]
    fn format_size_rounds_to_one_decimal() {
        // 1.5 GB
        assert_eq!(format_size((1.5 * 1024.0 * 1024.0) as u64), "1.5 GB");
        // 2.5 TB
        assert_eq!(
            format_size((2.5 * 1024.0 * 1024.0 * 1024.0) as u64),
            "2.5 TB"
        );
    }

    #[test]
    fn get_parent_drive_maps_sd_partitions() {
        assert_eq!(get_parent_drive("/dev/sda1"), "/dev/sda");
        assert_eq!(get_parent_drive("/dev/sdb5"), "/dev/sdb");
    }

    #[test]
    fn get_parent_drive_maps_nvme_partitions() {
        assert_eq!(get_parent_drive("/dev/nvme0n1p1"), "/dev/nvme0n1");
        assert_eq!(get_parent_drive("/dev/nvme1n2p3"), "/dev/nvme1n2");
    }

    #[test]
    fn get_parent_drive_returns_input_for_non_partitions() {
        assert_eq!(
            get_parent_drive("/dev/mapper/luks-root"),
            "/dev/mapper/luks-root"
        );
    }
}
