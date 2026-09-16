# 09 — Ngôn ngữ thiết kế

## Phạm vi

Đây là quy tắc visual cho toàn bộ UI BabyDra: surface, màu, typography, spacing, motion và state. Quy tắc áp dụng cho component mới và thay đổi component hiện có.

## Nguyên tắc

1. Bề mặt có phân cấp rõ ràng nhưng không tạo quá nhiều lớp nổi.
2. Accent dùng để chỉ hành động hoặc trạng thái đang hoạt động.
3. Dark và light phải có cùng cấu trúc, radius và hierarchy.
4. Animation giải thích một thay đổi trạng thái; không dùng animation để trang trí.
5. Một token được định nghĩa một lần và dùng lại ở các component.

## Surface và elevation

Một surface có thể gồm nền, blur, border và shadow. Có hai cấp chính:

| Cấp | Ví dụ | Mục đích |
| :--- | :--- | :--- |
| Cấp 1 | Panel, sidebar, card | Bề mặt chính của ứng dụng. |
| Cấp 2 | Dialog, popover, tooltip | Bề mặt tạm thời nằm trên cấp 1. |

Không tạo cấp elevation mới nếu chỉ cần tăng contrast hoặc border. Bo góc tham chiếu: pill `9999px`, dialog khoảng `20px`, card/panel khoảng `16–24px`.

## Màu

| Token | Giá trị dark | Giá trị light | Sử dụng |
| :--- | :--- | :--- | :--- |
| `accent` | `#3b82f6` | `#3b82f6` | Primary action, active state, progress. |
| `accent-pressed` | `#2563eb` | `#2563eb` | Pressed state. |
| `surface` | `rgba(14,14,18,0.96)` | `rgba(255,255,255,0.98)` | Nền chính. |
| `border` | `rgba(255,255,255,0.14)` | `rgba(0,0,0,0.08)` | Viền. |
| `text-primary` | `rgba(255,255,255,0.95)` | `rgba(28,28,30,0.95)` | Nội dung chính. |
| `text-secondary` | `rgba(255,255,255,0.50)` | `rgba(28,28,30,0.50)` | Mô tả và metadata. |
| `hover-bg` | `rgba(255,255,255,0.08)` | `rgba(0,0,0,0.05)` | Hover. |

Không thêm màu tùy ý vào một component. Nếu trạng thái mới có ý nghĩa dùng chung, bổ sung semantic token cho cả dark và light.

## Typography

Ưu tiên font chính được khai báo trong theme package. Phân cấp bằng weight, size và opacity:

| Cấp | Kích thước tham chiếu | Sử dụng |
| :--- | :--- | :--- |
| Heading | `14–16px`, weight `700` | Tiêu đề vùng hoặc dialog. |
| Label | `13–14px`, weight `500–600` | Nhãn nút và item. |
| Body | `12–13px`, weight `400` | Nội dung và mô tả. |
| Metadata | `10–12px`, weight `400–700` | Badge, timestamp, trạng thái phụ. |

Không dùng chữ in hoa cho đoạn dài. Text nhiều dòng cần line-height đủ để đọc trên màn hình HiDPI.

## Spacing

| Nhóm | Giá trị tham chiếu | Quan hệ |
| :--- | :--- | :--- |
| Micro | `4–6px` | Icon và label trong cùng control. |
| Standard | `8–12px` | Các item cùng nhóm. |
| Section | `16–24px` | Hai nhóm chức năng khác nhau. |

Một component không nên dùng quá nhiều giá trị spacing độc lập. Ưu tiên token và component layout của ui-kit.

## Motion

| Loại | Duration | Sử dụng |
| :--- | :--- | :--- |
| State transition | khoảng `200ms` | Hover, active, đổi màu. |
| Enter | khoảng `200ms` | Popover hoặc nội dung xuất hiện. |
| Exit | khoảng `150ms` | Đóng popover hoặc ẩn status. |
| Panel/Island transition | khoảng `400ms` | Mở rộng hoặc thu gọn surface lớn. |

Animation phải có thể dừng hoặc bỏ qua mà không làm mất chức năng. Không dùng bounce, parallax hoặc transform để thay thế phản hồi trạng thái.

## States và accessibility

- Hover: thay đổi background hoặc border nhẹ, không scale toàn control.
- Focus: phải có dấu hiệu nhìn thấy được, không chỉ dựa vào màu rất nhạt.
- Active: dùng accent hoặc contrast rõ hơn hover.
- Disabled: giảm opacity và chặn thao tác; không làm mất hoàn toàn khả năng đọc.
- Icon-only button phải có tooltip hoặc accessible label.

CSS layout đặt trong `libs/babydra-ui-kit/src/styles/shared/`; CSS màu đặt trong theme package. Mọi thay đổi màu phải kiểm tra cả dark và light.
