//! Integration tests: explore file grouping (`models::explore::grouping`).
//!
//! Verifies locale-aware group naming for today/folders/other files through
//! the public `get_group_name` API.

use babydra_core::i18n::{set_locale, t};
use babydra_core::models::explore::file_entry::{FileEntry, FileType};
use babydra_core::models::explore::grouping::get_group_name;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::Mutex;

// `get_group_name` reads the process-global locale; serialize these tests.
static GROUPING_LOCK: Mutex<()> = Mutex::new(());

fn entry(file_type: FileType) -> FileEntry {
    FileEntry {
        path: PathBuf::from("/tmp/test"),
        name: OsString::from("test"),
        display_name: "test".to_string(),
        file_type,
        mime_type: String::new(),
        size: 0,
        modified: None,
        created: None,
        permissions: 0o644,
        owner: String::new(),
        group: String::new(),
        is_hidden: false,
        icon_name: String::new(),
        thumbnail_path: None,
    }
}

#[test]
fn group_today_uses_locale() {
    let _guard = GROUPING_LOCK.lock().unwrap();
    set_locale("en");
    let today = chrono::Local::now();
    let mut e = entry(FileType::RegularFile);
    e.modified = Some(today.into());
    let name = get_group_name(&e, "date");
    assert!(name.starts_with("Today"), "got: {name}");
}

#[test]
fn group_folders_uses_locale() {
    let _guard = GROUPING_LOCK.lock().unwrap();
    set_locale("vi");
    assert_eq!(
        get_group_name(&entry(FileType::Directory), "group"),
        t("explore.group_folders")
    );
}

#[test]
fn group_other_files_uses_locale() {
    let _guard = GROUPING_LOCK.lock().unwrap();
    set_locale("en");
    assert_eq!(
        get_group_name(&entry(FileType::RegularFile), "group"),
        t("explore.group_other_files")
    );
}
