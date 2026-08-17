# 08 — API Reference

**Phạm vi:** bản đồ API của `babydra-core`, `babydra-ui-kit`, `babydra-explore`.
**Phiên bản:** 2.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## 1. Bản đồ API

| Thư viện | Vai trò | Xem thêm |
| :--- | :--- | :--- |
| `babydra-core` | Service, config, i18n, models (thuần logic, không GTK) | Mục 2 |
| `babydra-ui-kit` | Widget, theme init, icon, animation | Mục 3 |
| `babydra-explore` | File explorer components (trong ui-kit) | Mục 4 |
| `babydra-island` | Engine Dynamic Island | [07-dynamic-island.md](./07-dynamic-island.md) |
| `babydra-theme` | Resolve theme packages | [05-themes-variants.md](./05-themes-variants.md) |

---

## 2. babydra-core

### 2.1. Services — truy cập phần cứng & hệ thống

```rust
use babydra_core::services::{system, wallpaper, updates, notification};
```

| Service | API chính |
| :--- | :--- |
| `system::wifi` | quét mạng, connect, lưu/forget network, cường độ tín hiệu |
| `system::vpn` | danh sách VPN, connect/disconnect, đọc log |
| `system::volume` | get/set volume, mute |
| `system::brightness` | get/set backlight (DDC/CI qua `ddcutil`) |
| `system::battery` | phần trăm pin, trạng thái sạc |
| `system::cpu` | tải CPU, nhiệt độ, governor |
| `wallpaper` | đổi ảnh nền |
| `updates` | kiểm tra bản cập nhật (pacman) |
| `notification` | gửi thông báo hệ thống |

### 2.2. Config

```rust
use babydra_core::config;
let cfg = config::load_babydra_config();        // đọc ~/.babydra/babydra.conf (cache)
config::apply_all_saved_settings();              // áp dụng các lựa chọn đã lưu
```

### 2.3. i18n

```rust
use babydra_core::i18n::t;
label.set_text(&t("settings.wifi"));             // tra locales/*/{en,vi}.json
```

### 2.4. Models

| Model | Dùng cho |
| :--- | :--- |
| `ThemeConfig` / `ThemeSelection` | `[theme] selection = { id, dark }` trong config |
| `ExploreGrouping` | Nhóm file trong explore |
| `Storage`, `Vpn`, `WifiNetwork`… | Dữ liệu service |

---

## 3. babydra-ui-kit

### 3.1. Theme

```rust
use babydra_ui_kit::ui::theme::init_theme;
init_theme();          // bắt buộc gọi khi app khởi động — nạp CSS toàn cục
```

### 3.2. Components — bản đồ

| Component | Entry point | Chi tiết |
| :--- | :--- | :--- |
| Button | `create_icon_button(icon, size, classes, tooltip, cb)` | [10-components.md](./10-components.md) |
| Badge | `create_status_badge(text, is_success)` · `create_icon_badge(icon, size, small)` | 〃 |
| Card | `create_card(orient, spacing)` · `create_title` · `create_item_row` · `create_switch_card` | 〃 |
| Switch | `create_switch(initial, cb)` · `ToggleRow::new(initial)` | 〃 |
| Slider | `CustomSlider::new(value, cb)` · `new_range(min, max, step, …)` | 〃 |
| Modal | `PasswordDialog::new(title, sub)` · `WifiPasswordDialog` · `VpnConfigDialog`… | 〃 |
| Popover | `create_popover(parent, pos, class)` · `attach_hover_popover` | 〃 |
| Navbar | `create_sidebar_row(label, icon)` | 〃 |
| List | `create_list_row(icon, title, sub, right)` · `clear_list_box` | 〃 |
| Placeholder | `create_placeholder_row(PlaceholderState)` | 〃 |
| Progress | `create_progress_bar(fraction, class)` · `create_disk_progress` | 〃 |
| Spinner | `create_spinner(size)` · `create_loading_box(text)` | 〃 |
| Tooltip | `set_tooltip(widget, text)` | 〃 |
| Close button | `create_close_button(class)` | 〃 |
| Wi-Fi icon | `create_system_wifi_signal_icon(size, color)` · `create_wifi_signal_icon_for_network(…)` | 〃 |

### 3.3. Icon & Animation

```rust
use babydra_ui_kit::ui::icon::{get_icon, get_system_or_file_icon};
use babydra_ui_kit::ui::animation;   // helper animate (ease, duration)
```

Quy tắc: dùng component có sẵn (`create_*`) — không tự dựng widget tay. Chi tiết từng component: [10-components.md](./10-components.md).

---

## 4. babydra-explore (file explorer)

### 4.1. Widgets chính

| Widget | Vai trò |
| :--- | :--- |
| `window` | Cửa sổ explore + layout tổng |
| `content_view` | Vùng nội dung — render theo `SessionState`, gestures (clipboard, background, context menu) |
| `sidebar` | Điều hướng thư mục |
| `status_bar` | Thanh trạng thái (số file, dung lượng) |
| `info_panel` | Chi tiết file đang chọn |

### 4.2. Items & menu

| Thành phần | Mô tả |
| :--- | :--- |
| `items::list_row` | Dòng file dạng danh sách |
| `items::grid_card` | Card file dạng grid |
| `context_menu::file_actions` | Hành động file (mở, đổi tên, xóa, properties…) |

### 4.3. Mẫu nhanh

```rust
// explore dùng tokio + SessionState; UI render thuần từ state
// (chi tiết luồng: 06-system-flows.md mục 4 — babydra-explore)
```

---

## 5. Quy tắc dùng API

| DO | DO NOT |
| :--- | :--- |
| Gọi `init_theme()` đầu mỗi app | Tự nạp CSS / đặt màu cứng |
| Dùng `create_*` từ ui-kit | Tự dựng GTK widget tay không chuẩn |
| Đi qua `i18n::t` cho chuỗi UI | Hardcode chuỗi tiếng Anh trong widget |
| Dùng `babydra_core` cho logic hệ thống | Nhân bản service logic trong app |
