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

| Biến thể | background | color |
| :--- | :--- | :--- |
| Hồng | `#fce7f3` | `#be185d` |
| Xanh lá | `#dcfce7` | `#15803d` |
| Xanh dương | `#dbeafe` | `#1e40af` |

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
