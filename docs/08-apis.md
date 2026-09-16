# 08 — API dùng chung

## Phạm vi

Trang này ghi lại public API thường dùng của `babydra-core`, `babydra-ui-kit`, `babydra-theme` và `babydra-island`. Khi API trong Rust thay đổi, cập nhật ví dụ và bảng bên dưới trong cùng thay đổi.

## `babydra-core`

`babydra-core` không phụ thuộc GTK. Application dùng thư viện này cho system service, config, notification, wallpaper, update và i18n.

### Service

| Nhóm | Module | Phạm vi |
| :--- | :--- | :--- |
| Network | `services::system::wifi` | Scan, connect, disconnect, forget và trạng thái tín hiệu. |
| VPN | `services::system::vpn` | Danh sách, connect/disconnect và log VPN. |
| Audio | `services::system::volume` | Volume, mute và player audio. |
| Display | `services::system::brightness` | Backlight hoặc DDC/CI tùy backend. |
| Power | `services::system::battery` | Phần trăm pin, charging và nguồn. |
| CPU | `services::system::cpu` | Load, temperature và governor. |
| Desktop | `services::wallpaper` | Wallpaper và trạng thái wallpaper. |
| Updates | `services::updates` | Kiểm tra cập nhật package. |
| Notification | `services::notification` | D-Bus notification và active notification model. |

Application không gọi `nmcli`, `wpctl` hoặc command hệ thống tương tự nếu core đã có service tương ứng. Điều này giữ behavior và error handling nhất quán giữa các app.

### Config

```rust
use babydra_core::config;

let config = config::load_babydra_config();
config::apply_all_saved_settings();
```

Config chính nằm ở `~/.babydra/babydra.conf`. Theme selection, app setting và giá trị đã lưu phải đi qua module config thay vì mỗi crate tự parse TOML.

### i18n

```rust
use babydra_core::i18n::trans;

let title = trans("settings.wifi");
```

Dùng translation key cho text hiển thị. Key mới phải được thêm vào locale data tương ứng và không được đặt string tiếng Anh trực tiếp trong widget nếu text cần dịch.

## `babydra-theme`

### Hàm chính

| Hàm/type | Mục đích |
| :--- | :--- |
| `themes_root()` | Chọn theme root theo env, user directory, system directory và workspace. |
| `load_package(id)` | Đọc một theme package thành `ThemePackage`. |
| `resolve_theme(id)` | Resolve base theme và trả `ThemeValue` hoàn chỉnh. |
| `ThemePackage` | Dữ liệu package trước khi merge kế thừa. |
| `ThemeValue` | Tokens, CSS dark/light, extra CSS, font map và package path sau khi resolve. |
| `ThemeError` | Lỗi package không tồn tại, JSON sai hoặc cycle kế thừa. |

Resolution order của `themes_root()`:

1. `BABYDRA_THEMES_DIR` nếu được đặt.
2. `~/.babydra/themes` nếu tồn tại.
3. `/usr/share/babydra/themes` nếu tồn tại.
4. `themes/` tương đối với workspace.

`load_package` đọc layout mới `css/dark.css`, `css/light.css`, `css/theme.css` và vẫn hỗ trợ layout CSS phẳng cũ như fallback. `resolve_theme` merge token base trước, child sau; CSS cũng nối base trước, child sau để rule child thắng.

## `babydra-ui-kit`

### Khởi tạo theme

```rust
use babydra_ui_kit::prelude::init_theme;

init_theme();
```

`init_theme()` đăng ký `GtkCssProvider` cho display, nạp CSS cấu trúc được nhúng trong crate, resolve color layer từ `babydra-theme`, đồng bộ GTK icon theme và theo dõi thay đổi color scheme/icon theme. Gọi trước khi dựng widget.

### Prelude

`babydra_ui_kit::prelude` re-export các builder thường dùng:

| Nhóm | API |
| :--- | :--- |
| Button | `create_button`, `create_accent_button`, `create_fab`, `create_icon_button`, `create_icon_btn` |
| Card | `create_card`, `create_css_card`, `create_collapsible_card`, `create_switch_card`, `create_title`, `create_subtitle` |
| List | `create_list_row`, `clear_box`, `clear_list_box`, `create_scroll_list` |
| Switch | `create_switch`, `CustomSwitch`, `ToggleRow` |
| Slider | `CustomSlider`, `PillSlider`, `bind_debounced_slider` |
| Modal | `PasswordDialog`, `WifiPasswordDialog`, `WifiInfoDialog`, `WifiConfigDialog`, `VpnConfigDialog`, `VpnLogDialog` |
| Popover | `create_popover`, `TooltipPopover`, `TooltipRow` |
| Placeholder | `create_placeholder`, `PlaceholderState` |
| Icon | `get_icon`, `get_fallback_icon`, `get_resolved_icon`, `set_image_from_icon` |
| Image/window | `create_rounded_picture`, `init_layer_window`, `setup_click_outside_dismiss` |

### Animation và window helper

Animation được chia theo mục đích trong `ui::animation`:

- easing: `linear`, `ease_in_cubic`, `ease_out_cubic`, `ease_in_out_cubic`;
- slide: `slide_in`, `slide_out`, `slide_out_cb`;
- island: `island_animate_size`, `island_animate_width`, `island_zoom_in`, `island_zoom_out`;
- genie: `genie_in`, `genie_out`;
- panel startup: `topbar_startup_cascade`.

Dùng helper hiện có để giữ duration và cancellation behavior thống nhất.

## `babydra-island`

Public export chính:

```rust
use babydra_island::{
    build_default_island, create_system_island, Island, IslandBuilder,
    IslandConfig, IslandFeature, IslandView, IslandViewHandle,
};
```

`IslandView` phù hợp với view điều khiển bằng handle; `IslandFeature` phù hợp với stateful feature. Chi tiết arbitration và lifecycle nằm trong [07 — Dynamic Island](07-dynamic-island.md).

## Quy tắc dùng API

- Dùng core cho system operation.
- Dùng ui-kit cho widget, icon, animation và theme initialization.
- Dùng theme engine thay vì tự tìm file theme.
- Không thao tác GTK từ thread nền.
- Bổ sung test cho parser, state machine và helper thuần.
- Khi API chung thiếu khả năng, sửa abstraction chung thay vì tạo implementation riêng trong từng app.
