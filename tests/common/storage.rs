//! Integration tests: storage helpers.
//!
//! Verifies human-readable capacity formatting and partition → parent
//! drive mapping through the public helper API.

use babydra_core::services::system::storage::helper::{format_disk_size, get_parent_drive};

#[test]
fn format_disk_size_uses_gb_below_1000() {
    assert_eq!(format_disk_size(1024 * 1024), "1.0 GB");
    assert_eq!(format_disk_size(512 * 1024), "0.5 GB");
    assert_eq!(format_disk_size(0), "0.0 GB");
}

#[test]
fn format_disk_size_switches_to_tb_at_1000_gb() {
    assert_eq!(format_disk_size(1000 * 1024 * 1024), "1.0 TB");
    assert_eq!(format_disk_size(2048 * 1024 * 1024), "2.0 TB");
}

#[test]
fn parent_drive_maps_sd_partitions() {
    assert_eq!(get_parent_drive("/dev/sda1"), "/dev/sda");
    assert_eq!(get_parent_drive("/dev/sdb5"), "/dev/sdb");
}

#[test]
fn parent_drive_maps_nvme_partitions() {
    assert_eq!(get_parent_drive("/dev/nvme0n1p1"), "/dev/nvme0n1");
    assert_eq!(get_parent_drive("/dev/nvme1n2p3"), "/dev/nvme1n2");
}

#[test]
fn parent_drive_leaves_other_paths_untouched() {
    assert_eq!(
        get_parent_drive("/dev/mapper/luks-root"),
        "/dev/mapper/luks-root"
    );
}

#[test]
fn format_disk_size_rounds_to_one_decimal() {
    assert_eq!(format_disk_size((1.5 * 1024.0 * 1024.0) as u64), "1.5 GB");
    assert_eq!(
        format_disk_size((2.5 * 1024.0 * 1024.0 * 1024.0) as u64),
        "2.5 TB"
    );
}
