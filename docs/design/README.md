# Hướng dẫn Thiết kế BabyDra

**Phiên bản:** 1.2.0
**Cập nhật lần cuối:** 2026-08-14

---

Tài liệu này mô tả phong cách và hướng làm thiết kế giao diện của hệ thống BabyDra. Đọc `tokens.md` trước để nắm giá trị cụ thể, sau đó đọc các tài liệu phong cách theo thứ tự dưới đây.

---

## Phong cách Thị giác

| Tài liệu | Nội dung |
| :--- | :--- |
| [visual-language.md](./visual-language.md) | Ngôn ngữ thị giác Glassmorphism — cảm giác và tư duy đằng sau |
| [surfaces.md](./surfaces.md) | Cách mọi bề mặt UI được xây dựng — cấu trúc và lớp nền |
| [color.md](./color.md) | Triết lý màu sắc — tại sao chọn từng màu, cách dùng đúng |
| [typography.md](./typography.md) | Phông chữ, phân cấp, và lý do đằng sau |

## Tương tác và Chuyển động

| Tài liệu | Nội dung |
| :--- | :--- |
| [states.md](./states.md) | Trạng thái tương tác — hover, active, disabled: tư duy và hướng làm |
| [motion.md](./motion.md) | Triết lý chuyển động — khi nào animate, khi nào không, tại sao |

## Bố cục và Hệ thống

| Tài liệu | Nội dung |
| :--- | :--- |
| [spacing.md](./spacing.md) | Không gian và khoảng cách — nguyên tắc và cách áp dụng |
| [theming.md](./theming.md) | Dark/Light theming — cách tiếp cận dual-theme |

## Components Giao diện

Mỗi component trong `libs/babydra-utils/src/components/` có một tài liệu riêng quy định API, CSS classes và quy tắc sử dụng:

| Tài liệu | Nội dung |
| :--- | :--- |
| [components/buttons.md](./components/buttons.md) | Nút bấm: Primary Action, Secondary Pill, Icon Button, Tile |
| [components/badge.md](./components/badge.md) | Badge trạng thái & icon badge tròn kính mờ |
| [components/card.md](./components/card.md) | Card kính mờ, switch card, danh sách cuộn |
| [components/switch.md](./components/switch.md) | CustomSwitch (Cairo, 160ms) & ToggleRow On/Off |
| [components/slider.md](./components/slider.md) | CustomSlider: range, step, tick marks, nhãn phần trăm |
| [components/modal.md](./components/modal.md) | Dialog: password, wifi (info/password/config), vpn (config/log) |
| [components/popovers.md](./components/popovers.md) | Popover chuẩn & Hover Popover (status card) |
| [components/navbar.md](./components/navbar.md) | Navigation row cho Sidebar (icon badge + label) |
| [components/list_group.md](./components/list_group.md) | List row chuẩn & helper dọn danh sách |
| [components/placeholder.md](./components/placeholder.md) | Placeholder state: Disabled / Loading / Empty |
| [components/progress.md](./components/progress.md) | Progress bar & disk progress |
| [components/spinners.md](./components/spinners.md) | Spinner & loading box |
| [components/tooltips.md](./components/tooltips.md) | Helper tooltip thống nhất |
| [components/close_button.md](./components/close_button.md) | Nút đóng icon / icon + nhãn |
| [components/alerts.md](./components/alerts.md) | Thông báo chiếm chỗ (placeholder message) |
| [components/wifi.md](./components/wifi.md) | Icon cường độ tín hiệu Wi-Fi (SVG 0–4 vạch) |

## Tham chiếu

| Tài liệu | Nội dung |
| :--- | :--- |
| [tokens.md](./tokens.md) | Bảng tra cứu giá trị — màu, font-size, radius, spacing, shadow |
