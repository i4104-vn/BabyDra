# Design Tokens — Bảng tra cứu giá trị

Đây là bảng tra cứu tham chiếu nhanh. Mọi giá trị CSS trong dự án phải lấy từ bảng này. Không tự đặt giá trị ngoài hệ thống.

---

## Màu nền và bề mặt

| Token | Dark | Light | Dùng cho |
| :--- | :--- | :--- | :--- |
| `surface` | `rgba(14, 14, 18, 0.96)` | `rgba(255, 255, 255, 0.98)` | Nền mọi bề mặt nổi |
| `blur` | `-gtk-background-blur: 24` | `-gtk-background-blur: 24` | Đặt kèm surface |
| `border` | `rgba(255, 255, 255, 0.14)` | `rgba(0, 0, 0, 0.08)` | Viền mọi bề mặt |
| `border-top-bevel` | `rgba(255, 255, 255, 0.28)` | `rgba(0, 0, 0, 0.06)` | Chỉ viền cạnh trên |
| `text-primary` | `rgba(255, 255, 255, 0.95)` | `rgba(28, 28, 30, 0.95)` | Tiêu đề, nhãn chính |
| `text-secondary` | `rgba(255, 255, 255, 0.50)` | `rgba(28, 28, 30, 0.50)` | Mô tả phụ, placeholder |
| `hover-bg` | `rgba(255, 255, 255, 0.08)` | `rgba(0, 0, 0, 0.05)` | Nền khi hover |
| `separator` | `rgba(255, 255, 255, 0.10)` | `rgba(0, 0, 0, 0.06)` | Đường phân cách |
| `shadow` | `0 10px 30px rgba(0,0,0,0.35)` | `0 10px 30px rgba(0,0,0,0.08)` | Bóng đổ bề mặt nổi |

---

## Màu điểm nhấn và trạng thái

| Token | Giá trị | Dùng cho |
| :--- | :--- | :--- |
| `accent` | `#3b82f6` | Viền active, nút primary, fill tiến trình |
| `accent-pressed` | `#2563eb` | Nền khi nhấn giữ |
| `success` | `#10b981` | Credit meter, trạng thái hoàn thành |
| `rainbow-gradient` | `conic-gradient(#3b82f6, #f472b6, #fbbf24, #3b82f6)` | Avatar ring |

---

## Badge

Badge trong code là **status badge** (`create_status_badge`) — màu được quyết định bởi class token, không phải bảng màu tĩnh:

| Biến thể | Class | Dark | Light | Dùng cho |
| :--- | :--- | :--- | :--- | :--- |
| Thành công | `.success-text` | `#4ade80` | `#16a34a` | Trạng thái hoàn thành, đã kết nối |
| Thông thường | `.settings-desc` | `rgba(255,255,255,0.55)` | `rgba(28,28,30,0.55)` | Mô tả phụ, trạng thái mặc định |

> [!NOTE]
> Chủ quyền: `styles/dark|light/apps/settings.css`. Icon badge dùng class `blue-icon-badge`/`blue-icon-badge-sm` (44px/34px), màu kính mờ theo layer tương ứng.

---

## Typography

| Cấp bậc | font-size | font-weight | Dùng cho |
| :--- | :--- | :--- | :--- |
| Header | `14px–15px` | `700` | Tiêu đề, tên người dùng |
| Body / Label | `13px–14px` | `500–600` | Nhãn menu, nút, chip |
| Subtext | `12px` | `400` | Mô tả phụ, tooltip |
| Badge | `10px–11px` | `800` | Nhãn viết hoa ngắn |

---

## Border Radius

| Token | Giá trị | Dùng cho |
| :--- | :--- | :--- |
| `radius-pill` | `9999px` | Nút, badge, chip, nav pill |
| `radius-xl` | `24px` | Khung ảnh lớn, input shell |
| `radius-lg` | `20px` | Dropdown, dialog |
| `radius-md` | `16px` | Card vừa, ảnh tham chiếu |
| `radius-sm` | `10px–12px` | Dòng menu hover |
| `radius-circle` | `50%` | Avatar, icon button tròn |

---

## Spacing

| Token | Giá trị | Dùng cho |
| :--- | :--- | :--- |
| `space-micro` | `4px–6px` | Giữa icon và text cùng dòng |
| `space-standard` | `8px–12px` | Padding trong dòng menu, gap giữa phần tử |
| `space-section` | `16px–20px` | Padding container, giữa các nhóm |

---

## Animation

| Loại | Duration | Easing | Dùng cho |
| :--- | :--- | :--- | :--- |
| State transition | `200ms` | `ease` | Hover, active, màu sắc |
| Enter animation | `200ms` | `ease-out` | Dropdown, popover xuất hiện |
| Exit animation | `150ms` | `ease-in` | Dropdown, popover biến mất |
| Genie / Slide | `400ms–450ms` | custom | Cửa sổ đóng/mở |
| Skeleton pulse | `1.2s` | `ease-in-out infinite` | Loading state |
| Button spinner | `0.8s` | `linear infinite` | Nút đang xử lý |

---

## Schema `tokens.json` (theme packages)

Từ `refactor/constructor` (Phase 3), mỗi theme package trong `themes/<theme-id>/`
có một `tokens.json` — schema dưới đây là **hợp đồng** với engine
`babydra-theme` (`ThemeTokens` → `DarkLightTokens` → `RadiusTokens`).
Mọi field đều có `#[serde(default)]`, nên theme có thể chỉ khai báo phần muốn
override so với `base`.

```jsonc
// themes/<theme-id>/tokens.json
{
  "name": "babydra-default",   // bắt buộc: id theme, khớp tên thư mục
  "base": null,                // kế thừa theme khác; null = không kế thừa

  "dark": {                    // lớp màu dark (bắt buộc cặp dark + light)
    "surface": "rgba(14, 14, 18, 0.96)",   // nền bề mặt nổi
    "border":  "rgba(255, 255, 255, 0.14)", // viền bề mặt
    "accent":  "#3b82f6",                   // điểm nhấn (nút primary, active)
    "font":    "Segoe UI Variable Static Text",
    "radius":  { "pill": 9999, "lg": 20, "md": 16, "sm": 10 }
  },

  "light": {
    "surface": "rgba(255, 255, 255, 0.98)",
    "border":  "rgba(0, 0, 0, 0.08)",
    "accent":  "#3b82f6",
    "font":    "Segoe UI Variable Static Text",
    "radius":  { "pill": 9999, "lg": 20, "md": 16, "sm": 10 }
  }
}
```

### Quy tắc

- **Màu** nhận CSS color string (`#hex`, `rgba(...)`); **radius** nhận số `px` (u32).
- **Kế thừa**: field `base: "<theme-id>"` cho phép theme con chỉ ghi phần khác
  (vd `babydra-blue` kế thừa default, đổi `accent` + `radius`). Engine tự phát
  hiện cycle kế thừa.
- **Theme mới**: copy thư mục `themes/babydra-default/` → đổi `name` + giá trị.
  Hướng dẫn từng bước: `docs/05-themes-variants.md`.
- `theme.css` và `fonts.json` đi kèm là lớp màu nạp lên core CSS và bảng font
  families — xem `docs/design/theming.md` mục Theme packages.
