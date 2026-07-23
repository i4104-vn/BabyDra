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
            map.insert("launcher.other_apps", "Other Applications");
            map.insert("launcher.files", "Files");
            map.insert("launcher.no_results", "No matching results found");
            map.insert("launcher.google_search", "Search Google for \"{}\"");
            map.insert("launcher.shutdown", "Shut Down");
            map.insert("launcher.restart", "Restart");
            map.insert("launcher.suspend", "Suspend");
            map.insert("launcher.logout", "Log Out");

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
            map.insert("control.clean", "Clean");
            map.insert("control.clean_my_linux", "Clean My Linux");
            map.insert("control.scan", "Scan");
            map.insert("control.free", "Free");
            map.insert("control.scanning", "Scanning...");
            map.insert("control.freed_success", "Successfully freed {}!");
            map.insert("control.nothing_to_free", "Nothing to free.");
            map.insert("control.bytes_can_be_freed", "{} can be freed");

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

            // Explore / File Explorer
            map.insert("explore.new_folder", "New Folder");
            map.insert("explore.empty_trash", "Empty Trash");
            map.insert("explore.cut", "Cut");
            map.insert("explore.copy", "Copy");
            map.insert("explore.paste", "Paste");
            map.insert("explore.rename", "Rename");
            map.insert("explore.delete", "Delete");
            map.insert("explore.view_grid", "View as Grid");
            map.insert("explore.view_list", "View as List");
            map.insert("explore.search_placeholder", "Search");
            map.insert("explore.sort_by", "Sort by");
            map.insert("explore.sort_auto", "Auto");
            map.insert("explore.sort_date", "By Date");
            map.insert("explore.sort_group", "By Group");
            map.insert("explore.places", "Places");
            map.insert("explore.this_pc", "This PC");
            map.insert("explore.home", "Home");
            map.insert("explore.downloads", "Downloads");
            map.insert("explore.documents", "Documents");
            map.insert("explore.pictures", "Pictures");
            map.insert("explore.music", "Music");
            map.insert("explore.videos", "Videos");
            map.insert("explore.desktop", "Desktop");
            map.insert("explore.trash", "Trash");
            map.insert("explore.local_disk", "Local Disk (/)");
            map.insert("explore.toggle_hidden", "Toggle Hidden Files (Ctrl+H)");
            map.insert("explore.toggle_preview", "Toggle Preview (F4)");
            map.insert("explore.preview_no_selection", "Select a file to preview");
            map.insert("explore.previewing", "Previewing: {}");
            map.insert("explore.preview_too_large", "[File is too large to preview (> 1MB)]");
            map.insert("explore.preview_failed", "[Failed to load file contents / Binary file]");
            map.insert("explore.items", "items");
            map.insert("explore.total_size", "Total size");
            map.insert("explore.back", "Back");
            map.insert("explore.forward", "Forward");
            map.insert("explore.up", "Up");
            map.insert("explore.refresh", "Refresh");
            map.insert("explore.loading", "Loading directory...");
            map.insert("explore.settings", "Settings");
            map.insert("explore.settings_general", "General");
            map.insert("explore.settings_keybinds", "Shortcuts");
            map.insert("explore.keybinds_title", "Keyboard Shortcuts");
            map.insert("explore.shortcut_open", "Open file or folder");
            map.insert("explore.shortcut_split", "Toggle split view (dual pane)");
            map.insert("explore.shortcut_preview", "Toggle preview panel");
            map.insert("explore.shortcut_hidden", "Toggle hidden files");
            map.insert("explore.shortcut_copy", "Copy selected items");
            map.insert("explore.shortcut_cut", "Cut selected items");
            map.insert("explore.shortcut_paste", "Paste items from clipboard");
            map.insert("explore.settings_behavior", "Behavior");
            map.insert("explore.settings_double_click", "Double-click to open");
            map.insert("explore.settings_permanent_delete", "Permanently delete immediately");
            map.insert("explore.settings_calculate_size", "Calculate folder sizes");
            map.insert("explore.settings_context_menu", "Context Menu");
            map.insert("explore.settings_custom_options", "Custom Context Options");
            map.insert("explore.settings_add_option", "Add Option");
            map.insert("explore.settings_option_name", "Option Name");
            map.insert("explore.settings_option_command", "Command");
            map.insert("explore.settings_add", "Add");
            map.insert("explore.settings_delete", "Delete");
            map.insert("explore.settings_placeholder_name", "e.g., Open with Code");
            map.insert("explore.settings_placeholder_command", "e.g., code {path}");
            map.insert("explore.settings_toggle_hidden_desc", "Show files and folders that start with a dot (.)");
            map.insert("explore.settings_toggle_preview_desc", "Show detailed file previews on the right sidebar (F4)");
            map.insert("explore.settings_double_click_desc", "Requires two clicks to open files and folders");
            map.insert("explore.settings_permanent_delete_desc", "Bypass the Trash bin and delete files permanently");
            map.insert("explore.settings_calculate_size_desc", "Show total size of directories in list view (may impact performance)");
            map.insert("explore.settings_edit", "Edit");
            map.insert("explore.settings_save", "Save");
            map.insert("explore.settings_cancel", "Cancel");
            map.insert("explore.settings_capture_title", "Record Shortcut");
            map.insert("explore.settings_close", "Close");
            map.insert("explore.placeholder_path_desc", "Full path of the selected item");
            map.insert("explore.placeholder_dir_desc", "Parent directory of the selected item (or current directory)");
            map.insert("explore.placeholder_name_desc", "File/Folder name with extension");
            map.insert("explore.placeholder_stem_desc", "File/Folder name without extension");
            map.insert("explore.placeholder_ext_desc", "File extension");

            // Context Menu & Dialogs
            map.insert("explore.menu_open_new_window", "Open in New Window");
            map.insert("explore.menu_copy_location", "Copy Location");
            map.insert("explore.menu_open", "Open");
            map.insert("explore.menu_compress", "Compress...");
            map.insert("explore.menu_decompress", "Decompress");
            map.insert("explore.menu_new", "New");
            map.insert("explore.menu_new_folder", "Folder");
            map.insert("explore.menu_new_file", "File");
            map.insert("explore.menu_cut", "Cut");
            map.insert("explore.menu_copy", "Copy");
            map.insert("explore.menu_paste", "Paste");
            map.insert("explore.menu_rename", "Rename");
            map.insert("explore.menu_trash", "Move to Trash");
            map.insert("explore.menu_restore", "Restore");
            map.insert("explore.menu_delete_perm", "Delete Permanently");

            map.insert("explore.dialog_rename_title", "Rename File");
            map.insert("explore.dialog_rename_label", "Enter new name:");
            map.insert("explore.dialog_new_file_title", "New File");
            map.insert("explore.dialog_new_file_label", "Enter file name:");
            map.insert("explore.dialog_new_folder_title", "New Folder");
            map.insert("explore.dialog_new_folder_label", "Enter folder name:");
            map.insert("explore.dialog_archive_title", "Compress File");
            map.insert("explore.dialog_archive_label", "Enter archive name:");
            map.insert("explore.dialog_confirm_title", "Are you sure?");
            map.insert("explore.dialog_confirm_body", "This action cannot be undone.");
            map.insert("explore.dialog_confirm_delete_title", "Confirm Delete");
            map.insert("explore.dialog_confirm_delete_single", "Are you sure you want to permanently delete '{}'?");
            map.insert("explore.dialog_confirm_delete_multi", "Are you sure you want to permanently delete these {} items?");
            map.insert("explore.dialog_password_title", "Enter Password");
            map.insert("explore.dialog_password_label", "This archive is password-protected. Please enter password:");
            map.insert("explore.dialog_password_incorrect", "Incorrect password. Please try again.");
            map.insert("explore.dialog_decompress_title", "Decompression Log");
            map.insert("explore.decompressing_running", "Decompressing...");
            map.insert("explore.decompress_success", "Decompression Succeeded");
            map.insert("explore.decompress_failed", "Decompression Failed");
            map.insert("explore.copying_title", "Copying Files");
            map.insert("explore.moving_title", "Moving Files");
            map.insert("explore.menu_refresh", "Refresh");
            map.insert("explore.menu_properties", "Properties");
            map.insert("explore.dialog_properties_title", "Properties");
            map.insert("explore.dialog_conflict_title", "Item Already Exists");
            map.insert("explore.dialog_conflict_msg", "An item named '{}' already exists in this location. Do you want to replace it?");
            map.insert("explore.dialog_override", "Replace");
            map.insert("explore.error_file_exists", "A file with this name already exists.");
            map.insert("explore.error_folder_exists", "A folder with this name already exists.");
            map.insert("explore.error_item_exists", "An item with this name already exists.");
            map.insert("explore.prop_folder", "Folder");
            map.insert("explore.prop_symlink", "Symlink");
            map.insert("explore.prop_file", "File");
            map.insert("explore.prop_general_info", "GENERAL INFORMATION");
            map.insert("explore.prop_selection_details", "SELECTION DETAILS");
            map.insert("explore.prop_size", "Size:");
            map.insert("explore.prop_total_size", "Total Size:");
            map.insert("explore.prop_calculating", "Calculating...");
            map.insert("explore.prop_contents", "Contents:");
            map.insert("explore.prop_counting", "Counting...");
            map.insert("explore.prop_created", "Created:");
            map.insert("explore.prop_modified", "Modified:");
            map.insert("explore.prop_owner_group", "Owner/Group:");
            map.insert("explore.prop_count", "Count:");
            map.insert("explore.prop_location", "Location:");
            map.insert("explore.prop_contents_format", "{} files, {} folders");
            map.insert("explore.prop_selected_items", "{} Selected Items");
            map.insert("explore.prop_items_count", "{} items");
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
            map.insert("launcher.other_apps", "Ứng dụng khác");
            map.insert("launcher.files", "Tập tin");
            map.insert("launcher.no_results", "Không tìm thấy kết quả phù hợp");
            map.insert("launcher.google_search", "Tìm trên Google cho \"{}\"");
            map.insert("launcher.shutdown", "Tắt máy");
            map.insert("launcher.restart", "Khởi động lại");
            map.insert("launcher.suspend", "Tạm dừng");
            map.insert("launcher.logout", "Đăng xuất");

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
            map.insert("control.clean", "Dọn dẹp");
            map.insert("control.clean_my_linux", "Dọn dẹp Linux");
            map.insert("control.scan", "Quét");
            map.insert("control.free", "Giải phóng");
            map.insert("control.scanning", "Đang quét...");
            map.insert("control.freed_success", "Đã giải phóng {}!");
            map.insert("control.nothing_to_free", "Không có tệp thừa.");
            map.insert("control.bytes_can_be_freed", "Có thể giải phóng {}");

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

            // Explore / File Explorer
            map.insert("explore.new_folder", "Tạo thư mục mới");
            map.insert("explore.empty_trash", "Xóa sạch thùng rác");
            map.insert("explore.cut", "Cắt");
            map.insert("explore.copy", "Sao chép");
            map.insert("explore.paste", "Dán");
            map.insert("explore.rename", "Đổi tên");
            map.insert("explore.delete", "Xóa");
            map.insert("explore.view_grid", "Xem dạng lưới");
            map.insert("explore.view_list", "Xem dạng danh sách");
            map.insert("explore.search_placeholder", "Tìm kiếm");
            map.insert("explore.sort_by", "Sắp xếp theo");
            map.insert("explore.sort_auto", "Tự động");
            map.insert("explore.sort_date", "Theo ngày");
            map.insert("explore.sort_group", "Theo nhóm");
            map.insert("explore.places", "Vị trí");
            map.insert("explore.this_pc", "Máy tính này");
            map.insert("explore.home", "Thư mục cá nhân");
            map.insert("explore.downloads", "Tải về");
            map.insert("explore.documents", "Tài liệu");
            map.insert("explore.pictures", "Hình ảnh");
            map.insert("explore.music", "Nhạc");
            map.insert("explore.videos", "Video");
            map.insert("explore.desktop", "Màn hình nền");
            map.insert("explore.trash", "Thùng rác");
            map.insert("explore.local_disk", "Ổ đĩa cục bộ (/)");
            map.insert("explore.toggle_hidden", "Ẩn/hiện tệp ẩn (Ctrl+H)");
            map.insert("explore.toggle_preview", "Ẩn/hiện xem trước (F4)");
            map.insert("explore.preview_no_selection", "Chọn một tệp để xem trước");
            map.insert("explore.previewing", "Đang xem trước: {}");
            map.insert("explore.preview_too_large", "[Tệp quá lớn để xem trước (> 1MB)]");
            map.insert("explore.preview_failed", "[Không tải được nội dung tệp / Tệp nhị phân]");
            map.insert("explore.items", "đối tượng");
            map.insert("explore.total_size", "Tổng dung lượng");
            map.insert("explore.back", "Quay lại");
            map.insert("explore.forward", "Đi tiếp");
            map.insert("explore.up", "Lên thư mục cha");
            map.insert("explore.refresh", "Tải lại");
            map.insert("explore.loading", "Đang tải thư mục...");
            map.insert("explore.settings", "Cài đặt");
            map.insert("explore.settings_general", "Chung");
            map.insert("explore.settings_keybinds", "Phím tắt");
            map.insert("explore.keybinds_title", "Phím tắt bàn phím");
            map.insert("explore.shortcut_open", "Mở tệp hoặc thư mục");
            map.insert("explore.shortcut_split", "Bật/tắt xem hai bảng (Split View)");
            map.insert("explore.shortcut_preview", "Bật/tắt bảng xem trước");
            map.insert("explore.shortcut_hidden", "Ẩn/hiện tệp ẩn");
            map.insert("explore.shortcut_copy", "Sao chép tệp đã chọn");
            map.insert("explore.shortcut_cut", "Cắt tệp đã chọn");
            map.insert("explore.shortcut_paste", "Dán tệp từ bộ nhớ tạm");
            map.insert("explore.settings_behavior", "Hành vi");
            map.insert("explore.settings_double_click", "Nhấp đúp để mở");
            map.insert("explore.settings_permanent_delete", "Xóa vĩnh viễn trực tiếp");
            map.insert("explore.settings_calculate_size", "Tính dung lượng thư mục");
            map.insert("explore.settings_context_menu", "Menu chuột phải");
            map.insert("explore.settings_custom_options", "Tùy chọn menu chuột phải");
            map.insert("explore.settings_add_option", "Thêm tùy chọn");
            map.insert("explore.settings_option_name", "Tên tùy chọn");
            map.insert("explore.settings_option_command", "Lệnh thực thi");
            map.insert("explore.settings_add", "Thêm");
            map.insert("explore.settings_delete", "Xóa");
            map.insert("explore.settings_placeholder_name", "Ví dụ: Mở bằng Code");
            map.insert("explore.settings_placeholder_command", "Ví dụ: code {path}");
            map.insert("explore.settings_toggle_hidden_desc", "Hiển thị các tệp và thư mục bắt đầu bằng dấu chấm (.)");
            map.insert("explore.settings_toggle_preview_desc", "Hiển thị bảng xem chi tiết tệp ở thanh bên phải (F4)");
            map.insert("explore.settings_double_click_desc", "Yêu cầu nhấp chuột hai lần để mở tệp và thư mục");
            map.insert("explore.settings_permanent_delete_desc", "Bỏ qua Thùng rác và xóa tệp vĩnh viễn ngay lập tức");
            map.insert("explore.settings_calculate_size_desc", "Hiển thị tổng dung lượng của thư mục trong danh sách (có thể làm chậm)");
            map.insert("explore.settings_edit", "Chỉnh sửa");
            map.insert("explore.settings_save", "Lưu");
            map.insert("explore.settings_cancel", "Hủy");
            map.insert("explore.settings_capture_title", "Ghi nhận phím tắt");
            map.insert("explore.settings_close", "Đóng");
            map.insert("explore.placeholder_path_desc", "Đường dẫn đầy đủ của đối tượng được chọn");
            map.insert("explore.placeholder_dir_desc", "Thư mục cha của đối tượng được chọn (hoặc thư mục hiện tại)");
            map.insert("explore.placeholder_name_desc", "Tên tệp/thư mục kèm theo phần mở rộng");
            map.insert("explore.placeholder_stem_desc", "Tên tệp/thư mục không kèm phần mở rộng");
            map.insert("explore.placeholder_ext_desc", "Phần mở rộng của tệp");

            // Context Menu & Dialogs
            map.insert("explore.menu_open_new_window", "Mở trong cửa sổ mới");
            map.insert("explore.menu_copy_location", "Sao chép đường dẫn");
            map.insert("explore.menu_open", "Mở");
            map.insert("explore.menu_compress", "Nén...");
            map.insert("explore.menu_decompress", "Giải nén");
            map.insert("explore.menu_new", "Tạo mới");
            map.insert("explore.menu_new_folder", "Thư mục");
            map.insert("explore.menu_new_file", "Tệp tin");
            map.insert("explore.menu_cut", "Cắt");
            map.insert("explore.menu_copy", "Sao chép");
            map.insert("explore.menu_paste", "Dán");
            map.insert("explore.menu_rename", "Đổi tên");
            map.insert("explore.menu_trash", "Di chuyển vào Thùng rác");
            map.insert("explore.menu_restore", "Khôi phục");
            map.insert("explore.menu_delete_perm", "Xóa vĩnh viễn");

            map.insert("explore.dialog_rename_title", "Đổi tên tệp");
            map.insert("explore.dialog_rename_label", "Nhập tên mới:");
            map.insert("explore.dialog_new_file_title", "Tệp mới");
            map.insert("explore.dialog_new_file_label", "Nhập tên tệp:");
            map.insert("explore.dialog_new_folder_title", "Thư mục mới");
            map.insert("explore.dialog_new_folder_label", "Nhập tên thư mục:");
            map.insert("explore.dialog_archive_title", "Nén tệp");
            map.insert("explore.dialog_archive_label", "Nhập tên tệp nén:");
            map.insert("explore.dialog_confirm_title", "Bạn có chắc chắn không?");
            map.insert("explore.dialog_confirm_body", "Hành động này không thể hoàn tác.");
            map.insert("explore.dialog_confirm_delete_title", "Xác nhận xóa");
            map.insert("explore.dialog_confirm_delete_single", "Bạn có chắc chắn muốn xóa vĩnh viễn '{}' không?");
            map.insert("explore.dialog_confirm_delete_multi", "Bạn có chắc chắn muốn xóa vĩnh viễn {} đối tượng này không?");
            map.insert("explore.dialog_password_title", "Nhập mật khẩu");
            map.insert("explore.dialog_password_label", "Tệp này được bảo vệ bằng mật khẩu. Vui lòng nhập mật khẩu:");
            map.insert("explore.dialog_password_incorrect", "Mật khẩu không chính xác. Vui lòng thử lại.");
            map.insert("explore.dialog_decompress_title", "Nhật ký giải nén");
            map.insert("explore.decompressing_running", "Đang giải nén...");
            map.insert("explore.decompress_success", "Giải nén thành công");
            map.insert("explore.decompress_failed", "Giải nén thất bại");
            map.insert("explore.copying_title", "Đang sao chép tập tin");
            map.insert("explore.moving_title", "Đang di chuyển tập tin");
            map.insert("explore.menu_refresh", "Làm mới");
            map.insert("explore.menu_properties", "Thuộc tính");
            map.insert("explore.dialog_properties_title", "Thuộc tính");
            map.insert("explore.dialog_conflict_title", "Mục đã tồn tại");
            map.insert("explore.dialog_conflict_msg", "Mục '{}' đã tồn tại tại vị trí này. Bạn có muốn ghi đè lên nó không?");
            map.insert("explore.dialog_override", "Ghi đè");
            map.insert("explore.error_file_exists", "Tệp với tên này đã tồn tại.");
            map.insert("explore.error_folder_exists", "Thư mục với tên này đã tồn tại.");
            map.insert("explore.error_item_exists", "Mục với tên này đã tồn tại.");
            map.insert("explore.prop_folder", "Thư mục");
            map.insert("explore.prop_symlink", "Liên kết mềm");
            map.insert("explore.prop_file", "Tệp");
            map.insert("explore.prop_general_info", "THÔNG TIN CHUNG");
            map.insert("explore.prop_selection_details", "CHI TIẾT MỤC ĐÃ CHỌN");
            map.insert("explore.prop_size", "Dung lượng:");
            map.insert("explore.prop_total_size", "Tổng dung lượng:");
            map.insert("explore.prop_calculating", "Đang tính toán...");
            map.insert("explore.prop_contents", "Nội dung:");
            map.insert("explore.prop_counting", "Đang đếm...");
            map.insert("explore.prop_created", "Ngày tạo:");
            map.insert("explore.prop_modified", "Ngày sửa:");
            map.insert("explore.prop_owner_group", "Chủ sở hữu/Nhóm:");
            map.insert("explore.prop_count", "Số lượng:");
            map.insert("explore.prop_location", "Vị trí:");
            map.insert("explore.prop_contents_format", "{} tệp, {} thư mục");
            map.insert("explore.prop_selected_items", "{} mục đã chọn");
            map.insert("explore.prop_items_count", "{} đối tượng");
        }
    }
    map
}
