# Luồng hoạt động — `babydra-theme`

**Phạm vi:** Cách theme engine đọc và resolve theme package, kế thừa `base`, và được `babydra-ui-kit` tiêu thụ.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Vai trò](#1-vai-trò)
- [2. Luồng resolve theme](#2-luồng-resolve-theme)
- [3. Thứ tự tìm thư mục themes](#3-thứ-tự-tìm-thư-mục-themes)
- [4. Kế thừa `base`](#4-kế-thừa-base)
- [5. Kiểm tra lỗi](#5-kiểm-tra-lỗi)

---

## 1. Vai trò

`babydra-theme` là crate **thuần logic** (không GTK, không parse CSS runtime): đọc theme package từ đĩa và resolve thành `ThemeValue` (tokens + CSS layers + fonts). `babydra-ui-kit` gọi nó trong `init_theme()`.

```text
babydra-ui-kit::resolve_theme_layers()
  └─ babydra_theme::resolve_theme(id)
       ├─ load_package(id)      ── đọc themes/<id>/ từ đĩa
       ├─ resolve_recursive()   ── kế thừa base (nếu có)
       └─ ThemeValue { dark, light, dark_css, light_css, css_layer, fonts }
```

---

## 2. Luồng resolve theme

```text
resolve_theme(id)
  1. resolve_recursive(id, visited)
       a. nếu id đã trong visited → ThemeError::Cycle (phát hiện vòng kế thừa)
       b. load_package(id)
       c. nếu package.base có → resolve_recursive(base) trước, rồi merge
  2. trả ThemeValue
```

`load_package(id)` — đọc từ đĩa:

```text
load_package(id)
  → dir = themes_root()/id
  → tokens.json   → ThemeTokens (serde)
  → css/dark.css  → dark_css   (fallback: flat <id>/dark.css)
  → css/light.css → light_css  (fallback: flat)
  → css/theme.css → css        (fallback: flat)
  → fonts.json    → fonts (HashMap family → fallbacks, lỗi → rỗng)
```

> [!NOTE]
> Layout cũ (flat `<id>/dark.css` không có thư mục `css/`) vẫn được đọc làm fallback
> cho theme đã deploy trước đó — fallback theo từng file.

---

## 3. Thứ tự tìm thư mục themes

`themes_root()` — cái đầu tiên tồn tại thắng:

```text
1. $BABYDRA_THEMES_DIR        (env override — test / deploy linh hoạt)
2. ~/.babydra/themes          (theme người dùng cài)
3. /usr/share/babydra/themes  (theme hệ thống)
4. <workspace>/themes         (repo dev — CARGO_MANIFEST_DIR 2 cấp lên)
```

---

## 4. Kế thừa `base`

Khi `tokens.json` có `"base": "<id>"`:

| Thành phần | Cách merge |
| :--- | :--- |
| `tokens.dark` / `light` | base trước, child `merge()` đè lên |
| `dark_css` / `light_css` / `css` | nối chuỗi: base layer trước, child layer sau (child rule thắng) |
| `fonts` | base là nền, child entry ghi đè |

Ví dụ `babydra-blue` kế thừa `babydra-default`:

```text
themes/babydra-default/  (tokens đầy đủ + dark.css + light.css)
        ▲ base
themes/babydra-blue/     (chỉ tokens.json đổi accent + css/theme.css override)
```

---

## 5. Kiểm tra lỗi

```bash
cargo test -p babydra-theme        # engine hoạt động
```

| Lỗi | Nguyên nhân | Fallback của ui-kit |
| :--- | :--- | :--- |
| `NotFound` | Không có thư mục `themes/<id>/` | thử `babydra-default`, lỗi tiếp → CSS cấu trúc thuần |
| `Invalid` | `tokens.json` sai format | như trên |
| `Cycle` | Kế thừa vòng (a→b→a) | như trên |

> [!NOTE]
> Hướng dẫn tạo theme/variant: [themes](../themes/index.md). API: [apis](../apis/index.md).
