# Luồng hoạt động — `babydra-settings`

**Phạm vi:** Luồng CLI args (background tasks) và luồng GUI (sidebar + 13 section).
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Hai luồng hoạt động](#1-hai-luồng-hoạt-động)
- [2. Luồng CLI args](#2-luồng-cli-args)
- [3. Luồng GUI](#3-luồng-gui)
- [4. Layout & sidebar](#4-layout--sidebar)
- [5. State dùng chung](#5-state-dùng-chung)

---

## 1. Hai luồng hoạt động

| Luồng | Khi nào | Hành vi |
| :--- | :--- | :--- |
| **CLI** | Có arg nhận biết (`--*`) | Thực hiện tác vụ nền rồi thoát, không mở UI |
| **GUI** | Không có arg / arg lạ | Mở cửa sổ settings |

```text
main()
  ├─ handle_cli_args() == true → return (đã xử lý)
  └─ GUI: gtk4::Application + build_main_window
```

---

## 2. Luồng CLI args

`handle_cli_args()` — mỗi arg một tác vụ nền:

| Arg | Tác vụ |
| :--- | :--- |
| `--apply-battery-saver` | Nếu `power.auto_saver_enabled`: đổi profile về Normal + save config + notification |
| `--check-battery-saver` | Đọc pin → `check_and_apply_auto_battery_saver` (bật saver nếu dưới ngưỡng) |
| `--set-power-profile <key>` | `set_performance_profile` + save config + notification (normal/balanced/performance) |
| `--apply-all-settings` | `apply_all_saved_settings()` — CPU, displays, wallpaper, battery saver |
| `--sync-greeter-wallpaper` | `apply_saved_greeter_wallpaper()` — sync nền greeter ra path hệ thống |
| `--run-background-update` | Đọc password từ stdin → `run_background_update_loop` (pacman update) |
| `--help` / `-h` | In danh sách CLI options |

Luồng mẫu `--set-power-profile`:

```text
set_performance_profile(prof)
  → load config → power.profile = key → save
  → send_settings_notification (i18n)
  → thoát
```

---

## 3. Luồng GUI

```text
main (không có CLI arg)
  → gtk4::Application::new("com.babydra.settings", NON_UNIQUE)
  connect_activate:
     init_theme()
     layout::build_main_window(app)
```

---

## 4. Layout & sidebar

`layout/mod.rs` + `layout/sidebar.rs`:

```text
build_main_window(app)
  → cửa sổ chính + sidebar trái (danh sách section)
  → chọn section → render nội dung tương ứng
```

| Section | Module | Chức năng |
| :--- | :--- | :--- |
| appearance | `widgets/appearance/` | Theme, wallpaper, avatar |
| apps | `widgets/apps/` | Launch, update, uninstall |
| bluetooth | `widgets/bluetooth/` | Quản lý thiết bị Bluetooth |
| certificates | `widgets/certificates/` | CA certificates |
| displays | `widgets/displays/` | Màn hình (save/apply) |
| env | `widgets/env/` | Biến môi trường |
| hosts | `widgets/hosts/` | File /etc/hosts |
| keybinds | `widgets/keybinds/` | Phím tắt |
| power | `widgets/power/` | Power profile + battery card (saver tự động) |
| startup | `widgets/startup/` | Ứng dụng khởi động cùng hệ thống |
| system_info | `widgets/system_info/` | Thông tin hệ thống |
| system_update | `widgets/system_update/` | Pacman update + log realtime |
| vpn / wifi | `widgets/vpn/`, `widgets/wifi/` | Kết nối mạng |

---

## 5. State dùng chung

- `widgets/state.rs` — struct `*Widget` (GTK) của settings, tách khỏi core.
- `widgets/helpers.rs` — helper dùng chung giữa các section.
- Mỗi section tách `mod.rs` (logic) + `render.rs` (UI) + `handler.rs`/`handlers.rs` (sự kiện) theo quy ước chung (xem [structure](../structure/index.md) mục 3).

> [!NOTE]
> CLI thao tác qua `babydra_core` services (battery, power, wallpaper, updates) —
> chi tiết [flows/core.md](./core.md).
