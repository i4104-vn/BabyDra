# Chương 05: Themes & Variants — Hướng dẫn mở rộng

**Phiên bản:** 1.1.0
**Cập nhật lần cuối:** 2026-08-17
**Phạm vi:** Cách tạo theme package mới, cách tạo variant mới — dành cho người
dùng thứ 3, không cần sửa code core.

---

## 1. Khái niệm

| Khái niệm | Là gì | Nằm ở đâu |
| :--- | :--- | :--- |
| **Theme package** | Giao diện: tokens + lớp CSS màu + fonts | `themes/<theme-id>/` |
| **Variant** | Một phiên bản hoàn chỉnh: theme nào + app nào + keybind nào | `variants/<name>/` |

Nguyên tắc: **người tạo theme/variant không cần sửa 1 dòng code core** — mọi thứ
đi qua file cấu hình.

---

## 2. Tạo theme mới

### 2.1. Tạo thư mục

```bash
mkdir themes/my-theme
```

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

### 2.3. Lớp màu: `dark.css` + `light.css`

Mỗi theme package sở hữu **lớp màu** của riêng nó (nạp sau CSS cấu trúc
`styles/shared/`):

```css
/* dark.css — màu dark-mode (nếu kế thừa base, có thể bỏ trống) */
.panel { background: rgba(14, 14, 18, 0.96); }
```

```css
/* light.css — màu light-mode */
.panel { background: rgba(255, 255, 255, 0.98); }
```

> [!TIP]
> Cách nhanh nhất để tạo theme mới: copy `themes/babydra-default/`, sửa màu
> trong `dark.css`/`light.css` và `tokens.json`, đổi `name`. Không cần động
> tới code.

### 2.4. `theme.css` — lớp override (tùy chọn, nạp cuối)

```css
/* theme.css — nạp SAU dark/light, thích hợp để override accent điểm nhấn */
button.baby-fab { background-color: #8b5cf6; }
```

Ví dụ thật: `themes/babydra-blue/theme.css` kế thừa `babydra-default` rồi
override accent `#3b82f6` → `#38bdf8` mà không ship lại toàn bộ dark/light.

### 2.5. `fonts.json`

```json
{
  "My Font": ["Segoe UI", "sans-serif"]
}
```

### 2.6. Kiểm tra theme

```bash
cargo test -p babydra-theme        # engine hoạt động
# Hoặc thêm integration test trong tests/theme/ nếu cần
```

### 2.7. Áp dụng theme

1. Deploy theme vào `~/.babydra/themes/<id>/` (hoặc `/usr/share/babydra/themes/`).
2. Đổi config — thêm/sửa trong `~/.babydra/babydra.conf`:

```toml
[theme]
selection = { id = "my-theme", dark = false }   # dark = null → theo hệ thống
```

3. Khởi động lại app (panel, settings, explore...). Installer cũng tự ghi
   `theme.selection.id` theo variant đã chọn ở bước 6.

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

### 3.3. Kiểm tra

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

Xem thêm: `WORKFLOW.md` (mô hình branch variant), `docs/design/theming.md`,
`docs/design/tokens.md` (schema mẫu tokens.json).
