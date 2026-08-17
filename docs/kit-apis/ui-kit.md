# API Reference — `babydra-ui-kit`

**Crate:** `libs/babydra-ui-kit/`
**Phạm vi:** Widget GTK4 dùng chung + UI helpers (theme, icon, animation, battery, window) + feature components `components/explore/`.
**Dependency:** `babydra-core`, `babydra-theme`, `gtk4`, `trash`.

---

## 1. Cách dùng nhanh

Thêm vào `Cargo.toml`:

```toml
babydra-ui-kit = { workspace = true }
```

Import toàn bộ API thông dụng qua `prelude`:

```rust
use babydra_ui_kit::prelude::*;

fn build_my_panel() {
    init_theme(); // nạp CSS shared + theme package (dark/light/override)

    let fab = create_fab("plus");
    let card = create_switch_card("Dark mode", "Bật/tắt giao diện tối");
    let icon = get_icon("settings", 24);
}
```

Cần truy cập sâu hơn thì dùng module trực tiếp:

```rust
use babydra_ui_kit::components::modal::WifiPasswordDialog;
use babydra_ui_kit::ui::animation::slide::SlideDirection;
```

---

## 2. `prelude` — re-export toàn bộ API thông dụng

`babydra_ui_kit::prelude::*` gộp mọi builder + helper hay dùng nhất về một nơi.
Các mục chính:

| Nhóm | Mục |
| :--- | :--- |
| **Buttons** | `create_button`, `create_accent_button`, `create_fab`, `create_icon_button`, `create_colored_icon_button`, `create_icon_label_button`, `create_sidebar_item_button(_with_widget)`, `create_toggle_tile`, `create_square_toggle_tile`, `update_toggle_tile_state`, `create_wifi_signal_icon`, `create_battery_percentage_icon`, `create_vpn_shield_icon`, `create_wallpaper_thumbnail_icon`, `create_colored_icon_widget` |
| **Cards** | `create_card`, `create_card_with_class`, `create_scrollable_list`, `create_switch_card`, `create_title`, `create_subtitle` |
| **List** | `create_list_row`, `clear_list_box`, `clear_box` |
| **Placeholder** | `PlaceholderState`, `create_placeholder_row` |
| **Popovers** | `create_popover`, `create_popover_with_content`, `attach_hover_popover`, `build_hover_popover_card`, `HoverPopoverRow` |
| **Switch / Slider** | `CustomSwitch`, `create_switch`, `ToggleRow`, `CustomSlider` |
| **Modals** | `PasswordDialog`, `WifiConfigDialog`, `WifiInfoDialog`, `WifiPasswordDialog`, `VpnConfigDialog`, `VpnLogDialog` |
| **Badge** | `create_icon_badge` |
| **Tooltips** | `set_tooltip` |
| **Wi-Fi icons** | `render_wifi_signal_svg`, `create_wifi_signal_icon_from_strength`, `create_wifi_signal_icon_for_network`, `create_system_wifi_signal_icon` |
| **Theme** | `init_theme`, `apply_theme_class`, `is_dark_mode`, `set_dark_mode` |
| **Icons** | `get_icon`, `get_icon_colored`, `get_icon_from_svg`, `get_logo_png`, `get_resolved_icon_path`, `set_image_from_icon`, `get_system_or_file_icon`, `set_system_or_file_icon` |
| **Animation** | `ease_*`, `linear`, `genie_in/out`, `island_zoom_in/out`, `island_animate_width/size`, `slide_in/out`, `slide_out_cb`, `SlideDirection` |
| **Battery** | `get_battery_color_hex`, `get_battery_color_rgb`, `draw_cairo_battery`, `create_battery_drawing_area` |
| **Window** | `init_layer_window`, `setup_click_outside_dismiss` |

> [!NOTE]
> `prelude` giữ **module gốc vẫn truy cập được** — chỉ gộp mặt phẳng các tên hay dùng,
> không thay thế cấu trúc module (`components`, `ui`).

---

## 3. Components (thành phần giao diện)

### 3.1. Buttons

| Hàm | Kiểu trả về | Mô tả |
| :--- | :--- | :--- |
| `create_button(label)` | `gtk4::Button` | Nút chuẩn (class `baby-button`) |
| `create_accent_button(label)` | `gtk4::Button` | Nút primary accent (`suggested-action`) |
| `create_fab(icon_name)` | `gtk4::Button` | Floating Action Button tròn |
| `create_icon_button(icon, size, classes, tooltip, on_click)` | `gtk4::Button` | Nút icon generic, nhận callback click |
| `create_toggle_tile(icon, title, active)` | `gtk4::Button` | Tile bật/tắt (control center) |
| `create_square_toggle_tile(...)` | `gtk4::Button` | Tile vuông |
| `update_toggle_tile_state(btn, active, icon)` | `()` | Cập nhật trạng thái tile |

### 3.2. Cards & Switch Card

```rust
let card = create_card(gtk4::Orientation::Vertical, 12);
let scrollable = create_scrollable_list("glass-panel"); // (ScrolledWindow, ListBox)
let (box_, sw) = create_switch_card("Title", "Subtitle");
```

### 3.3. Switch / Slider (vẽ Cairo)

```rust
let sw = CustomSwitch::new(true);
sw.connect_state_set(|active| { println!("{active}"); });

let slider = CustomSlider::new(50, |v| println!("{v}"));
slider.set_range(0, 100, 5); // range + step
let value = slider.value();
```

### 3.4. Modal Dialogs

| Struct | Dùng cho | API chính |
| :--- | :--- | :--- |
| `PasswordDialog` | Nhập mật khẩu | `new`, `show_for`, `connect_submit`, `hide` |
| `WifiPasswordDialog` | Mật khẩu Wi-Fi | `new`, `show_for(ssid, security)`, `set_error`, `connect_submit` |
| `WifiConfigDialog` | Cấu hình mạng tĩnh | `new`, `show_for(ssid, cfg)`, `connect_save` |
| `WifiInfoDialog` | Thông tin mạng | `new`, `show_for(net, config)`, `connect_configure`, `connect_forget` |
| `VpnConfigDialog` | Thêm/sửa VPN | `new`, `show_for_new/edit`, `apply_config_file`, `connect_save/delete` |
| `VpnLogDialog` | Log VPN | `new`, `show_for_vpn(name)` |

### 3.5. Placeholder

```rust
let row = create_placeholder_row(PlaceholderState::Loading);
let row = create_placeholder_row(PlaceholderState::Empty {
    title_key: "explore.empty", desc_key: None, icon_name: "folder",
});
```

### 3.6. Wi-Fi signal icons

```rust
let icon = create_wifi_signal_icon_from_strength(3);          // 0–4 vạch
let icon = create_wifi_signal_icon_for_network(&network);      // từ WifiNetwork
let svg  = render_wifi_signal_svg(strength, size, color);      // SVG thô
```

---

## 4. UI helpers

### 4.1. Theme

| Hàm | Mô tả |
| :--- | :--- |
| `init_theme()` | Gọi 1 lần khi app khởi động — nạp CSS shared + theme package đang chọn (`~/.babydra/babydra.conf` → `theme.selection`) |
| `set_dark_mode(bool)` | Đổi GSettings + reload CSS |
| `is_dark_mode() -> bool` | Trạng thái dark hiện tại |
| `apply_theme_class(window)` | Stub tương thích (theme áp dụng toàn cục) |

### 4.2. Icons

```rust
let img = get_icon("settings", 24);              // dark/light tự động theo mode
let img = get_icon_colored("bell", 20, "#3b82f6");
let img = get_icon_from_svg(svg_string, 32);
let img = get_logo_png(64);
let img = get_system_or_file_icon("/path/icon.png", "image-missing");
```

### 4.3. Animation

```rust
slide_in(&widget, SlideDirection::Up, 200, None);
genie_out(&widget, 200, None);
island_animate_width(&widget, 300, 200, None);
let t = ease_out_cubic(0.5);
```

### 4.4. Battery

```rust
let hex = get_battery_color_hex(80, true);   // "#2ec27e"...
let (r, g, b) = get_battery_color_rgb(30, false);
draw_cairo_battery(&ctx, width, height, percent, charging);
let area = create_battery_drawing_area(percent, charging);
```

### 4.5. Window

```rust
init_layer_window(&window, "top");  // cửa sổ layer-shell cho panel/overlay
setup_click_outside_dismiss(&window, &content);
```

---

## 5. Quy tắc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Import qua `prelude::*` cho API thông dụng; dùng module sâu khi cần cụ thể |
| DO | Gọi `init_theme()` đúng 1 lần lúc khởi động trước khi tạo widget |
| DO | Mọi chuỗi hiển thị phải qua `babydra_core::i18n::t()` — không hardcode tiếng Việt/Anh |
| DO NOT | Tạo `GtkCssProvider` riêng trong app — màu/CSS đi qua theme package + `init_theme()` |
| DO NOT | Viết CSS inline trong Rust |

Xem thêm: [tổng hợp API kits](../06-kits-api.md), [components design](../design/README.md), [theming](../design/theming.md),
[API `explore`](./explore-kit.md).
