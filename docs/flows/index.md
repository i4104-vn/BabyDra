# Luồng hoạt động — Tổng quan toàn hệ thống

**Phạm vi:** Luồng hoạt động hiện tại của toàn bộ hệ thống BabyDra: khởi động DE, mô hình daemon-client, và mục lục luồng chi tiết theo từng crate/lib.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

> [!NOTE]
> Khác với [architecture](../architecture/index.md) (4 pattern thiết kế), tài liệu này mô tả
> **luồng chạy thực tế** của từng thành phần trong code — ai gọi ai, khi nào, theo thứ tự nào.

---

## Mục lục

- [1. Luồng khởi động toàn hệ thống](#1-luồng-khởi-động-toàn-hệ-thống)
- [2. Mô hình daemon-client trong thực tế](#2-mô-hình-daemon-client-trong-thực-tế)
- [3. Bản đồ luồng theo từng crate/lib](#3-bản-đồ-luồng-theo-từng-cratelib)
- [4. Thứ tự đề xuất khi đọc](#4-thứ-tự-đề-xuất-khi-đọc)

---

## 1. Luồng khởi động toàn hệ thống

Khi người dùng đăng nhập (qua greeter → greetd → cage), toàn hệ thống khởi động theo luồng:

```text
labwc --startup (qua start.sh)
   │
   ├── autostart (configs/labwc/autostart)
   │    ├── babydra-panel            ── DE chính: dock, tray, island, clock, control center
   │    ├── babydra-switcher --daemon ── Alt-Tab overlay giữ sẵn trong bộ nhớ
   │    ├── fcitx5 / other daemons
   │    └── scripts/bat_saver.sh     ── battery saver tự động
   │
   └── D-Bus session bus
        └── org.freedesktop.Notifications  ── do notification feature của island host
```

Khởi động từng ứng dụng đều theo mẫu chung (xem [architecture](../architecture/index.md) mục 6):

```text
main()
  → init các service nền (tray watcher, DDC detection, apps cache...)
  → gtk4::Application::new("org.babydra.*")
  → connect_activate:
        babydra_ui_kit::ui::theme::init_theme()   ← nạp CSS + theme package
        build_*_ui(app, ...)                       ← dựng cây widget + layer shell
        window.present()
  → application.run()                              ← main loop
```

| Crate | Luồng chi tiết |
| :--- | :--- |
| panel | [flows/panel.md](./panel.md) |
| switcher | [flows/switcher.md](./switcher.md) |
| screenshot | [flows/screenshot.md](./screenshot.md) |
| lock | [flows/lock.md](./lock.md) |
| greeter | [flows/greeter.md](./greeter.md) |
| settings | [flows/settings.md](./settings.md) |
| preview | [flows/preview.md](./preview.md) |
| explore | [flows/explore.md](./explore.md) |
| launcher | [flows/launcher.md](./launcher.md) |
| installer | [flows/installer.md](./installer.md) |

---

## 2. Mô hình daemon-client trong thực tế

Hai ứng dụng triển khai mô hình daemon-client đầy đủ — daemon giữ cửa sổ trong bộ nhớ, client chỉ gửi tín hiệu rồi thoát:

| Crate | Socket | Client gửi | Daemon nhận |
| :--- | :--- | :--- | :--- |
| `babydra-switcher` | `/tmp/babydra-switcher.socket` | `show` / `next` / `hide` | Hiện/cycle/ẩn overlay |
| `babydra-settings` | (CLI args, không socket) | `--apply-battery-saver`... | Thực hiện rồi thoát |

Chi tiết luồng socket của switcher: [flows/switcher.md](./switcher.md).

---

## 3. Bản đồ luồng theo từng crate/lib

### 3.1. Libs (thư viện dùng chung)

| Lib | Luồng chính | Tài liệu |
| :--- | :--- | :--- |
| `babydra-core` | Services hệ thống (battery, wifi, vpn, volume, backlight, power, storage...), i18n, config, notification daemon, tray watcher, window tracker, screenshot helpers | [flows/core.md](./core.md) |
| `babydra-ui-kit` | Nạp theme (`init_theme`), widget builders, icon resolver, animation, battery/window helpers | [flows/ui-kit.md](./ui-kit.md) |
| `babydra-theme` | Đọc theme package `themes/<id>/`, resolve CSS dark/light + tokens + fonts, kế thừa `base` | [flows/theme.md](./theme.md) |
| `babydra-island` | Controller loop 150ms: timer → feature ticks → arbitration → transition; media player + notification | [flows/island.md](./island.md) (chi tiết: [guides/island-internals](../guides/island-internals.md)) |

### 3.2. Crates (ứng dụng)

| Crate | Luồng chính | Tài liệu |
| :--- | :--- | :--- |
| `babydra-panel` | Tray watcher → DDC detect → apps cache → GTK app → build_panel_ui → island + status indicators | [flows/panel.md](./panel.md) |
| `babydra-switcher` | Daemon: socket listener → message pump (8ms) → show/cycle/hide | [flows/switcher.md](./switcher.md) |
| `babydra-screenshot` | `--full` → chụp ngay; ngược lại → regional capture → editor | [flows/screenshot.md](./screenshot.md) |
| `babydra-lock` | Parse `--image` → build_lock_ui → map toàn màn hình | [flows/lock.md](./lock.md) |
| `babydra-greeter` | init_logger → GTK app → build_greeter_ui → setup_handlers → PAM auth | [flows/greeter.md](./greeter.md) |
| `babydra-settings` | CLI args → apply/set; ngược lại → GTK app → layout sidebar + 13 section | [flows/settings.md](./settings.md) |
| `babydra-preview` | argv path → viewer; fallback FileDialog → viewer | [flows/preview.md](./preview.md) |
| `babydra-explore` | tokio runtime → SessionState → create_explore_window → content/grid/list + gestures | [flows/explore.md](./explore.md) |
| `babydra-launcher` | GTK app → build_launcher_ui → fuzzy search apps/file | [flows/launcher.md](./launcher.md) |
| `babydra-installer` | TUI 8 bước: raw mode → event loop 50ms → worker installation | [flows/installer.md](./installer.md) |

---

## 4. Thứ tự đề xuất khi đọc

1. [core.md](./core.md) — mọi crate gọi API của core.
2. [ui-kit.md](./ui-kit.md) + [theme.md](./theme.md) — nạp giao diện chung.
3. [panel.md](./panel.md) — DE chính, nơi mọi thứ gắn lại.
4. [island.md](./island.md) — widget phức tạp nhất trong panel.
5. Các crate còn lại theo nhu cầu.
