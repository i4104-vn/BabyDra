# Chương 05: Themes & Variants — Hướng dẫn mở rộng

**Phiên bản:** 1.0.0
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

### 2.3. `theme.css` — lớp màu theme

```css
/* the-my-theme — nạp lên core CSS (styles/shared/) */
.my-theme-accent { color: #8b5cf6; }
```

### 2.4. `fonts.json`

```json
{
  "My Font": ["Segoe UI", "sans-serif"]
}
```

### 2.5. Kiểm tra theme

```bash
cargo test -p babydra-theme        # engine hoạt động
# Hoặc thêm integration test trong tests/theme/ nếu cần
```

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
cargo test -p babydra-common variant
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
| DO | Test `babydra-theme` + `babydra-common variant` xanh trước khi gửi PR |
| DO NOT | Không hardcode màu/font trong code app — đi qua theme package |
| DO NOT | Không sửa theme/variant của người khác — mỗi người 1 thư mục |

Xem thêm: `WORKFLOW.md` (mô hình branch variant), `docs/design/theming.md`,
`docs/design/tokens.md` (schema mẫu tokens.json).
