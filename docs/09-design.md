# 09 — Ngôn ngữ thiết kế

## Phạm vi

Trang này quy định cách tổ chức giao diện BabyDra ở cấp token, surface, typography, spacing, motion, state và accessibility. CSS cấu trúc nằm trong `babydra-ui-kit`; màu và giá trị theme nằm trong theme package.

## Phân lớp style

```text
libs/babydra-ui-kit/src/styles/shared/
├── panel/                   Panel, tray, clock, taskbar, workspaces
├── control_center/          Settings và power controls
├── island/                  Dynamic Island
├── launcher/                Launcher
├── explore/                 File manager
├── apps/                    Style riêng của app dùng chung
└── shared/                  Button, dialog, sidebar, scrollbar

themes/<theme-id>/
├── tokens.json              Token dark/light
├── fonts.json               Font family và fallback
└── css/
    ├── dark.css             Color layer dark
    ├── light.css            Color layer light
    └── theme.css            Override nạp sau cùng
```

CSS trong `styles/shared` được `include_str!` vào `babydra-ui-kit`, vì vậy nó định nghĩa layout và cấu trúc. Theme CSS được đọc lúc runtime. Không đặt màu theme vào shared CSS.

## Thứ tự nạp CSS

```text
shared structural CSS
  → dark.css hoặc light.css
  → theme.css
```

Nếu theme có `base`, `babydra-theme` resolve base package trước, merge tokens, nối CSS base trước rồi mới tới CSS của theme con. Theme con có quyền ghi đè giá trị của theme cha.

## Surface và elevation

Surface nên có phân cấp rõ ràng bằng nền, border, shadow và blur khi cần. Dùng hai cấp:

| Cấp | Ví dụ | Đặc điểm |
| :--- | :--- | :--- |
| Nền chính | Panel, sidebar, card | Ổn định, ít shadow, phục vụ đọc nội dung. |
| Bề mặt nổi | Popover, modal, tooltip | Tách khỏi nền chính bằng shadow, border và contrast. |

Không lồng nhiều surface nổi nếu không có lý do tương tác. Radius tham chiếu: pill `9999px`, modal `20px`, card/panel `16–24px`, control nhỏ `10–12px`.

## Token màu

| Token | Dark | Light | Dùng cho |
| :--- | :--- | :--- | :--- |
| `accent` | `#3b82f6` | `#3b82f6` | Primary action, active, progress. |
| `accent-pressed` | `#2563eb` | `#2563eb` | Pressed state. |
| `surface` | `rgba(14,14,18,0.96)` | `rgba(255,255,255,0.98)` | Bề mặt chính. |
| `border` | `rgba(255,255,255,0.14)` | `rgba(0,0,0,0.08)` | Viền control/surface. |
| `text-primary` | `rgba(255,255,255,0.95)` | `rgba(28,28,30,0.95)` | Tiêu đề và nội dung chính. |
| `text-secondary` | `rgba(255,255,255,0.50)` | `rgba(28,28,30,0.50)` | Mô tả và metadata. |
| `hover-bg` | `rgba(255,255,255,0.08)` | `rgba(0,0,0,0.05)` | Hover. |
| `separator` | `rgba(255,255,255,0.10)` | `rgba(0,0,0,0.06)` | Phân cách nội dung. |

Accent không phải màu trang trí. Nó biểu thị hành động chính hoặc state đang được chọn. Error, warning và success chỉ thêm khi state cần phân biệt; dùng semantic token thay vì giá trị riêng của component.

## Typography

Theme package cung cấp font family và fallback trong `fonts.json`. Component dùng hierarchy sau:

| Cấp | Size/weight tham chiếu | Sử dụng |
| :--- | :--- | :--- |
| Heading | `14–16px`, `700` | Tiêu đề card, dialog, vùng nội dung. |
| Label | `13–14px`, `500–600` | Tên setting, button, menu item. |
| Body | `12–13px`, `400` | Mô tả, nội dung phụ. |
| Metadata | `10–12px`, `400–700` | Timestamp, badge, trạng thái phụ. |

Không dùng font-size lớn để bù cho hierarchy kém. Tăng weight hoặc contrast khi cần nhấn mạnh; giữ line-height đủ cho text nhiều dòng.

## Spacing và kích thước control

| Nhóm | Giá trị tham chiếu | Quan hệ |
| :--- | :--- | :--- |
| Micro | `4–6px` | Icon với label, text với badge. |
| Standard | `8–12px` | Item trong cùng group. |
| Section | `16–24px` | Hai group khác nhau. |
| Control switch | `46×24px` | Kích thước `CustomSwitch`. |
| Slider track | `6px` | Track rounded của `CustomSlider`. |

Một component nên dùng ít giá trị spacing và lấy từ layout chung. Không đặt margin rải rác trong application nếu component chung đã xử lý khoảng cách.

## Motion

| Chuyển trạng thái | Duration tham chiếu | Dùng cho |
| :--- | :--- | :--- |
| Hover/active | `160–200ms` | Button, switch, màu nền. |
| Enter | `200ms` | Popover, reveal, nội dung mới. |
| Exit | `150ms` | Đóng popover hoặc ẩn trạng thái. |
| Island expand | `350ms` | Capsule mở rộng. |
| Island collapse | `500ms` | Capsule thu gọn. |
| Collapsible card | `250ms` | `GtkRevealer` mở/đóng. |

Animation phải mô tả quan hệ nguyên nhân-kết quả. Không dùng bounce, parallax hoặc scale toàn layout khi hover. Khi widget bị dispose hoặc state đổi giữa animation, phải hủy hoặc bỏ qua callback cũ để tránh cập nhật widget đã mất.

## State và accessibility

Mỗi interactive control phải kiểm tra:

- default: có thể đọc và nhận biết chức năng;
- hover: feedback nhẹ, không thay đổi kích thước layout;
- focus: dấu hiệu nhìn thấy được và dùng được bằng bàn phím;
- pressed/active: accent hoặc contrast rõ hơn hover;
- disabled: không tương tác nhưng vẫn đọc được;
- error/loading/empty: text đi qua i18n và không chỉ dựa vào màu.

Icon-only button phải có tooltip hoặc accessible label. Dialog phải đặt focus vào control chính, có nút đóng/hủy và không khóa toàn bộ session ngoài phạm vi cần thiết.

## Checklist khi sửa UI

1. Xác định style thuộc shared CSS hay theme CSS.
2. Kiểm tra dark và light.
3. Kiểm tra state keyboard, hover, focus, disabled.
4. Kiểm tra text dài và font fallback.
5. Dùng component chung trước khi thêm CSS hoặc widget mới.
6. Chạy app hoặc test phù hợp trên branch nguồn.
