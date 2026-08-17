# Luồng hoạt động — `babydra-core`

**Phạm vi:** Cách các service/helper của `babydra-core` vận hành và được gọi từ các crate UI.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Vị trí trong hệ thống](#1-vị-trí-trong-hệ-thống)
- [2. Luồng gọi từ tầng View](#2-luồng-gọi-từ-tầng-view)
- [3. Service chạy nền (daemon)](#3-service-chạy-nền-daemon)
- [4. Luồng config & i18n](#4-luồng-config--i18n)
- [5. Luồng apply_all_saved_settings](#5-luồng-apply_all_saved_settings)
- [6. Re-export phẳng & helper module](#6-re-export-phẳng--helper-module)

---

## 1. Vị trí trong hệ thống

`babydra-core` là tầng **Engine** duy nhất — không import GTK, không biết gì về UI. Mọi crate (panel, settings, explore, island...) gọi API qua đây. Chi tiết pattern: [architecture](../architecture/index.md) mục 2.

```text
Crate UI (panel, settings, explore, island, ...)
   │  gọi hàm
   ▼
babydra-core
   ├── services/     ── nghiệp vụ hệ thống (battery, wifi, vpn, volume, ...)
   ├── models/       ── dữ liệu thuần (serde)
   ├── config/       ── babydra.conf
   ├── i18n/         ── dịch chuỗi en/vi
   └── error/        ── CoreError / CoreResult
```

---

## 2. Luồng gọi từ tầng View

Mọi tương tác hệ thống tuân theo mẫu: **UI gọi hàm core → core đọc/ghi hệ thống → trả dữ liệu → UI render**.

Ví dụ kéo thanh trượt âm lượng:

```text
panel (items/volume/render.rs)
  └─ set_volume(value)                        ── services::system::volume
       └─ ghi PipeWire / WirePlumber
```

Ví dụ bật WiFi:

```text
panel (popover/network.rs)
  └─ wifi::connect(ssid, password)            ── services::system::wifi
       └─ NetworkManager qua D-Bus
```

| Nhóm gọi | API tiêu biểu | Bên dưới |
| :--- | :--- | :--- |
| Apps | `find_desktop_apps()`, `refresh_desktop_apps_cache()` | Quét `.desktop` files |
| Explore | `load_directory()`, `copy_path()`, `delete_path()`, `search_files()` | `services::explore` (async) |
| Power | `set_performance_profile()`, `suspend()`, `reboot()`, `poweroff()` | `services::system::power` |
| Wallpaper | `set_wallpaper()`, `set_greeter_wallpaper()`, `set_avatar()` | `services::wallpaper` (awww/swaybg/feh + base64) |
| Notification | `send_notification()`, `send_settings_notification()` | `services::notification::service` |
| Window | `close_window()`, `focus_window()`, `get_running_apps()` | `services::window` |

---

## 3. Service chạy nền (daemon)

Một số service phải **chạy sẵn** — crate UI spawn chúng trong `main()` trước khi vào GTK loop:

| Service | Spawn bởi | Chức năng | Thoát khi |
| :--- | :--- | :--- | :--- |
| `tray::spawn_watcher_service()` | panel `main()` | D-Bus StatusNotifierWatcher cho system tray | process exit |
| `spawn_switcher_tracker()` | panel `main()` | Thread theo dõi window focus (cho switcher) | process exit |
| `refresh_desktop_apps_cache()` | panel `main()` (thread riêng) | Cache danh sách app bất đồng bộ | xong 1 lần |
| `notification::spawn_dbus_listener()` | island (notification feature) | Host `org.freedesktop.Notifications` | channel đóng |
| `explore::start_dbus_service()` | explore | D-Bus service cho file operations | process exit |

Luồng notification từ đầu đến island:

```text
app gửi send_notification(title, msg)
  → babydra_core::services::notification::service
  → (D-Bus message)
  → island: spawn_dbus_listener nhận → NotificationMsg::New
  → main-thread task → show_notification_popup → SHARED_NOTIFICATION
  → notification feature tick đọc → hiển thị trên island
```

Chi tiết phía island: [flows/island.md](./island.md) mục notification.

---

## 4. Luồng config & i18n

### 4.1. Config

```text
load_babydra_config()
  → đọc ~/.babydra/babydra.conf (cache OnceLock)
  → BabyDraConfig { theme, power, notification, wallpaper, shell, explore }
save_babydra_config(&conf)
  → ghi file
```

- `theme.selection.id` — theme đang chọn (được `babydra-ui-kit::init_theme` đọc).
- `theme.selection.dark` — `Some(bool)` ép mode, `None` = theo GSettings.

### 4.2. i18n

```text
t("namespace.key")
  → đọc locale hiện tại (en/vi)
  → tra file locales/<app>/{en,vi}.json
  → trả chuỗi (fallback: key nếu thiếu)
```

- `watch_locale_change(cb)` — panel đăng ký để rebuild khi đổi locale.
- Mọi chuỗi UI phải qua `t()` — không hardcode.

---

## 5. Luồng apply_all_saved_settings

Được `babydra-settings --apply-all-settings` gọi (khởi động / sau login):

```text
apply_all_saved_settings()
  1. apply_saved_profile()            ── CPU performance profile
  2. apply_saved_displays()           ── resolution, refresh, position, scale
  3. apply_saved_wallpaper()          ── wallpaper
  4. apply_saved_greeter_wallpaper()  ── greeter/lock wallpaper → world-readable path
  5. check_and_apply_auto_battery_saver() ── bật saver nếu pin thấp
```

---

## 6. Re-export phẳng & helper module

`lib.rs` re-export phẳng toàn bộ API tiện dụng ở gốc (`babydra_core::*`) để crate UI import gọn. Ngoài ra có:

```rust
pub mod helper {
    pub use crate::services::notification::service as notification;
    pub use crate::services::system::backlight;
    // ... clean, network, storage, volume, wifi, window
}
```

Island dùng `babydra_core::helper::notification::*` (qua `babydra-island::widgets`) để giữ tương thích.

> [!NOTE]
> API reference đầy đủ: [apis/core](../apis/core.md). Cấu trúc thư mục: [structure](../structure/index.md).
