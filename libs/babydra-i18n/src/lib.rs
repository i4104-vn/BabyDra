//! Internationalization (i18n) support library for the BabyDra workspace.
//! Provides locale management, translations, and string formatting utilities
//! for English ("en") and Vietnamese ("vi") locales.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

/// Returns the default system locale determined from the `LANG` environment variable.
fn default_locale() -> String {
    if let Ok(lang) = std::env::var("LANG") {
        if lang.to_lowercase().starts_with("vi") {
            return "vi".to_string();
        }
    }
    "en".to_string()
}

static CURRENT_LOCALE: OnceLock<RwLock<String>> = OnceLock::new();

/// Retrieves the current active system locale ("vi" or "en").
pub fn get_locale() -> String {
    let lock = CURRENT_LOCALE.get_or_init(|| {
        RwLock::new(default_locale())
    });
    lock.read().unwrap().clone()
}

/// Sets the current active system locale.
pub fn set_locale(locale: &str) {
    let normalized = if locale == "en" { "en" } else { "vi" };
    
    let lock = CURRENT_LOCALE.get_or_init(|| {
        RwLock::new(normalized.to_string())
    });
    if let Ok(mut writer) = lock.write() {
        *writer = normalized.to_string();
    }
}

/// Translates a given key into the current active locale's string.
/// If the key is not found, returns the key itself.
pub fn t(key: &str) -> String {
    let locale = get_locale();
    let dict = get_translations(&locale);
    dict.get(key)
        .map(|&s| s.to_string())
        .unwrap_or_else(|| key.to_string())
}

/// Retrives dictionary mappings for the given locale.
fn get_translations(locale: &str) -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    match locale {
        "en" => {
            // Menu
            map.insert("menu.terminal", "Terminal");
            map.insert("menu.file_manager", "File Manager");
            map.insert("menu.change_wallpaper", "Change Wallpaper");
            map.insert("menu.reconfigure_shell", "Reconfigure Shell");
            map.insert("menu.exit_shell", "Exit Shell");
            
            // Launcher
            map.insert("launcher.search_placeholder", "Search apps or files...");
            map.insert("launcher.welcome", "Type keywords to search apps and files...");
            map.insert("launcher.apps", "Applications");
            map.insert("launcher.files", "Files");
            map.insert("launcher.no_results", "No matching results found");
            map.insert("launcher.google_search", "Search Google for \"{}\"");
            map.insert("launcher.shutdown", "Shut Down");
            map.insert("launcher.restart", "Restart");
            map.insert("launcher.suspend", "Suspend");

            // Panel / Clock
            map.insert("panel.no_notifications", "No new notifications");
            map.insert("panel.notifications", "Notifications");
            map.insert("panel.storage_usage", "Storage Usage");
            map.insert("panel.no_storage", "No physical storage found");
            map.insert("panel.system", "System");
            map.insert("panel.clear_all", "Clear All");
            map.insert("panel.just_now", "Just now");
            map.insert("panel.minutes_ago", "{}m ago");
            map.insert("panel.hours_ago", "{}h ago");
            map.insert("panel.days_ago", "{}d ago");
            map.insert("panel.system_resources", "System Resources");
            map.insert("panel.cpu_load", "CPU Load");
            map.insert("panel.ram_usage", "RAM Usage");

            // Control center
            map.insert("control.network", "Network");
            map.insert("control.connected", "Connected");
            map.insert("control.bluetooth", "Bluetooth");
            map.insert("control.not_connected", "Not Connected");
            map.insert("control.dnd", "Do Not Disturb");
            map.insert("control.on", "On");
            map.insert("control.off", "Off");
            map.insert("control.dark_mode", "Dark\nMode");
            map.insert("control.night_light", "Night\nColor");
            map.insert("control.title", "Control Center");
            map.insert("control.lang_changed_title", "Language Changed");
            map.insert("control.lang_changed_msg", "Restart widgets to apply changes system-wide.");
            map.insert("control.switch_language", "Switch Language / Đổi ngôn ngữ");

            // Screenshot
            map.insert("screenshot.reset_tooltip", "Discard and restart (Clear all drawings)");
            map.insert("screenshot.pen_tooltip", "Pen");
            map.insert("screenshot.rect_tooltip", "Draw rectangle");
            map.insert("screenshot.blur_tooltip", "Blur information");
            map.insert("screenshot.eraser_tooltip", "Erase drawings");
            map.insert("screenshot.color_tooltip", "Select drawing color");
            map.insert("screenshot.copy_tooltip", "Copy to Clipboard (Enter)");
            map.insert("screenshot.save_tooltip", "Save screenshot (Ctrl+S)");
            map.insert("screenshot.cancel_tooltip", "Cancel (Escape)");
            map.insert("screenshot.copied_title", "Copied Screenshot");
            map.insert("screenshot.copied_msg", "Screenshot has been saved to your clipboard.");
            map.insert("screenshot.saved_title", "Screenshot Saved");
            map.insert("screenshot.saved_msg", "Saved to {}");
            map.insert("screenshot.full_saved_title", "Fullscreen Captured");

            // Colors
            map.insert("color.red", "Red");
            map.insert("color.orange", "Orange");
            map.insert("color.yellow", "Yellow");
            map.insert("color.green", "Green");
            map.insert("color.blue", "Blue");
            map.insert("color.purple", "Purple");
            map.insert("color.white", "White");
            map.insert("color.black", "Black");

            // Lock Screen
            map.insert("lock.placeholder", "Enter password to unlock...");
            map.insert("lock.status", "Device is locked");
            map.insert("lock.status_incorrect", "Incorrect password! Try again.");
            map.insert("lock.date_format", "{weekday}, {month} {day}, {year}");

            // Date formats & values
            map.insert("panel.date_format", "{weekday}, {month} {day}");
            map.insert("weekday.mon", "Monday");
            map.insert("weekday.tue", "Tuesday");
            map.insert("weekday.wed", "Wednesday");
            map.insert("weekday.thu", "Thursday");
            map.insert("weekday.fri", "Friday");
            map.insert("weekday.sat", "Saturday");
            map.insert("weekday.sun", "Sunday");
            map.insert("weekday.today", "Today");

            map.insert("month.01", "January");
            map.insert("month.02", "February");
            map.insert("month.03", "March");
            map.insert("month.04", "April");
            map.insert("month.05", "May");
            map.insert("month.06", "June");
            map.insert("month.07", "July");
            map.insert("month.08", "August");
            map.insert("month.09", "September");
            map.insert("month.10", "October");
            map.insert("month.11", "November");
            map.insert("month.12", "December");

            // Taskbar
            map.insert("taskbar.tasks", "Tasks");
            map.insert("taskbar.close_all", "Close All");
        }
        _ => {
            // Menu
            map.insert("menu.terminal", "Terminal");
            map.insert("menu.file_manager", "Trình quản lý tệp");
            map.insert("menu.change_wallpaper", "Thay đổi hình nền");
            map.insert("menu.reconfigure_shell", "Cấu hình lại Shell");
            map.insert("menu.exit_shell", "Thoát Shell");

            // Launcher
            map.insert("launcher.search_placeholder", "Tìm ứng dụng hoặc tệp tin...");
            map.insert("launcher.welcome", "Nhập từ khóa để tìm kiếm ứng dụng và tệp tin...");
            map.insert("launcher.apps", "Ứng dụng");
            map.insert("launcher.files", "Tập tin");
            map.insert("launcher.no_results", "Không tìm thấy kết quả phù hợp");
            map.insert("launcher.google_search", "Tìm trên Google cho \"{}\"");
            map.insert("launcher.shutdown", "Tắt máy");
            map.insert("launcher.restart", "Khởi động lại");
            map.insert("launcher.suspend", "Tạm dừng");

            // Panel / Clock
            map.insert("panel.no_notifications", "Không có thông báo mới");
            map.insert("panel.notifications", "Thông báo");
            map.insert("panel.storage_usage", "Dung lượng đĩa");
            map.insert("panel.no_storage", "Không tìm thấy ổ lưu trữ");
            map.insert("panel.system", "Hệ thống");
            map.insert("panel.clear_all", "Xóa tất cả");
            map.insert("panel.just_now", "Vừa xong");
            map.insert("panel.minutes_ago", "{} phút trước");
            map.insert("panel.hours_ago", "{} giờ trước");
            map.insert("panel.days_ago", "{} ngày trước");
            map.insert("panel.system_resources", "Tài nguyên Hệ thống");
            map.insert("panel.cpu_load", "Tải CPU");
            map.insert("panel.ram_usage", "Sử dụng RAM");

            // Control center
            map.insert("control.network", "Mạng");
            map.insert("control.connected", "Đã kết nối");
            map.insert("control.bluetooth", "Bluetooth");
            map.insert("control.not_connected", "Chưa kết nối");
            map.insert("control.dnd", "Chế độ không làm phiền");
            map.insert("control.on", "Bật");
            map.insert("control.off", "Tắt");
            map.insert("control.dark_mode", "Chế độ\nTối");
            map.insert("control.night_light", "Ánh sáng\nĐêm");
            map.insert("control.title", "Trung tâm Điều khiển");
            map.insert("control.lang_changed_title", "Đã thay đổi ngôn ngữ");
            map.insert("control.lang_changed_msg", "Khởi động lại widgets để áp dụng toàn hệ thống.");
            map.insert("control.switch_language", "Switch Language / Đổi ngôn ngữ");

            // Screenshot
            map.insert("screenshot.reset_tooltip", "Bỏ chụp và làm lại (Xóa hết nét vẽ)");
            map.insert("screenshot.pen_tooltip", "Bút vẽ");
            map.insert("screenshot.rect_tooltip", "Vẽ hình chữ nhật");
            map.insert("screenshot.blur_tooltip", "Làm mờ thông tin");
            map.insert("screenshot.eraser_tooltip", "Xóa hình vẽ");
            map.insert("screenshot.color_tooltip", "Chọn màu vẽ");
            map.insert("screenshot.copy_tooltip", "Sao chép vào Clipboard (Enter)");
            map.insert("screenshot.save_tooltip", "Lưu ảnh chụp (Ctrl+S)");
            map.insert("screenshot.cancel_tooltip", "Hủy (Escape)");
            map.insert("screenshot.copied_title", "Đã sao chép ảnh");
            map.insert("screenshot.copied_msg", "Ảnh chụp đã được lưu vào clipboard.");
            map.insert("screenshot.saved_title", "Đã chụp ảnh màn hình");
            map.insert("screenshot.saved_msg", "Đã lưu tại {}");
            map.insert("screenshot.full_saved_title", "Đã chụp toàn màn hình");

            // Colors
            map.insert("color.red", "Đỏ");
            map.insert("color.orange", "Cam");
            map.insert("color.yellow", "Vàng");
            map.insert("color.green", "Lục");
            map.insert("color.blue", "Lam");
            map.insert("color.purple", "Tím");
            map.insert("color.white", "Trắng");
            map.insert("color.black", "Đen");

            // Lock Screen
            map.insert("lock.placeholder", "Nhập mật khẩu để mở khóa...");
            map.insert("lock.status", "Thiết bị đang bị khóa");
            map.insert("lock.status_incorrect", "Mật khẩu không chính xác! Thử lại.");
            map.insert("lock.date_format", "{weekday}, {day} tháng {month}, {year}");

            // Date formats & values
            map.insert("panel.date_format", "{weekday}, {day} tháng {month}");
            map.insert("weekday.mon", "Thứ Hai");
            map.insert("weekday.tue", "Thứ Ba");
            map.insert("weekday.wed", "Thứ Tư");
            map.insert("weekday.thu", "Thứ Năm");
            map.insert("weekday.fri", "Thứ Sáu");
            map.insert("weekday.sat", "Thứ Bảy");
            map.insert("weekday.sun", "Chủ Nhật");
            map.insert("weekday.today", "Hôm nay");

            map.insert("month.01", "01");
            map.insert("month.02", "02");
            map.insert("month.03", "03");
            map.insert("month.04", "04");
            map.insert("month.05", "05");
            map.insert("month.06", "06");
            map.insert("month.07", "07");
            map.insert("month.08", "08");
            map.insert("month.09", "09");
            map.insert("month.10", "10");
            map.insert("month.11", "11");
            map.insert("month.12", "12");

            // Taskbar
            map.insert("taskbar.tasks", "Cửa sổ");
            map.insert("taskbar.close_all", "Đóng tất cả");
        }
    }
    map
}

