# Themes & Variants — Hướng dẫn mở rộng

**Phạm vi:** Cách tạo theme package mới và variant mới — dành cho người dùng thứ 3, không cần sửa code core.
**Phiên bản:** 1.2.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Khái niệm](#1-khái-niệm)
- [2. Tạo theme mới](#2-tạo-theme-mới)
- [3. Tạo variant mới](#3-tạo-variant-mới)
- [4. Merge thứ tự](#4-merge-thứ-tự)
- [5. Quy tắc](#5-quy-tắc)

---

## 1. Khái niệm

| Khái niệm | Là gì | Nằm ở đâu |
| :--- | :--- | :--- |
| **Theme package** | Giao diện: tokens + lớp CSS màu + fonts | `themes/<theme-id>/` |
| **Variant** | Một phiên bản hoàn chỉnh: theme nào + app nào + keybind nào | `variants/<name>/` |

Nguyên tắc: **người tạo theme/variant không cần sửa 1 dòng code core** — mọi thứ đi qua file cấu hình.

---

## 2. Tạo theme mới

### 2.1. Tạo thư mục

Mọi theme package **bắt buộc** có chung cấu trúc — file JSON ở gốc, toàn bộ CSS trong `css/`:

```bash
mkdir -p themes/my-theme/css
```

```text
themes/my-theme/
├── tokens.json        (JSON) design tokens: surface, border, accent, font, radius
├── fonts.json         (JSON) font families + fallbacks
└── css/               (CSS) — tách riêng, KHÔNG nằm chung với file JSON
    ├── dark.css       lớp màu dark-mode
    ├── light.css      lớp màu light-mode
    └── theme.css      (tùy chọn) override nạp cuối
```

> [!IMPORTANT]
> Cấu trúc này là bắt buộc cho **mọi** theme package — xem `themes/babydra-default/` và `themes/babydra-blue/` làm mẫu.

### 2.2. `tokens.json`

```jsonc
{
  "name": "my-theme",
  "base": "babydra-default",        // kế thừa theme khác (tùy chọn)
  "dark": {
    "surface": "rgba(14, 14, 18, 0.96)",
    "border": "rgba(255, 255, 255, 0.14)",
    "accent": "#8b5cf6",            // đổi màu điểm nhấn
    "font": "Segoe UI Variable Static Text",
    "radius": { "pill": 9999, "lg": 20, "md": 16, "sm": 10 }
  },
  "light": { /* tương tự */ }
}
```

> [!TIP]
> Bỏ qua field nào thì field đó **kế thừa từ `base`**. `base = null` = độc lập.

### 2.3. Lớp màu: `css/dark.css` + `css/light.css`

Mỗi theme package sở hữu **lớp màu** riêng (nạp sau CSS cấu trúc `styles/shared/`):

```css
/* css/dark.css — màu dark-mode */
.panel { background: rgba(14, 14, 18, 0.96); }
```

```css
/* css/light.css — màu light-mode */
.panel { background: rgba(255, 255, 255, 0.98); }
```

> [!TIP]
> Cách nhanh nhất: copy toàn bộ `themes/babydra-default/`, sửa màu trong `css/dark.css`/`css/light.css` và `tokens.json`, đổi `name`. Không cần động tới code.

### 2.4. `css/theme.css` — lớp override (tùy chọn, nạp cuối)

```css
/* css/theme.css — nạp SAU dark/light, thích hợp để override accent */
button.baby-fab { background-color: #8b5cf6; }
```

Ví dụ thật: `themes/babydra-blue/css/theme.css` kế thừa `babydra-default` rồi override accent `#3b82f6` → `#38bdf8` mà không ship lại toàn bộ dark/light.

### 2.5. `fonts.json`

```json
{
  "My Font": ["Segoe UI", "sans-serif"]
}
```

### 2.6. Kiểm tra theme

```bash
cargo test -p babydra-theme        # engine hoạt động
```

### 2.7. Themes đi kèm (repo)

| Theme | Accent | Cách hoạt động |
| :--- | :--- | :--- |
| `babydra-default` | `#3b82f6` (xanh dương) | Base — sở hữu `css/dark.css` + `css/light.css` đầy đủ |
| `babydra-blue` | `#38bdf8` (xanh trời) | Kế thừa default, override qua `css/theme.css` |
| `babydra-purple` | `#8b5cf6` (tím) | Kế thừa default, override qua `css/theme.css` |
| `babydra-green` | `#10b981` (ngọc lục bảo) | Kế thừa default, override qua `css/theme.css` |
| `babydra-rose` | `#f43f5e` (hồng) | Kế thừa default, override qua `css/theme.css` |

> [!NOTE]
> Các theme màu dùng **1 lớp override chung** cho cả dark + light mode (`css/theme.css`). Nếu tạo theme màu, hãy kiểm tra cả 2 mode — màu text sáng có thể tương phản thấp trên nền light.

### 2.8. Áp dụng theme

1. Deploy theme vào `~/.babydra/themes/<id>/` (hoặc `/usr/share/babydra/themes/`).
2. Đổi config trong `~/.babydra/babydra.conf`:

```toml
[theme]
selection = { id = "my-theme", dark = false }   # dark = null → theo hệ thống
```

3. Khởi động lại app (panel, settings, explore...).

---

## 3. Tạo variant mới

### 3.1. Tạo thư mục

```bash
mkdir variants/my-name-variant
```

### 3.2. `variant.toml`

```toml
name = "my-name-variant"
theme = "my-theme"                  # ref tới themes/<id>/
apps = ["panel", "explore", "settings"]

[keybinds]
"A-Tab" = "babydra-switcher"
"W-q" = "babydra-launcher"

[config_overrides]
"labwc.rc.margin.gap" = 12
```

### 3.3. Variants đi kèm (repo)

| Variant | Theme |
| :--- | :--- |
| `default` | `babydra-default` |
| `blue` | `babydra-blue` |
| `purple` | `babydra-purple` |
| `green` | `babydra-green` |
| `rose` | `babydra-rose` |

### 3.4. Kiểm tra

```bash
cargo test -p babydra-core variant
```

---

## 4. Merge thứ tự (thắng từ phải sang trái)

```text
system defaults < configs/ seed < theme package < variant < ~/.babydra/ (user)
```

---

## 5. Quy tắc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Mỗi theme/variant = 1 thư mục riêng, không đụng file của người khác |
| DO | Field mới trong tokens phải có `#[serde(default)]` để file cũ vẫn load |
| DO | Test `babydra-theme` + `babydra-core variant` xanh trước khi gửi PR |
| DO NOT | Không hardcode màu/font trong code app — đi qua theme package |
| DO NOT | Không sửa theme/variant của người khác — mỗi người 1 thư mục |

Xem thêm: [design/theming](../design/theming.md), [design/tokens](../design/tokens.md).
