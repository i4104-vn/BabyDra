//! Integration tests: internationalization (i18n).
//!
//! Verifies locale-aware key lookup through the public `t()` API for both
//! the `en` and `vi` locales.

use babydra_core::i18n::{get_locale, set_locale, t};
use std::sync::Mutex;

// `t()` reads a process-global locale; serialize these tests to avoid races.
static I18N_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn english_lookup_returns_translations() {
    let _guard = I18N_LOCK.lock().unwrap();
    set_locale("en");
    assert_eq!(t("menu.terminal"), "Terminal");
    assert_eq!(t("panel.no_notifications"), "No new notifications");
}

#[test]
fn vietnamese_lookup_returns_translations() {
    let _guard = I18N_LOCK.lock().unwrap();
    set_locale("vi");
    assert_eq!(t("menu.file_manager"), "Trình quản lý tệp");
    assert_eq!(t("weekday.mon"), "Thứ Hai");
}

#[test]
fn missing_key_returns_key_itself() {
    let _guard = I18N_LOCK.lock().unwrap();
    set_locale("en");
    assert_eq!(t("this.key.does.not.exist"), "this.key.does.not.exist");
}

#[test]
fn locale_switch_changes_language() {
    let _guard = I18N_LOCK.lock().unwrap();
    set_locale("en");
    let en = t("menu.change_wallpaper");
    set_locale("vi");
    let vi = t("menu.change_wallpaper");
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
