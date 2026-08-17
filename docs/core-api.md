# API Reference — `babydra-core`

**Phiên bản:** 1.0.0
**Ngày:** 2026-08-17
**Nguyên tắc:** `babydra-core` là logic thuần — **không phụ thuộc GTK**. Mọi struct dữ liệu đều serde; mọi lỗi đều là typed error (`CoreError`).

---

## 1. Tổng quan

`babydra-core` là thư viện lõi của BabyDra: services hệ thống (wifi, vpn, volume, battery, power, display…), models dữ liệu thuần, config tập trung và i18n. Các crate UI (kits, apps) phụ thuộc vào nó, không ngược lại.

- **Ngôn ngữ:** Rust (edition 2021)
- **Public API:** `libs/babydra-core/src/lib.rs` (re-export phẳng)
- **Error:** `babydra_core::error::{CoreError, CoreResult}` — dùng `thiserror`

---

## 2. Error handling

```rust
use babydra_core::{CoreError, CoreResult};

pub fn my_service() -> CoreResult<()> {
    std::fs::read_to_string("/tmp/x")?;                 // io::Error → CoreError::Io
    Err(CoreError::NotFound("file".into()))             // typed variant
}
```

| Variant | Ý nghĩa |
| :--- | :--- |
| `CoreError::Io(io::Error)` | Lỗi I/O (file, command spawn) |
| `CoreError::Command(String)` | Lệnh hệ thống thất bại |
| `CoreError::Invalid(String)` | Input người dùng không hợp lệ |
| `CoreError::NotFound(String)` | Tài nguyên không tồn tại |
| `CoreError::Message(String)` | Lỗi chung (fallback) |

`From<String>`, `From<&str>`, `From<io::Error>` được implement — dùng `?` hoặc `.into()` tự do.

---

## 3. Config

| API | Mô tả |
| :--- | :--- |
| `load_babydra_config() -> BabyDraConfig` | Đọc `babydra.conf` (cache `OnceLock`) |
| `save_babydra_config(&BabyDraConfig)` | Ghi config |
| `get_babydra_conf_path() / get_babydra_config_dir()` | Đường dẫn config |
| `load_explore_settings() / save_explore_settings()` | Settings riêng của explore |
| `apply_all_saved_settings()` | Áp dụng toàn bộ settings đã lưu |

Các struct: `BabyDraConfig`, `ExploreSettings`, `PowerConfig`, `NotificationConfig`, `WallpaperConfig`, `ShellConfig`, `ThemeConfig`.

---

## 4. Models

### Explore
`FileEntry`, `FileType`, `DirectoryModel`, `SortColumn`, `SortOrder`, `SessionState`, `ActivePane`, `TabState`, `get_group_name(&FileEntry, &str) -> String`

### Settings (data thuần — không widget)
`InstalledApp`, `InstalledPackage`, `CertInfo`, `MonitorConfig`, `EnvVar`, `Keybind`, `StartupCommand`, `PackageUpdate`, `SystemUpdateState`, `UpdateStatus`, `SystemInfoData`, `VpnConn`, `VpnConnDetails`, `WifiConfig`, `WifiNetwork`

> [!IMPORTANT]
> Các struct `*Widget` (GTK) **không còn nằm trong core** — chúng đã chuyển sang `crates/babydra-settings/src/widgets/state.rs` và `crates/babydra-explore/src/widgets/state.rs` (Phase 1).

### Shell
`BatteryInfo`, `PerformanceProfile`

### Screenshot
`EditorState`, `Drawing`, `Tool`

---

## 5. Services

### 5.1. i18n
| API | Mô tả |
| :--- | :--- |
| `t(key) -> String` | Tra cứu chuỗi theo locale (en/vi) |
| `set_locale(&str)` | Đổi locale (chuẩn hóa unknown → `vi`) |
| `get_locale() -> String` | Locale hiện tại |

### 5.2. Wallpaper
| API | Mô tả |
| :--- | :--- |
| `set_wallpaper(&Path) -> CoreResult<()>` | Đặt wallpaper qua backend (awww/swaybg/feh) |
| `get_current_wallpaper() -> Option<PathBuf>` | Đường dẫn wallpaper đang dùng |
| `set_greeter_wallpaper(&Path) -> CoreResult<()>` | Set nền greeter (base64) |
| `get_greeter_wallpaper_css() -> String` | CSS data-URL của nền greeter |
| `set_avatar(&Path) -> CoreResult<()>` | Set avatar (base64) |
| `get_avatar_bytes() -> Option<Vec<u8>>` | Bytes avatar |

> PIXBUF helpers (`crop_to_circle_pixbuf`…) đã chuyển sang UI layer (Phase 1) — xem kits.

### 5.3. System services
| Module | API tiêu biểu |
| :--- | :--- |
| `system::wifi` | `scan_networks`, `connect_wifi_async`, `get_wifi_state` |
| `system::vpn` | `get_vpn_connections`, `parse_vpn_config_file`, `save_vpn_connection` |
| `system::volume` | `parse_profile_parts`, `set_volume` |
| `system::battery` | `get_battery_info`, `set_charge_limit`, `set_charge_limit_auth`, `has_charge_limit_support` |
| `system::power` | `get_current_profile`, `set_performance_profile`, `apply_saved_profile`, `suspend`, `reboot`, `poweroff` |
| `system::display` | `get_displays`, `save_displays`, `apply_saved_displays` |
| `system::storage` | `DiskInfo`, helper `format_size`, `get_parent_drive` |
| `system::auth` | `verify_password(&str, &str) -> bool` (PAM) |
| `system::updates` | `check_updates`, `update_system`, `parse_pacman_progress_line`, `get_update_log_path` |
| `system::certificates` | `list_ca_certificates`, `add_ca_certificate`, `delete_ca_certificate` |
| `system::startup` | `save_startup_commands` |
| `system::theme` | `apply_appearance` (gsettings + labwc env) |
| `system::backlight` | `get_current_brightness`, `set_brightness` |

### 5.4. Apps & Explore
| API | Mô tả |
| :--- | :--- |
| `find_desktop_apps() -> Vec<DesktopApp>` | Quét .desktop files |
| `refresh_desktop_apps_cache()` | Refresh cache |
| `load_directory(path, show_hidden) -> io::Result<Vec<FileEntry>>` | Đọc thư mục async |
| `copy_path / move_path / delete_path / rename_path / send_to_trash` | Thao tác file async |
| `search_files(query)` | Tìm kiếm file |
| `parse_desktop_file(&Path) -> Option<DesktopApp>` | Parse .desktop |

### 5.5. Notification & Window
| API | Mô tả |
| :--- | :--- |
| `send_notification(title, msg)` | Gửi thông báo desktop |
| `send_settings_notification`, `send_app_notification` | Biến thể |
| `close_window / focus_window` | Quản lý window |
| `get_running_apps / get_history / save_history` | Window MRU |

### 5.6. Tray & Clock
| API | Mô tả |
| :--- | :--- |
| `tray::activate_item(service, x, y, is_right_click)` | Kích hoạt StatusNotifierItem |
| `tray::get_dbus_menu(service) -> Option<Vec<MenuItem>>` | Lấy menu D-Bus |
| `format_clock_date(key) -> (String, String)` | Format (time, date) — thuần, không GTK |

---

## 6. Ví dụ nhanh

```rust
use babydra_core::{load_babydra_config, send_notification, t, CoreError};

fn example() -> Result<(), CoreError> {
    let conf = load_babydra_config();
    println!("profile: {}", conf.power.profile);

    let (time, date) = babydra_core::format_clock_date("lock.date_format");
    println!("{time} {date}");

    if let Some(bytes) = babydra_core::get_avatar_bytes() {
        // avatar available
    }

    send_notification(&t("screenshot.copied_title"), &t("screenshot.copied_msg"));
    Ok(())
}
```

---

## 7. Tài liệu liên quan

- [planning.md](./planning.md) — Phase 1–2: GTK-free core + typed errors
- [06-kits-api.md](./06-kits-api.md) — API của UI kits
- [07-codebase-report.md](./07-codebase-report.md) — báo cáo đánh giá
