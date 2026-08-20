//! Integration tests: internationalization (i18n).
//!
//! Verifies locale-aware key lookup through the public `trans()` API for both
//! the `en` and `vi` locales.

use babydra_core::i18n::{get_locale, set_locale, trans};
use std::sync::Mutex;

// `trans()` reads a process-global locale; serialize these tests to avoid races.
static I18N_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn english_lookup_returns_translations() {
    let _guard = I18N_LOCK.lock().unwrap();
    set_locale("en");
    assert_eq!(trans("menu.terminal"), "Terminal");
    assert_eq!(trans("panel.no_notifications"), "No new notifications");
    assert_eq!(trans("desktop.new_folder"), "New Folder");
    assert_eq!(trans("desktop.open_in_terminal"), "Open in Terminal");
    assert_eq!(trans("desktop.more"), "More...");
    assert_eq!(trans("desktop.other_apps"), "Other Applications");
    assert_eq!(trans("desktop.back"), "Back");
    assert_eq!(trans("explore.menu_open_with"), "Open With...");
}

#[test]
fn vietnamese_lookup_returns_translations() {
    let _guard = I18N_LOCK.lock().unwrap();
    set_locale("vi");
    assert_eq!(trans("menu.file_manager"), "Trình quản lý tệp");
    assert_eq!(trans("weekday.mon"), "Thứ Hai");
    assert_eq!(trans("desktop.new_folder"), "Thư mục mới");
    assert_eq!(trans("desktop.open_in_terminal"), "Mở trong Terminal");
    assert_eq!(trans("desktop.more"), "Thêm...");
    assert_eq!(trans("desktop.other_apps"), "Ứng dụng khác");
    assert_eq!(trans("desktop.back"), "Quay lại");
    assert_eq!(trans("explore.menu_open_with"), "Mở bằng...");
}

#[test]
fn missing_key_returns_key_itself() {
    let _guard = I18N_LOCK.lock().unwrap();
    set_locale("en");
    assert_eq!(trans("this.key.does.not.exist"), "this.key.does.not.exist");
}

#[test]
fn locale_switch_changes_language() {
    let _guard = I18N_LOCK.lock().unwrap();
    set_locale("en");
    let en = trans("menu.change_wallpaper");
    set_locale("vi");
    let vi = trans("menu.change_wallpaper");
    assert_eq!(en, "Change Wallpaper");
    assert_eq!(vi, "Thay đổi hình nền");
}

#[test]
fn set_locale_normalizes_unknown_to_vi() {
    let _guard = I18N_LOCK.lock().unwrap();
    set_locale("fr");
    assert_eq!(get_locale(), "vi");
}

#[test]
fn set_locale_roundtrips_between_en_and_vi() {
    let _guard = I18N_LOCK.lock().unwrap();
    set_locale("en");
    assert_eq!(get_locale(), "en");
    set_locale("vi");
    assert_eq!(get_locale(), "vi");
    set_locale("en");
    assert_eq!(get_locale(), "en");
}
