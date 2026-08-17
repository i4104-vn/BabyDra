# Luồng hoạt động — `babydra-ui-kit`

**Phạm vi:** Cách `babydra-ui-kit` nạp giao diện (theme), cung cấp widget/icon/animation, và được các crate dùng chung.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Vai trò](#1-vai-trò)
- [2. Luồng nạp giao diện (`init_theme`)](#2-luồng-nạp-giao-diện-init_theme)
- [3. Luồng đổi dark/light runtime](#3-luồng-đổi-darklight-runtime)
- [4. Luồng widget builder & prelude](#4-luồng-widget-builder--prelude)
- [5. Luồng icon & animation](#5-luồng-icon--animation)
- [6. Luồng các crate gọi ui-kit](#6-luồng-các-crate-gọi-ui-kit)

---

## 1. Vai trò

`babydra-ui-kit` là tầng **UI infrastructure**: widget GTK dùng chung, CSS cấu trúc, theme coordinator, icon resolver, animation. Đây là crate UI **duy nhất** được phép import GTK ngoài các app (xem [architecture](../architecture/index.md) mục 2.4).

```text
Crate UI (panel, settings, explore, ...)
   │
   ├── gọi init_theme()           ── nạp CSS + theme package
   ├── gọi create_button/create_card/...  ── widget builders (qua prelude)
   ├── gọi get_icon(...)          ── icon resolver
   └── gọi slide_in/island_animate_*     ── animation helpers
```

---

## 2. Luồng nạp giao diện (`init_theme`)

Mọi app gọi `babydra_ui_kit::ui::theme::init_theme()` trong `activate()`. Luồng bên trong:

```text
init_theme()
  1. Đồng bộ GTK Settings với GSettings:
       color-scheme → set_gtk_application_prefer_dark_theme
       icon-theme   → set_gtk_icon_theme_name
       thêm search paths cho IconTheme (~/.local/share/icons, /usr/share/icons, /usr/share/pixmaps)

  2. Đăng ký CssProvider toàn cục (1 lần):
       style_context_add_provider_for_display(display, provider, PRIORITY_USER)
       gsettings.connect_changed("color-scheme")  ── cập nhật GTK dark preference
       gsettings.connect_changed("icon-theme")    ── cập nhật icon theme

  3. build_css():
       selection = load_babydra_config().theme.selection
       is_dark   = selection.dark.unwrap_or_else(is_dark_mode)   ← config ép mode hoặc theo GSettings
       (dark_css, light_css, extra_layer) = resolve_theme_layers()
       css = SHARED_CSS + color_layer(dark/light) + extra_layer

  4. provider.load_from_data(&css)
       + connect_gtk_application_prefer_dark_theme_notify → rebuild CSS + load lại
```

`resolve_theme_layers()` gọi qua `babydra-theme`:

```text
resolve_theme_layers()
  → id = theme.selection.id (rỗng → "babydra-default")
  → babydra_theme::resolve_theme(&id)
       lỗi → fallback "babydra-default"
       vẫn lỗi → load CSS cấu trúc thuần (không màu)
  → (theme.dark_css, theme.light_css, theme.css_layer)
```

> [!IMPORTANT]
> `SHARED_CSS` là CSS **cấu trúc** (layout, kích thước) nhúng sẵn trong crate
> (`include_str!`). Màu sắc chỉ nằm trong theme packages — xem
> [flows/theme.md](./theme.md).

---

## 3. Luồng đổi dark/light runtime

Khi người dùng đổi mode giữa chừng:

```text
GSettings "color-scheme" thay đổi
  → init_theme() được gọi lại (một số app) HOẶC
  → GTK Settings notify → provider load lại build_css() tự động
  → toàn bộ widget đổi màu ngay (không restart)
```

Đổi chủ động:

```text
set_dark_mode(dark)
  → thread nền: set_gsettings_color_scheme(dark)
  → idle main thread: set_gtk_application_prefer_dark_theme + init_theme()
```

---

## 4. Luồng widget builder & prelude

Các widget dùng chung được dựng bằng hàm builder thuần — không dùng `#[derive]` widget class:

```text
use babydra_ui_kit::prelude::*;

let btn   = create_button("OK");          ── class baby-button
let card  = create_switch_card(...);      ── card + switch
let slider = CustomSlider::new(50, cb);   ── vẽ Cairo, set_range(0,100,5)
```

Luồng một dialog modal:

```text
PasswordDialog::new(title, subtitle)   ── overlay box ẩn sẵn
  → show_for(prompt, sub)              ── hiện + focus entry
  → connect_submit(|pwd| ...)          ── user submit → callback (None nếu rỗng)
  → hide()                             ── ẩn lại
```

> [!NOTE]
> API reference: [apis/ui-kit](../apis/ui-kit.md). Danh sách prelude: mục 2 của tài liệu đó.

---

## 5. Luồng icon & animation

### 5.1. Icon

```text
get_icon("settings", 24)
  → tìm trong theme icon hiện tại (GTK IconTheme) → dark/light tự chọn
  → trả gtk4::Image

get_icon_colored(name, size, "#hex")   ── icon tint theo màu (Cairo)
get_system_or_file_icon(path, fallback) ── icon theo file/type
```

### 5.2. Animation

```text
slide_in(&widget, SlideDirection::Up, 200, None)   ── widget trượt vào
genie_out(&widget, 200, None)                       ── hiệu ứng genie đóng cửa sổ
island_animate_size(&widget, cur_w, tw, cur_h, th, ms, on_complete)
island_zoom_in/out(&widget, w, h, ms)               ── dùng bởi island transitions
```

Island dùng animation này trong `animate_expand`/`animate_collapse` — chi tiết: [flows/island.md](./island.md).

---

## 6. Luồng các crate gọi ui-kit

| Crate | Gọi gì | Khi nào |
| :--- | :--- | :--- |
| panel | `init_theme`, `apply_theme_class`, `get_icon`, `create_*`, animation | `activate()` + dựng widget |
| settings | `init_theme`, `create_switch_card`, `CustomSlider`, dialog... | `activate()` + từng section |
| explore | `init_theme`, `components::explore::*` (context menu, dialogs, drag&drop) | `activate()` + runtime |
| island | `init_theme` (không — do panel), animation island_*, `get_icon` | transitions, art, notification icon |
| screenshot/lock/preview/greeter/launcher | `init_theme`, widget builders | `activate()` |
