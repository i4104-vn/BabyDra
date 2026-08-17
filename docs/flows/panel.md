# Luồng hoạt động — `babydra-panel`

**Phạm vi:** Luồng khởi động, dựng giao diện, rebuild, và vòng đời của thanh taskbar chính.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Luồng khởi động](#1-luồng-khởi-động)
- [2. Dựng giao diện (`build_panel_ui`)](#2-dựng-giao-diện-build_panel_ui)
- [3. Rebuild panel](#3-rebuild-panel)
- [4. Input region (click-through)](#4-input-region-click-through)
- [5. Các widget chính](#5-các-widget-chính)
- [6. Island & launcher từ panel](#6-island--launcher-từ-panel)

---

## 1. Luồng khởi động

`crates/babydra-panel/src/main.rs`:

```text
main()
  1. tray::spawn_watcher_service()        ── D-Bus StatusNotifierWatcher (tray)
  2. widgets::panel::detect_ddc_bus()     ── DDC/CI cho màn hình ngoài
  3. thread: refresh_desktop_apps_cache() ── cache apps bất đồng bộ
  4. spawn_switcher_tracker()             ── theo dõi window focus
  5. gtk4::Application::new("org.babydra.panel")

  connect_activate:
    a. init_theme()                       ── nạp CSS + theme package
    b. GSettings color-scheme → init_theme() khi đổi (real-time)
    c. Tạo 3 Rc<RefCell<Option<ApplicationWindow>>>:
         control_center_window / calendar_window / launcher_window
         ── mutual exclusivity: chỉ 1 cửa sổ nổi mở tại một thời điểm
    d. build_panel_ui(app, cc, cal, lw)
    e. window.present()

  application.run()
```

---

## 2. Dựng giao diện (`build_panel_ui`)

`crates/babydra-panel/src/render.rs`:

```text
build_panel_ui(app, ...)
  1. ApplicationWindow + apply_theme_class
  2. init_layer_shell()
     layer = Top, exclusive_zone = 38, anchor Top+Left+Right
     margin top 8, default size (0, 36)
  3. rebuild_panel_window(window, ...)
  4. Đăng ký rebuild triggers:
       settings.connect_gtk_application_prefer_dark_theme_notify → rebuild
       babydra_core::i18n::watch_locale_change → rebuild
```

### Layout 3 vùng (CenterBox)

```text
┌──────────────────────────────────────────────────────────────┐
│ left_wrapper        │  center_box        │  right_wrapper    │
│  workspace_box      │    notch_capsule   │   tray_widget     │
│  (logo_btn +        │    (create_system_ │   sys_monitor     │
│   separator +       │     island)        │   status_indicators│
│   workspace switcher)│                    │                   │
└──────────────────────────────────────────────────────────────┘
```

- **Left:** `logo_btn` (mở launcher) + separator + `create_workspace_switcher()`.
- **Center:** `create_system_island()` → notch capsule (Dynamic Island).
- **Right:** `create_tray_widget` (StatusNotifier) + `create_sys_monitor_widget` (CPU/RAM) + `create_status_indicators` (clock, wifi, volume, battery...).

---

## 3. Rebuild panel

Panel **rebuild toàn bộ cây widget** khi đổi theme hoặc locale (không restart process):

```text
rebuild_panel_window(window, app, cc, cal, lw)
  1. window.set_child(None)              ── bỏ toàn bộ widget cũ
  2. Dựng lại CenterBox + logo + workspace + status + tray + sys_monitor
  3. center_box.append(create_system_island())   ── island MỚI
  4. window.set_child(box_layout)
  5. Gắn tick callback input region
```

> [!IMPORTANT]
> `create_system_island()` mỗi lần rebuild tạo island mới — island cũ bị
> `dispose()` tự động qua `set_default_island` (xem [flows/island.md](./island.md) mục dispose).
> Mọi feature đăng ký thêm vào `default_island()` phải **re-register** sau rebuild.

---

## 4. Input region (click-through)

Panel là cửa sổ layer-shell 36px nhưng island có thể mở rộng xuống dưới. Để vùng ngoài panel/island không chặn chuột, mỗi frame (`add_tick_callback`):

```text
tick:
  region = vùng trống
  union: toàn bộ top bar rect (0, 0, win_w, 36)
  nếu notch capsule đang hiển thị và nh > 36:
      union: notch rect (từ translate_coordinates(win, 0, 0))
  surface.set_input_region(&region)
```

---

## 5. Các widget chính

| Widget | File | Chức năng |
| :--- | :--- | :--- |
| `create_status_indicators` | `widgets/panel/mod.rs` | Gom các item trạng thái: clock, wifi, volume, vpn, backlight, storage, clean |
| `create_workspace_switcher` | `widgets/workspace/` | Workspace + preview |
| `create_sys_monitor_widget` | `widgets/sys_monitor/` | CPU/RAM |
| `create_tray_widget` | `widgets/tray/` | Khay hệ thống StatusNotifier |
| clock | `widgets/clock/` | Đồng hồ + calendar_window + notifications (`notification_group`) |
| control center | `widgets/panel/popover/` | network, volume, vpn, battery popovers |

---

## 6. Island & launcher từ panel

- **Island:** panel là nơi duy nhất gọi `create_system_island()` — qua đó `build_default_island()` chạy và đăng ký `default_island()` toàn process. Các feature khác (volume overlay...) lấy manager qua `babydra_island::default_island()`.
- **Launcher:** `logo_btn` click → nếu `launcher_window` chưa có → `babydra_launcher::build_launcher_ui(&app, lw)` + present; ngược lại close (toggle). Cùng lúc đóng control center + calendar (mutual exclusivity).

Chi tiết island: [flows/island.md](./island.md).
