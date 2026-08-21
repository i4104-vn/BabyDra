//! Integration tests: Explore URI decoding and target folder resolution.

use babydra_core::services::explore::{resolve_target_from_path, resolve_target_from_uri};
use babydra_core::services::mpris::decode_uri;
use std::fs::File;

#[test]
fn test_decode_uri_ascii_and_spaces() {
    assert_eq!(decode_uri("My%20Documents"), "My Documents");
    assert_eq!(decode_uri("/home/user/Downloads/test%20file.pdf"), "/home/user/Downloads/test file.pdf");
}

#[test]
fn test_decode_uri_utf8_multibyte() {
    // "Tải xuống" in percent encoding: T%E1%BA%A3i%20xu%E1%BB%91ng
    assert_eq!(decode_uri("T%E1%BA%A3i%20xu%E1%BB%91ng"), "Tải xuống");
    // "ảnh đẹp": %E1%BA%A3nh%20%C4%91%E1%BA%B9p
    assert_eq!(decode_uri("%E1%BA%A3nh%20%C4%91%E1%BA%B9p"), "ảnh đẹp");
}

#[test]
fn test_resolve_target_from_directory() {
    let tmp = std::env::temp_dir();
    let uri = format!("file://{}", tmp.display());
    let (target_dir, focus_item) = resolve_target_from_uri(&uri);
    assert_eq!(target_dir, tmp);
    assert_eq!(focus_item, None);
}

#[test]
fn test_resolve_target_from_file_in_directory() {
    let tmp = std::env::temp_dir();
    let test_file = tmp.join("babydra_test_download_sample.txt");
    let _ = File::create(&test_file);

    let uri = format!("file://{}", test_file.display());
    let (target_dir, focus_item) = resolve_target_from_uri(&uri);
    assert_eq!(target_dir, tmp);
    assert_eq!(focus_item, Some(test_file.clone()));

    let _ = std::fs::remove_file(test_file);
}

#[test]
fn test_resolve_target_from_pending_or_virtual_file() {
    let tmp = std::env::temp_dir();
    let pending_file = tmp.join("non_existent_yet.crdownload");

    let uri = format!("file://{}", pending_file.display());
    let (target_dir, focus_item) = resolve_target_from_uri(&uri);
    assert_eq!(target_dir, tmp);
    assert_eq!(focus_item, Some(pending_file));
}

#[test]
fn test_resolve_target_from_plain_path() {
    let tmp = std::env::temp_dir();
    let (target_dir, _) = resolve_target_from_path(&tmp);
    assert_eq!(target_dir, tmp);
}
